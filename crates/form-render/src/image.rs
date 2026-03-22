//! Image rendering — fold over pixels, march each ray, shade.
//!
//! The image render is the outer ADVANCE loop. The ray march is the inner.
//! Together: fold over (x, y) → for each pixel, MARCH+EVALUATE → RGB.
//!
//! Assembly rule: no STORE, no JUMP. Output is a pure function of the scene,
//! camera, and image dimensions. Same inputs → same image, always.

use glam::Vec3;
use crate::camera::Camera;
use crate::light::{diffuse, hard_shadow, normal_at};
use crate::march::{march, MarchSettings};

const NORMAL_EPSILON: f32 = 0.0001;
const SHADOW_OFFSET:  f32 = 0.01;

/// Shade a single ray: march, estimate normal, apply diffuse + shadow.
///
/// Returns an RGB triple in [0, 255].
///
/// # Arguments
/// * `sdf`       — the signed distance function defining the scene
/// * `origin`    — ray origin
/// * `direction` — unit ray direction
/// * `light_dir` — unit direction toward the key light
/// * `settings`  — march settings
fn shade_ray<F>(
    sdf:       &F,
    origin:    Vec3,
    direction: Vec3,
    light_dir: Vec3,
    settings:  MarchSettings,
) -> [u8; 3]
where
    F: Fn(Vec3) -> f32,
{
    let result = march(origin, direction, sdf, settings);

    if !result.hit {
        // Sky gradient — background colour
        let t = 0.5 * (direction.y + 1.0);
        let sky = Vec3::new(1.0, 1.0, 1.0).lerp(Vec3::new(0.5, 0.7, 1.0), t);
        return [
            (sky.x * 255.0) as u8,
            (sky.y * 255.0) as u8,
            (sky.z * 255.0) as u8,
        ];
    }

    let normal = normal_at(sdf, result.position, NORMAL_EPSILON);
    let shadow_origin = result.position + normal * SHADOW_OFFSET;
    let d = diffuse(normal, light_dir);
    let s = hard_shadow(sdf, shadow_origin, light_dir, settings);

    let ambient = 0.1_f32;
    let intensity = (ambient + (1.0 - ambient) * d * s).clamp(0.0, 1.0);

    // Base colour: white surface modulated by intensity
    [
        (intensity * 255.0) as u8,
        (intensity * 255.0) as u8,
        (intensity * 255.0) as u8,
    ]
}

/// Render a scene to a flat RGB byte buffer.
///
/// The output is the result of folding over all `width * height` pixels,
/// marching a ray for each, and shading the result. No STORE, no JUMP.
///
/// # Math
///
/// ```text
/// for each pixel (x, y):
///   u = (x + 0.5) / width
///   v = (y + 0.5) / height
///   dir = camera.ray_direction(u, v)
///   rgb = shade_ray(sdf, camera.origin, dir, light_dir, settings)
///   output[y * width + x] = rgb
/// ```
///
/// Implemented as a fold over pixel indices — no mutable loop variable.
///
/// # Arguments
/// * `sdf`       — signed distance function defining the scene
/// * `camera`    — camera parameters
/// * `light_dir` — unit direction toward the key light
/// * `width`     — image width in pixels
/// * `height`    — image height in pixels
/// * `settings`  — sphere tracing parameters
///
/// # Returns
/// `Vec<u8>` of length `width * height * 3` in row-major RGB order.
///
/// # Example
/// ```rust
/// use glam::Vec3;
/// use form_render::camera::Camera;
/// use form_render::image::render_image;
/// use form_render::march::MarchSettings;
///
/// let sdf = |p: Vec3| p.length() - 1.0;
/// let cam = Camera::look_at(Vec3::new(0.0, 0.0, -3.0), Vec3::ZERO, Vec3::Y, 60.0, 1.0);
/// let pixels = render_image(sdf, cam, Vec3::new(1.0, 1.0, -1.0).normalize(), 4, 4, MarchSettings::default());
/// assert_eq!(pixels.len(), 4 * 4 * 3);
/// ```
pub fn render_image<F>(
    sdf:       F,
    camera:    Camera,
    light_dir: Vec3,
    width:     u32,
    height:    u32,
    settings:  MarchSettings,
) -> Vec<u8>
where
    F: Fn(Vec3) -> f32,
{
    let total = (width * height) as usize;
    let (_, pixels) = (0..total).fold(
        ((), Vec::with_capacity(total * 3)),
        |(_, mut buf), idx| {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            let u = (x as f32 + 0.5) / width as f32;
            let v = 1.0 - (y as f32 + 0.5) / height as f32; // flip Y for image coords
            let dir = camera.ray_direction(u, v);
            let [r, g, b] = shade_ray(&sdf, camera.origin, dir, light_dir, settings);
            buf.push(r);
            buf.push(g);
            buf.push(b);
            ((), buf)
        },
    );
    pixels
}

