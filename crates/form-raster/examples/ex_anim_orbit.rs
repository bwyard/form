/// Phase 6 — 2D animation: orbiting circle
///
/// Proves that position is a pure function of time.
/// A small circle orbits a fixed centre point via:
///
///   centre(t) = (cos(t * 2π) * radius, sin(t * 2π) * radius)
///
/// No stored state. Same t always gives the same position.
/// This is the foundation of every joint in the running figure —
/// a point moving on a known arc, driven purely by time.
///
/// Output: 32 BMP frames, one full orbit.
///
/// T3 Expected (deferred — run manually):
///   Small white circle orbiting smoothly counterclockwise around the centre.
///   Clean circular path, constant radius, dark background.
///
/// Run: `cargo run --example ex_anim_orbit -p form-raster`
use form_raster::{rasterize_sequence, RasterSettings, to_bmp};
use glam::Vec2;

fn sdf(p: Vec2, t: f32) -> f32 {
    let orbit_radius = 0.35_f32;
    let angle        = t * std::f32::consts::TAU;
    let centre       = Vec2::new(angle.cos() * orbit_radius, angle.sin() * orbit_radius);
    (p - centre).length() - 0.12
}

fn main() {
    let width       = 256u32;
    let height      = 256u32;
    let frame_count = 32usize;
    let dt          = 1.0 / frame_count as f32;
    let settings    = RasterSettings { scale: 1.5, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_anim_orbit — {frame_count} frames ({width}x{height})...");

    let frames = rasterize_sequence(sdf, width, height, settings, frame_count, dt);

    for (i, pixels) in frames.iter().enumerate() {
        let path = format!("ex_anim_orbit_{i:03}.bmp");
        std::fs::write(&path, to_bmp(pixels, width, height)).unwrap();
    }

    println!(
        "Saved {frame_count} frames: ex_anim_orbit_000.bmp .. ex_anim_orbit_{:03}.bmp",
        frame_count - 1
    );
    println!("T3 Expected: circle orbiting counterclockwise on clean circular path");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, rasterize_sequence, RasterSettings};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 1.5, edge_width: 0.02, flat_bg: false };

    fn sdf(p: Vec2, t: f32) -> f32 {
        let orbit_radius = 0.35_f32;
        let angle        = t * std::f32::consts::TAU;
        let centre       = Vec2::new(angle.cos() * orbit_radius, angle.sin() * orbit_radius);
        (p - centre).length() - 0.12
    }

    // T1 — position as f(t)

    #[test]
    fn at_t0_circle_is_on_right() {
        // t=0: angle=0, centre=(0.35, 0.0)
        assert!(sdf(Vec2::new(0.35, 0.0), 0.0) < 0.0, "circle centre should be inside at t=0");
    }

    #[test]
    fn at_t_quarter_circle_is_on_top() {
        // t=0.25: angle=π/2, centre=(0.0, 0.35)
        assert!(sdf(Vec2::new(0.0, 0.35), 0.25) < 0.0, "circle should be at top at t=0.25");
    }

    #[test]
    fn at_t_half_circle_is_on_left() {
        // t=0.5: angle=π, centre=(-0.35, 0.0)
        assert!(sdf(Vec2::new(-0.35, 0.0), 0.5) < 0.0, "circle should be at left at t=0.5");
    }

    #[test]
    fn at_t_three_quarter_circle_is_on_bottom() {
        // t=0.75: angle=3π/2, centre=(0.0, -0.35)
        assert!(sdf(Vec2::new(0.0, -0.35), 0.75) < 0.0, "circle should be at bottom at t=0.75");
    }

    #[test]
    fn full_period_returns_to_start() {
        // t=0 and t=1.0 are the same phase
        let a = sdf(Vec2::new(0.2, 0.1), 0.0);
        let b = sdf(Vec2::new(0.2, 0.1), 1.0);
        assert!((a - b).abs() < 1e-5, "t=0 and t=1 should be identical, got {a} vs {b}");
    }

    #[test]
    fn origin_is_always_outside() {
        // Origin is at the centre of the orbit — the circle never passes through it
        // (orbit radius 0.35 > circle radius 0.12, so closest approach = 0.23 > 0)
        assert!(sdf(Vec2::ZERO, 0.0)  > 0.0);
        assert!(sdf(Vec2::ZERO, 0.25) > 0.0);
        assert!(sdf(Vec2::ZERO, 0.5)  > 0.0);
    }

    #[test]
    fn far_corner_always_outside() {
        assert!(sdf(Vec2::new(2.0, 2.0), 0.0)  > 0.0);
        assert!(sdf(Vec2::new(2.0, 2.0), 0.33) > 0.0);
    }

    #[test]
    fn sdf_is_pure() {
        let a = sdf(Vec2::new(0.2, 0.1), 0.3);
        let b = sdf(Vec2::new(0.2, 0.1), 0.3);
        assert_eq!(a, b);
    }

    // T2 — render

    #[test]
    fn t0_right_side_has_white_pixel() {
        // At t=0 circle is on the right — pixels around (0.35, 0.0) should be white
        let px = rasterize(|p| sdf(p, 0.0), 32, 32, S);
        // scale=1.5 → world x=0.35 maps to col = (0.35/1.5 + 0.5) * 32 ≈ 22
        let col = 22usize;
        let row = 16usize;
        let brightness = px[(row * 32 + col) * 3];
        assert!(brightness > 200, "right side should be white at t=0, got {brightness}");
    }

    #[test]
    fn t0_left_side_is_dark() {
        // At t=0 circle is on the right — left side should be dark (outside)
        let px = rasterize(|p| sdf(p, 0.0), 32, 32, S);
        let col = 9usize;
        let row = 16usize;
        let brightness = px[(row * 32 + col) * 3];
        assert!(brightness < 100, "left side should be dark at t=0, got {brightness}");
    }

    #[test]
    fn sequence_frames_differ() {
        let frames = rasterize_sequence(sdf, 16, 16, S, 4, 0.25);
        // Quarter-phase frames should look different (circle at right vs top vs left vs bottom)
        let differ_0_1 = frames[0].iter().zip(frames[1].iter()).any(|(a, b)| a != b);
        let differ_0_2 = frames[0].iter().zip(frames[2].iter()).any(|(a, b)| a != b);
        assert!(differ_0_1, "quarter-phase frames should differ");
        assert!(differ_0_2, "half-phase frames should differ");
    }

    #[test]
    fn sequence_length_correct() {
        let frames = rasterize_sequence(sdf, 8, 8, S, 32, 1.0 / 32.0);
        assert_eq!(frames.len(), 32);
    }
}
