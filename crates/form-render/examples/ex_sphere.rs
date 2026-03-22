/// Phase 2 — Primitive gallery: sphere
///
/// T3 Expected: white sphere filling roughly 60% of frame,
///              blue sky gradient visible at edges,
///              light from upper-right (right side brighter than left).
///
/// Run: `cargo run --example ex_sphere -p form-render`
use form_render::{Camera, MarchSettings, render_image, to_bmp};
use glam::Vec3;

fn main() {
    let width  = 512u32;
    let height = 512u32;

    let sdf = |p: Vec3| p.length() - 1.0;

    let camera = Camera::look_at(
        Vec3::new(0.0, 0.0, -3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        width as f32 / height as f32,
    );

    let light_dir = Vec3::new(1.0, 2.0, -1.0).normalize();

    let settings = MarchSettings {
        max_steps:   128,
        max_dist:    100.0,
        hit_epsilon: 0.001,
    };

    println!("Rendering sphere ({width}x{height})...");
    println!("T3 Expected: white sphere filling ~60% of frame, blue sky at edges, light from upper-right");
    let pixels = render_image(sdf, camera, light_dir, width, height, settings);
    let bmp    = to_bmp(&pixels, width, height);
    std::fs::write("ex_sphere.bmp", &bmp).expect("failed to write bmp");
    println!("Saved ex_sphere.bmp — confirm T3 expected output matches");
}

#[cfg(test)]
mod tests {
    use form_render::{Camera, MarchSettings, render_image};
    use glam::Vec3;

    const W: u32 = 11;
    const H: u32 = 11;
    const EPSILON: f32 = 1e-5;

    fn sphere_sdf(p: Vec3) -> f32 { p.length() - 1.0 }

    fn cam() -> Camera {
        Camera::look_at(Vec3::new(0.0, 0.0, -3.0), Vec3::ZERO, Vec3::Y, 60.0, 1.0)
    }

    fn light() -> Vec3 { Vec3::new(0.0, 1.0, -1.0).normalize() } // symmetric light for symmetry test

    fn render() -> Vec<u8> {
        render_image(sphere_sdf, cam(), light(), W, H, MarchSettings::default())
    }

    // T1 — SDF math tests

    #[test]
    fn inside_origin_is_negative() {
        assert!(sphere_sdf(Vec3::ZERO) < 0.0);
    }

    #[test]
    fn outside_is_positive() {
        assert!(sphere_sdf(Vec3::new(2.0, 0.0, 0.0)) > 0.0);
    }

    #[test]
    fn surface_is_near_zero() {
        assert!(sphere_sdf(Vec3::X).abs() < EPSILON);
        assert!(sphere_sdf(Vec3::Y).abs() < EPSILON);
        assert!(sphere_sdf(Vec3::Z).abs() < EPSILON);
    }

    #[test]
    fn farther_point_has_larger_sdf() {
        let near = sphere_sdf(Vec3::new(1.5, 0.0, 0.0));
        let far  = sphere_sdf(Vec3::new(3.0, 0.0, 0.0));
        assert!(far > near);
    }

    // T2 — render tests

    #[test]
    fn centre_pixel_is_sphere_not_sky() {
        let pixels = render();
        let i = ((H / 2 * W + W / 2) * 3) as usize;
        let r = pixels[i] as i32;
        let b = pixels[i + 2] as i32;
        assert!(b - r < 20, "centre should be sphere (R≈B), got R={r} B={b}");
    }

    #[test]
    fn top_left_corner_is_sky() {
        let pixels = render();
        let r = pixels[0] as i32;
        let b = pixels[2] as i32;
        assert!(b > r, "top-left corner should be sky (B>R), got R={r} B={b}");
    }

    #[test]
    fn top_right_corner_is_sky() {
        let pixels = render();
        let i = ((W - 1) * 3) as usize;
        let r = pixels[i] as i32;
        let b = pixels[i + 2] as i32;
        assert!(b > r, "top-right corner should be sky (B>R), got R={r} B={b}");
    }

    #[test]
    fn symmetric_light_produces_symmetric_image() {
        let pixels = render(); // symmetric light, no x component
        let left_sum: u32 = (0..H)
            .flat_map(|row| (0..W / 2).map(move |col| ((row * W + col) * 3) as usize))
            .map(|i| pixels[i] as u32)
            .sum();
        let right_sum: u32 = (0..H)
            .flat_map(|row| ((W / 2 + 1)..W).map(move |col| ((row * W + col) * 3) as usize))
            .map(|i| pixels[i] as u32)
            .sum();
        let diff = (left_sum as i64 - right_sum as i64).unsigned_abs();
        assert!(diff < 100, "symmetric scene should produce symmetric pixels, diff={diff}");
    }
}
