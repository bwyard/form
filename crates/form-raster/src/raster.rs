//! 2D SDF rasterizer — pure pixel evaluation, no marching.
//!
//! Assembly rule: no STORE, no JUMP.
//! `pixel[x, y] = color(sdf(world(x, y)))`
//! Same SDF + same settings = same image, always.

use glam::Vec2;

/// Settings for the 2D rasterizer.
///
/// # Fields
/// * `scale`      — world units visible from edge to edge (e.g. 3.0 = ±1.5 units)
/// * `edge_width` — SDF distance threshold for the edge highlight line
#[derive(Debug, Clone, Copy)]
pub struct RasterSettings {
    pub scale:      f32,
    pub edge_width: f32,
}

impl Default for RasterSettings {
    fn default() -> Self {
        RasterSettings { scale: 3.0, edge_width: 0.02 }
    }
}

/// Rasterize a 2D SDF to an RGB pixel buffer.
///
/// Maps each pixel to a world-space point, evaluates the SDF, and assigns a colour:
/// - Inside  (`d < -edge_width`): white `[255, 255, 255]`
/// - Edge    (`d.abs() <= edge_width`): gray `[160, 160, 160]`
/// - Outside (`d > edge_width`): dark blue-grey, darkening with distance
///
/// # Arguments
/// * `sdf`      — signed distance function: `Vec2 → f32`
/// * `width`    — output image width in pixels
/// * `height`   — output image height in pixels
/// * `settings` — scale and edge width
///
/// # Returns
/// Flat RGB byte buffer, row-major, `width * height * 3` bytes.
///
/// # Example
/// ```rust
/// use glam::Vec2;
/// use form_raster::{rasterize, RasterSettings};
///
/// // Unit circle
/// let sdf = |p: Vec2| p.length() - 1.0;
/// let pixels = rasterize(sdf, 16, 16, RasterSettings::default());
/// assert_eq!(pixels.len(), 16 * 16 * 3);
/// // Centre pixel is inside — should be white
/// let centre = (8 * 16 + 8) * 3;
/// assert_eq!(pixels[centre], 255);
/// ```
pub fn rasterize<F>(sdf: F, width: u32, height: u32, settings: RasterSettings) -> Vec<u8>
where
    F: Fn(Vec2) -> f32,
{
    let half_scale = settings.scale * 0.5;

    (0..height * width).flat_map(|i| {
        let row = i / width;
        let col = i % width;
        let wx = (col as f32 + 0.5) / width  as f32 * settings.scale - half_scale;
        let wy = (row as f32 + 0.5) / height as f32 * settings.scale - half_scale;
        let p  = Vec2::new(wx, -wy); // flip y so +y is up in world space
        pixel_color(sdf(p), settings.edge_width)
    }).collect()
}

/// Map an SDF distance value to an RGB colour.
///
/// Inside < 0 → white. Edge ≈ 0 → gray. Outside > 0 → dark, gets darker with distance.
fn pixel_color(d: f32, edge_width: f32) -> [u8; 3] {
    if d < -edge_width {
        // Inside — white
        [255, 255, 255]
    } else if d.abs() <= edge_width {
        // Edge line — gray
        [160, 160, 160]
    } else {
        // Outside — dark blue-grey, gets darker further out
        let t = (d / 1.5).clamp(0.0, 1.0);
        let v = (20.0 + (1.0 - t) * 40.0) as u8;
        [v / 3, v / 3, v]
    }
}

