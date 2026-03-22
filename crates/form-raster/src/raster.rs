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
/// * `flat_bg`    — if true, outside pixels are a flat dark colour (no distance gradient).
///                  Use this for domain-warp examples where the warped SDF no longer
///                  has a uniform gradient, which would otherwise produce visible
///                  background colour banding.
#[derive(Debug, Clone, Copy)]
pub struct RasterSettings {
    pub scale:      f32,
    pub edge_width: f32,
    pub flat_bg:    bool,
}

impl Default for RasterSettings {
    fn default() -> Self {
        RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false }
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
        pixel_color(sdf(p), settings.edge_width, settings.flat_bg)
    }).collect()
}

/// Map an SDF distance value to an RGB colour.
///
/// Inside < 0 → white. Edge ≈ 0 → gray. Outside > 0 → dark, gets darker with distance.
fn pixel_color(d: f32, edge_width: f32, flat_bg: bool) -> [u8; 3] {
    if d < -edge_width {
        [255, 255, 255]
    } else if d.abs() <= edge_width {
        [160, 160, 160]
    } else if flat_bg {
        [6, 6, 20]
    } else {
        let t = (d / 1.5).clamp(0.0, 1.0);
        let v = (20.0 + (1.0 - t) * 40.0) as u8;
        [v / 3, v / 3, v]
    }
}

/// Rasterize a time-parameterized 2D SDF into a sequence of RGB pixel buffers.
///
/// Each frame evaluates the SDF at `t = frame_index * dt`.
/// The SDF signature is `Fn(Vec2, f32) -> f32` — space point and time.
///
/// # Arguments
/// * `sdf`         — time-parameterized SDF: `(Vec2, t) → f32`
/// * `width`       — output image width in pixels
/// * `height`      — output image height in pixels
/// * `settings`    — scale, edge width, background style
/// * `frame_count` — number of frames to render
/// * `dt`          — time step between frames in seconds
///
/// # Returns
/// Vec of RGB pixel buffers, one per frame, each `width * height * 3` bytes.
///
/// # Example
/// ```rust
/// use glam::Vec2;
/// use form_raster::{rasterize_sequence, RasterSettings};
///
/// // Pulsing circle: radius oscillates with time
/// let sdf = |p: Vec2, t: f32| p.length() - (0.3 + (t * 2.0).sin() * 0.05);
/// let frames = rasterize_sequence(sdf, 8, 8, RasterSettings::default(), 4, 0.25);
/// assert_eq!(frames.len(), 4);
/// assert_eq!(frames[0].len(), 8 * 8 * 3);
/// ```
pub fn rasterize_sequence<F>(
    sdf: F,
    width: u32,
    height: u32,
    settings: RasterSettings,
    frame_count: usize,
    dt: f32,
) -> Vec<Vec<u8>>
where
    F: Fn(Vec2, f32) -> f32,
{
    (0..frame_count)
        .map(|i| {
            let t = i as f32 * dt;
            rasterize(|p| sdf(p, t), width, height, settings)
        })
        .collect()
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

    const SETTINGS: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    // T1 — pixel color function

    #[test]
    fn inside_is_white() {
        assert_eq!(pixel_color(-0.5, 0.02, false), [255, 255, 255]);
    }

    #[test]
    fn edge_is_gray() {
        let c = pixel_color(0.0, 0.02, false);
        assert_eq!(c, [160, 160, 160]);
    }

    #[test]
    fn outside_is_dark() {
        let c = pixel_color(1.0, 0.02, false);
        assert!(c[0] < 100 && c[2] < 100, "outside should be dark, got {c:?}");
    }

    #[test]
    fn outside_gets_darker_with_distance() {
        let near = pixel_color(0.1, 0.02, false);
        let far  = pixel_color(1.0, 0.02, false);
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

    // T2 — rasterize_sequence

    #[test]
    fn sequence_correct_frame_count() {
        let sdf = |p: Vec2, _t: f32| p.length() - 1.0;
        let frames = rasterize_sequence(sdf, 8, 8, SETTINGS, 6, 0.1);
        assert_eq!(frames.len(), 6);
    }

    #[test]
    fn sequence_each_frame_correct_length() {
        let sdf = |p: Vec2, _t: f32| p.length() - 1.0;
        let frames = rasterize_sequence(sdf, 8, 8, SETTINGS, 3, 0.1);
        for frame in &frames {
            assert_eq!(frame.len(), 8 * 8 * 3);
        }
    }

    #[test]
    fn sequence_frame0_matches_t0_rasterize() {
        // Frame 0 (t=0) should be identical to rasterize() with t=0 baked in
        let sdf = |p: Vec2, _t: f32| p.length() - 1.0;
        let frames  = rasterize_sequence(sdf, 8, 8, SETTINGS, 2, 0.5);
        let direct  = rasterize(|p| p.length() - 1.0, 8, 8, SETTINGS);
        assert_eq!(frames[0], direct);
    }

    #[test]
    fn sequence_moving_shape_frames_differ() {
        // A bouncing circle: centre moves over time, so frames should differ
        // at pixels near the boundary.
        let sdf = |p: Vec2, t: f32| {
            let centre = Vec2::new(0.0, (t * std::f32::consts::PI).sin() * 0.3);
            (p - centre).length() - 0.3
        };
        let frames = rasterize_sequence(sdf, 16, 16, SETTINGS, 2, 0.5);
        // With t=0: centre at (0,0). With t=0.5: centre at (0, sin(π/2)*0.3)=(0,0.3).
        // At least some pixels should differ.
        let differ = frames[0].iter().zip(frames[1].iter()).any(|(a, b)| a != b);
        assert!(differ, "frames should differ as shape moves");
    }

    #[test]
    fn sequence_static_shape_frames_identical() {
        // A static SDF that ignores t should produce identical frames
        let sdf = |p: Vec2, _t: f32| p.length() - 0.5;
        let frames = rasterize_sequence(sdf, 8, 8, SETTINGS, 4, 0.25);
        assert!(frames.windows(2).all(|w| w[0] == w[1]), "static SDF should produce identical frames");
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