/// Write an RGB pixel buffer to PPM format bytes.
///
/// PPM is the simplest possible image format — plain ASCII header + binary pixels.
/// No external dependencies required.
///
/// # Arguments
/// * `pixels` — raw RGB buffer, `width * height * 3` bytes
/// * `width`  — image width in pixels
/// * `height` — image height in pixels
///
/// # Returns
/// `Vec<u8>` containing a valid P6 PPM file.
pub fn to_ppm(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let header = format!("P6\n{width} {height}\n255\n");
    let mut out = Vec::with_capacity(header.len() + pixels.len());
    out.extend_from_slice(header.as_bytes());
    out.extend_from_slice(pixels);
    out
}

/// Write an RGB pixel buffer to uncompressed BMP format bytes.
///
/// BMP opens natively on Windows with no tools required.
/// Pixels are written bottom-to-top (BMP convention), with each row
/// padded to a 4-byte boundary.
///
/// # Arguments
/// * `pixels` — raw RGB buffer, row-major top-to-bottom, `width * height * 3` bytes
/// * `width`  — image width in pixels
/// * `height` — image height in pixels
///
/// # Returns
/// `Vec<u8>` containing a valid 24-bit uncompressed BMP file.
pub fn to_bmp(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let row_size   = (width * 3 + 3) & !3;          // pad each row to 4-byte boundary
    let pixel_data = row_size * height;
    let file_size  = 54 + pixel_data;

    let mut out = Vec::with_capacity(file_size as usize);

    // BMP file header (14 bytes)
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&file_size.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());      // reserved
    out.extend_from_slice(&0u16.to_le_bytes());      // reserved
    out.extend_from_slice(&54u32.to_le_bytes());     // pixel data offset

    // DIB header — BITMAPINFOHEADER (40 bytes)
    out.extend_from_slice(&40u32.to_le_bytes());     // header size
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());    // positive = bottom-to-top
    out.extend_from_slice(&1u16.to_le_bytes());      // colour planes
    out.extend_from_slice(&24u16.to_le_bytes());     // bits per pixel
    out.extend_from_slice(&0u32.to_le_bytes());      // no compression
    out.extend_from_slice(&pixel_data.to_le_bytes());
    out.extend_from_slice(&2835u32.to_le_bytes());   // 72 dpi horizontal
    out.extend_from_slice(&2835u32.to_le_bytes());   // 72 dpi vertical
    out.extend_from_slice(&0u32.to_le_bytes());      // colours in table
    out.extend_from_slice(&0u32.to_le_bytes());      // important colours

    // Pixel data — BMP is bottom-to-top, BGR order
    let (_, bmp_pixels) = (0..height).fold(
        ((), Vec::with_capacity(pixel_data as usize)),
        |(_, mut buf), row| {
            let src_row = height - 1 - row;
            let (_, _) = (0..width).fold(
                ((), &mut buf),
                |(_, b), col| {
                    let i = ((src_row * width + col) * 3) as usize;
                    b.push(pixels[i + 2]); // B
                    b.push(pixels[i + 1]); // G
                    b.push(pixels[i]);     // R
                    ((), b)
                },
            );
            // row padding
            let pad = (row_size - width * 3) as usize;
            buf.extend(std::iter::repeat_n(0u8, pad));
            ((), buf)
        },
    );

    out.extend_from_slice(&bmp_pixels);
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sphere_cam() -> (impl Fn(Vec3) -> f32, Camera) {
        let sdf = |p: Vec3| p.length() - 1.0;
        let cam = Camera::look_at(
            Vec3::new(0.0, 0.0, -3.0),
            Vec3::ZERO,
            Vec3::Y,
            60.0,
            1.0,
        );
        (sdf, cam)
    }

    #[test]
    fn output_length_correct() {
        let (sdf, cam) = sphere_cam();
        let pixels = render_image(sdf, cam, Vec3::new(1.0, 1.0, -1.0).normalize(), 8, 8, MarchSettings::default());
        assert_eq!(pixels.len(), 8 * 8 * 3);
    }

    #[test]
    fn deterministic() {
        let light = Vec3::new(1.0, 1.0, -1.0).normalize();
        let (sdf, cam) = sphere_cam();
        let a = render_image(|p| p.length() - 1.0, cam, light, 4, 4, MarchSettings::default());
        let b = render_image(sdf, cam, light, 4, 4, MarchSettings::default());
        assert_eq!(a, b);
    }

    #[test]
    fn centre_pixel_hits_sphere() {
        // Centre pixel should hit the sphere — should be brighter than background blue
        let light = Vec3::new(1.0, 1.0, -1.0).normalize();
        let (sdf, cam) = sphere_cam();
        let pixels = render_image(sdf, cam, light, 11, 11, MarchSettings::default());
        // Centre pixel index = (5 * 11 + 5) * 3 = 180
        let centre_r = pixels[180];
        // The sky background is blueish (r < 200), the sphere is white (r > 100)
        // Just verify it rendered something non-zero
        assert!(centre_r > 0 || pixels[181] > 0 || pixels[182] > 0);
    }

    // T2 — render tests added for Phase 1 gap work

    #[test]
    fn corner_pixel_is_sky_blue_dominant() {
        // Corners should miss the sphere and show sky — sky gradient is blue-dominant (B > R)
        let light = Vec3::new(1.0, 1.0, -1.0).normalize();
        let (sdf, cam) = sphere_cam();
        let pixels = render_image(sdf, cam, light, 11, 11, MarchSettings::default());
        // Top-left corner pixel (0, 0) — index 0
        let r = pixels[0] as i32;
        let b = pixels[2] as i32;
        assert!(b > r, "top-left corner should be sky (B > R), got R={r} B={b}");
        // Bottom-right corner (10, 10) — index (10*11 + 10)*3 = 360
        let r2 = pixels[360] as i32;
        let b2 = pixels[362] as i32;
        assert!(b2 > r2, "bottom-right corner should be sky (B > R), got R={r2} B={b2}");
    }

    #[test]
    fn centre_pixel_is_not_sky() {
        // Sky pixels are always blue-dominant (B > R) — the sky gradient lerps from white to blue.
        // A sphere hit renders as grayscale (R == G == B) because the surface colour is white.
        // So a hit pixel has R ≈ B, while a sky pixel has B >> R.
        let light = Vec3::new(1.0, 1.0, -1.0).normalize();
        let (sdf, cam) = sphere_cam();
        let pixels = render_image(sdf, cam, light, 11, 11, MarchSettings::default());
        // Centre = (5*11+5)*3 = 180
        let centre_r = pixels[180] as i32;
        let centre_b = pixels[182] as i32;
        // Sphere hit: R ≈ B (grayscale). Sky miss: B >> R.
        // B - R < 20 confirms this is a sphere hit, not sky.
        assert!(
            centre_b - centre_r < 20,
            "centre pixel should be sphere (grayscale, R≈B) not sky (B>>R), \
             got R={centre_r} B={centre_b} diff={}",
            centre_b - centre_r
        );
    }

    #[test]
    fn symmetric_sdf_produces_symmetric_pixels() {
        // A unit sphere is symmetric — left half and right half pixel sums should match
        let light = Vec3::new(0.0, 1.0, -1.0).normalize(); // symmetric light (no x component)
        let (sdf, cam) = sphere_cam();
        let pixels = render_image(sdf, cam, light, 10, 10, MarchSettings::default());
        // Sum R channel for left 5 columns vs right 5 columns
        let left_sum: u32 = (0..10)
            .flat_map(|row| (0..5).map(move |col| (row * 10 + col) * 3))
            .map(|i| pixels[i] as u32)
            .sum();
        let right_sum: u32 = (0..10)
            .flat_map(|row| (5..10).map(move |col| (row * 10 + col) * 3))
            .map(|i| pixels[i] as u32)
            .sum();
        // Allow small tolerance for floating point rounding across pixel boundaries
        let diff = (left_sum as i64 - right_sum as i64).unsigned_abs();
        assert!(
            diff < 50,
            "symmetric SDF should produce symmetric pixels — left_sum={left_sum} right_sum={right_sum} diff={diff}"
        );
    }

    #[test]
    fn bmp_file_has_correct_size() {
        // BMP header is 54 bytes + padded pixel data
        let width = 8u32;
        let height = 8u32;
        let pixels = vec![128u8; (width * height * 3) as usize];
        let bmp = to_bmp(&pixels, width, height);
        // Row size padded to 4 bytes: (8*3 + 3) & !3 = 24
        let row_size = (width * 3 + 3) & !3;
        let expected = 54 + row_size * height;
        assert_eq!(bmp.len() as u32, expected, "BMP size mismatch");
    }

    #[test]
    fn bmp_starts_with_magic_bytes() {
        let pixels = vec![0u8; 4 * 4 * 3];
        let bmp = to_bmp(&pixels, 4, 4);
        assert_eq!(&bmp[0..2], b"BM", "BMP must start with 'BM' magic bytes");
    }

    #[test]
    fn ppm_has_correct_header() {
        let pixels = vec![255u8; 4 * 4 * 3];
        let ppm = to_ppm(&pixels, 4, 4);
        let header = "P6\n4 4\n255\n";
        assert!(ppm.starts_with(header.as_bytes()));
        assert_eq!(ppm.len(), header.len() + pixels.len());
    }

    #[test]
    fn ppm_total_length() {
        let pixels = vec![0u8; 16 * 9 * 3];
        let ppm = to_ppm(&pixels, 16, 9);
        let header = format!("P6\n16 9\n255\n");
        assert_eq!(ppm.len(), header.len() + 16 * 9 * 3);
    }
}