/// Write an RGB pixel buffer to an uncompressed 24-bit BMP file.
///
/// # Arguments
/// * `pixels` — flat RGB buffer (width * height * 3 bytes)
/// * `width`  — image width in pixels
/// * `height` — image height in pixels
///
/// # Returns
/// BMP file as a byte vector, ready to write to disk.
pub fn to_bmp(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let row_size   = (width * 3 + 3) & !3;
    let pixel_data = row_size * height;
    let file_size  = 54 + pixel_data;

    let mut buf = Vec::with_capacity(file_size as usize);

    // BMP file header (14 bytes)
    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(&[0u8; 4]); // reserved
    buf.extend_from_slice(&54u32.to_le_bytes()); // pixel data offset

    // DIB header — BITMAPINFOHEADER (40 bytes)
    buf.extend_from_slice(&40u32.to_le_bytes());
    buf.extend_from_slice(&width.to_le_bytes());
    buf.extend_from_slice(&height.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());  // planes
    buf.extend_from_slice(&24u16.to_le_bytes()); // bits per pixel
    buf.extend_from_slice(&[0u8; 24]);           // compression + remaining fields

    // Pixel data — bottom-to-top, BGR
    for row in (0..height).rev() {
        for col in 0..width {
            let i = ((row * width + col) * 3) as usize;
            buf.push(pixels[i + 2]); // B
            buf.push(pixels[i + 1]); // G
            buf.push(pixels[i    ]); // R
        }
        // Row padding
        buf.extend(std::iter::repeat_n(0u8, (row_size - width * 3) as usize));
    }

    buf
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    fn unit_circle(p: Vec2) -> f32 { p.length() - 1.0 }

    const SETTINGS: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02 };

    // T1 — pixel color function

    #[test]
    fn inside_is_white() {
        assert_eq!(pixel_color(-0.5, 0.02), [255, 255, 255]);
    }

    #[test]
    fn edge_is_gray() {
        let c = pixel_color(0.0, 0.02);
        assert_eq!(c, [160, 160, 160]);
    }

    #[test]
    fn outside_is_dark() {
        let c = pixel_color(1.0, 0.02);
        assert!(c[0] < 100 && c[2] < 100, "outside should be dark, got {c:?}");
    }

    #[test]
    fn outside_gets_darker_with_distance() {
        let near = pixel_color(0.1, 0.02);
        let far  = pixel_color(1.0, 0.02);
        // Further = darker = lower blue channel
        assert!(far[2] <= near[2], "further should be darker: near={near:?} far={far:?}");
    }

    // T2 — rasterize output

    #[test]
    fn output_length_correct() {
        let pixels = rasterize(unit_circle, 16, 16, SETTINGS);
        assert_eq!(pixels.len(), 16 * 16 * 3);
    }

    #[test]
    fn deterministic() {
        let a = rasterize(unit_circle, 16, 16, SETTINGS);
        let b = rasterize(unit_circle, 16, 16, SETTINGS);
        assert_eq!(a, b);
    }

    #[test]
    fn centre_pixel_is_white() {
        // Centre of image maps to (0,0) in world — inside unit circle
        let pixels = rasterize(unit_circle, 16, 16, SETTINGS);
        let centre = (8 * 16 + 8) * 3;
        assert_eq!(
            pixels[centre], 255,
            "centre pixel should be white (inside circle)"
        );
    }

    #[test]
    fn corner_pixel_is_dark() {
        // Corner maps to (±1.5, ±1.5) — well outside unit circle
        let pixels = rasterize(unit_circle, 16, 16, SETTINGS);
        let corner_r = pixels[0];
        assert!(corner_r < 100, "corner should be dark (outside circle), got R={corner_r}");
    }

    #[test]
    fn symmetric_circle_has_symmetric_pixels() {
        let pixels = rasterize(unit_circle, 16, 16, SETTINGS);
        let left_sum: u32 = (0..16u32)
            .flat_map(|row| (0..8u32).map(move |col| ((row * 16 + col) * 3) as usize))
            .map(|i| pixels[i] as u32)
            .sum();
        let right_sum: u32 = (0..16u32)
            .flat_map(|row| (8..16u32).map(move |col| ((row * 16 + col) * 3) as usize))
            .map(|i| pixels[i] as u32)
            .sum();
        let diff = (left_sum as i64 - right_sum as i64).unsigned_abs();
        assert!(diff < 50, "symmetric SDF should produce symmetric pixels, diff={diff}");
    }

    // T2 — BMP output

    #[test]
    fn bmp_starts_with_magic() {
        let pixels = vec![255u8; 8 * 8 * 3];
        let bmp = to_bmp(&pixels, 8, 8);
        assert_eq!(&bmp[0..2], b"BM");
    }

    #[test]
    fn bmp_correct_size() {
        let pixels = vec![0u8; 8 * 8 * 3];
        let bmp = to_bmp(&pixels, 8, 8);
        let row_size = (8 * 3 + 3) & !3;
        assert_eq!(bmp.len() as u32, 54 + row_size * 8);
    }
}
