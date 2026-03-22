/// Phase 6 — 2D animation: bouncing circle
///
/// Proves the animation model: `sdf(p, t)` is a pure function of space AND time.
/// No stored state. Same `(p, t)` always returns the same distance.
///
/// The circle centre follows a sinusoidal path:
///   centre = (0.0, sin(t * 2π / period) * 0.3)
///
/// At t=0:        centre at (0,  0.0)  — middle
/// At t=period/4: centre at (0,  0.3)  — top
/// At t=period/2: centre at (0,  0.0)  — middle
/// At t=3period/4: centre at (0, -0.3) — bottom
///
/// Output: 24 BMP frames, `ex_anim_bounce_NNN.bmp`, one full cycle.
///
/// T3 Expected (T3 deferred — run manually):
///   Circle moves smoothly up then down, radius unchanged, clean arc throughout.
///
/// Run: `cargo run --example ex_anim_bounce -p form-raster`
use form_raster::{rasterize_sequence, RasterSettings, to_bmp};
use glam::Vec2;

fn sdf(p: Vec2, t: f32) -> f32 {
    let period = 1.0_f32;
    let centre = Vec2::new(0.0, (t * std::f32::consts::TAU / period).sin() * 0.3);
    (p - centre).length() - 0.25
}

fn main() {
    let width       = 256u32;
    let height      = 256u32;
    let frame_count = 24usize;
    let dt          = 1.0 / frame_count as f32; // one full period across 24 frames
    let settings    = RasterSettings { scale: 2.0, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_anim_bounce — {frame_count} frames ({width}x{height})...");

    let frames = rasterize_sequence(sdf, width, height, settings, frame_count, dt);

    for (i, pixels) in frames.iter().enumerate() {
        let path = format!("ex_anim_bounce_{i:03}.bmp");
        std::fs::write(&path, to_bmp(pixels, width, height)).unwrap();
    }

    println!("Saved {frame_count} frames: ex_anim_bounce_000.bmp .. ex_anim_bounce_{:03}.bmp", frame_count - 1);
    println!("T3 Expected: circle bounces smoothly up and down over one full cycle");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, rasterize_sequence, RasterSettings};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 2.0, edge_width: 0.02, flat_bg: false };

    fn sdf(p: Vec2, t: f32) -> f32 {
        let period = 1.0_f32;
        let centre = Vec2::new(0.0, (t * std::f32::consts::TAU / period).sin() * 0.3);
        (p - centre).length() - 0.25
    }

    // T1 — time-parameterized SDF

    #[test]
    fn centre_at_t0_inside() {
        // At t=0: centre is at origin, so (0,0) is inside the circle
        assert!(sdf(Vec2::ZERO, 0.0) < 0.0);
    }

    #[test]
    fn far_outside_at_any_t() {
        // (2,2) is always outside regardless of t (circle radius 0.25, travel ±0.3)
        assert!(sdf(Vec2::new(2.0, 2.0), 0.0) > 0.0);
        assert!(sdf(Vec2::new(2.0, 2.0), 0.25) > 0.0);
        assert!(sdf(Vec2::new(2.0, 2.0), 0.5) > 0.0);
    }

    #[test]
    fn circle_moves_upward_at_quarter_period() {
        // At t=0.25 (quarter period): sin(π/2)=1, centre at (0, 0.3)
        // A point at (0, 0.3) should be inside
        assert!(sdf(Vec2::new(0.0, 0.3), 0.25) < 0.0);
        // That same point is outside at t=0 (centre at origin, radius 0.25)
        assert!(sdf(Vec2::new(0.0, 0.3), 0.0) > 0.0);
    }

    #[test]
    fn circle_moves_downward_at_three_quarter_period() {
        // At t=0.75: sin(3π/2)=-1, centre at (0, -0.3)
        // A point at (0, -0.3) should be inside
        assert!(sdf(Vec2::new(0.0, -0.3), 0.75) < 0.0);
        // Outside at t=0
        assert!(sdf(Vec2::new(0.0, -0.3), 0.0) > 0.0);
    }

    #[test]
    fn full_period_returns_to_start() {
        // t=0 and t=1.0 are the same phase — SDF should match exactly
        let a = sdf(Vec2::new(0.1, 0.2), 0.0);
        let b = sdf(Vec2::new(0.1, 0.2), 1.0);
        assert!((a - b).abs() < 1e-5, "t=0 and t=1 should be identical, got {a} vs {b}");
    }

    #[test]
    fn sdf_is_pure() {
        let a = sdf(Vec2::new(0.1, 0.2), 0.3);
        let b = sdf(Vec2::new(0.1, 0.2), 0.3);
        assert_eq!(a, b);
    }

    #[test]
    fn radius_preserved_at_surface() {
        // At t=0: centre at origin. Point at (0.25, 0) should be on the surface.
        let d = sdf(Vec2::new(0.25, 0.0), 0.0);
        assert!(d.abs() < 1e-5, "radius should be exactly 0.25 at surface, got {d}");
    }

    // T2 — render

    #[test]
    fn frame0_centre_is_white() {
        // At t=0 the circle is centred — centre pixel should be white
        let px = rasterize(|p| sdf(p, 0.0), 16, 16, S);
        assert_eq!(px[(8 * 16 + 8) * 3], 255, "centre should be white at t=0");
    }

    #[test]
    fn corner_always_dark() {
        let px = rasterize(|p| sdf(p, 0.0), 16, 16, S);
        assert!(px[0] < 100, "corner should always be dark");
    }

    #[test]
    fn sequence_has_correct_frame_count() {
        let frames = rasterize_sequence(sdf, 8, 8, S, 24, 1.0 / 24.0);
        assert_eq!(frames.len(), 24);
    }

    #[test]
    fn frames_differ_across_cycle() {
        // First and seventh frame (t=0 vs t≈0.29) should differ — circle has moved
        let frames = rasterize_sequence(sdf, 16, 16, S, 24, 1.0 / 24.0);
        let differ = frames[0].iter().zip(frames[6].iter()).any(|(a, b)| a != b);
        assert!(differ, "frames at different phases should produce different images");
    }

    #[test]
    fn first_and_last_frame_match() {
        // 24 frames over period=1.0, dt=1/24 → last frame t=23/24 (not quite 1.0)
        // Instead test t=0 vs t=1.0 directly via rasterize
        let f0 = rasterize(|p| sdf(p, 0.0), 16, 16, S);
        let f1 = rasterize(|p| sdf(p, 1.0), 16, 16, S);
        assert_eq!(f0, f1, "t=0 and t=1.0 (full period) should produce identical frames");
    }
}
