/// Phase 6 — 2D animation: swinging limb
///
/// A capsule with one end fixed (the pivot/joint) and the other tracing an arc.
/// This is the core building block of every limb in the running figure.
///
/// Joint model:
///   pivot  = fixed point (the hip or shoulder)
///   angle  = sin(t * 2π * speed) * max_swing   — pure f(t), no stored state
///   end    = pivot + (sin(angle), -cos(angle)) * length
///   shape  = capsule(p, pivot, end, radius)
///
/// The swing axis is vertical at rest (angle=0 → straight down).
/// Positive angle swings right, negative angle swings left.
///
/// This example shows a single leg swinging from a hip at (0.0, 0.1).
///
/// T3 Expected (deferred — run manually):
///   A capsule (thick line) swinging left and right from a fixed top endpoint.
///   Clean arc at the foot end. Gray edge. Dark background.
///
/// Run: `cargo run --example ex_anim_limb -p form-raster`
use form_raster::{rasterize_sequence, RasterSettings, to_bmp};
use form_sdf::capsule_2d;
use glam::Vec2;

const PIVOT:     Vec2  = Vec2::new(0.0, 0.15);
const LENGTH:    f32   = 0.45;
const RADIUS:    f32   = 0.06;
const MAX_SWING: f32   = 0.55; // radians (~31°)
const SPEED:     f32   = 1.0;  // cycles per second

fn limb_end(t: f32) -> Vec2 {
    let angle = (t * std::f32::consts::TAU * SPEED).sin() * MAX_SWING;
    PIVOT + Vec2::new(angle.sin(), -angle.cos()) * LENGTH
}

fn sdf(p: Vec2, t: f32) -> f32 {
    capsule_2d(p, PIVOT, limb_end(t), RADIUS)
}

fn main() {
    let width       = 256u32;
    let height      = 256u32;
    let frame_count = 32usize;
    let dt          = 1.0 / frame_count as f32;
    let settings    = RasterSettings { scale: 1.5, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_anim_limb — {frame_count} frames ({width}x{height})...");

    let frames = rasterize_sequence(sdf, width, height, settings, frame_count, dt);

    for (i, pixels) in frames.iter().enumerate() {
        let path = format!("ex_anim_limb_{i:03}.bmp");
        std::fs::write(&path, to_bmp(pixels, width, height)).unwrap();
    }

    println!(
        "Saved {frame_count} frames: ex_anim_limb_000.bmp .. ex_anim_limb_{:03}.bmp",
        frame_count - 1
    );
    println!("T3 Expected: capsule swinging left and right from fixed top pivot");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, rasterize_sequence, RasterSettings};
    use form_sdf::capsule_2d;
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 1.5, edge_width: 0.02, flat_bg: false };

    const PIVOT:     Vec2 = Vec2::new(0.0, 0.15);
    const LENGTH:    f32  = 0.45;
    const RADIUS:    f32  = 0.06;
    const MAX_SWING: f32  = 0.55;
    const SPEED:     f32  = 1.0;

    fn limb_end(t: f32) -> Vec2 {
        let angle = (t * std::f32::consts::TAU * SPEED).sin() * MAX_SWING;
        PIVOT + Vec2::new(angle.sin(), -angle.cos()) * LENGTH
    }

    fn sdf(p: Vec2, t: f32) -> f32 {
        capsule_2d(p, PIVOT, limb_end(t), RADIUS)
    }

    // T1 — joint mechanics

    #[test]
    fn pivot_always_inside() {
        // The pivot is always on the limb regardless of angle
        assert!(sdf(PIVOT, 0.0)  < 0.0);
        assert!(sdf(PIVOT, 0.25) < 0.0);
        assert!(sdf(PIVOT, 0.5)  < 0.0);
    }

    #[test]
    fn at_t0_limb_hangs_straight_down() {
        // t=0: sin(0)=0, angle=0, end = pivot + (0, -1)*length = (0, 0.15-0.45) = (0, -0.30)
        let end = limb_end(0.0);
        assert!((end.x).abs() < 1e-5, "at t=0 limb should hang straight, x={}", end.x);
        assert!((end.y - (PIVOT.y - LENGTH)).abs() < 1e-5, "at t=0 foot should be at pivot.y - length");
    }

    #[test]
    fn at_t0_foot_inside() {
        let foot = limb_end(0.0);
        assert!(sdf(foot, 0.0) < 0.0, "foot endpoint should be inside capsule");
    }

    #[test]
    fn limb_swings_right_at_quarter_period() {
        // t=0.25: sin(π/2)=1, angle=MAX_SWING → end is to the right of pivot
        let end = limb_end(0.25);
        assert!(end.x > 0.0, "limb should swing right at t=0.25, x={}", end.x);
    }

    #[test]
    fn limb_swings_left_at_three_quarter_period() {
        // t=0.75: sin(3π/2)=-1, angle=-MAX_SWING → end is to the left of pivot
        let end = limb_end(0.75);
        assert!(end.x < 0.0, "limb should swing left at t=0.75, x={}", end.x);
    }

    #[test]
    fn swing_is_symmetric() {
        // |end.x| at t=0.25 should equal |end.x| at t=0.75
        let right = limb_end(0.25).x;
        let left  = limb_end(0.75).x;
        assert!((right + left).abs() < 1e-5, "swing should be symmetric: right={right}, left={left}");
    }

    #[test]
    fn full_period_returns_to_start() {
        let a = sdf(Vec2::new(0.1, 0.0), 0.0);
        let b = sdf(Vec2::new(0.1, 0.0), 1.0);
        assert!((a - b).abs() < 1e-5, "t=0 and t=1 should match: {a} vs {b}");
    }

    #[test]
    fn far_corner_always_outside() {
        assert!(sdf(Vec2::new(2.0, 2.0), 0.0)  > 0.0);
        assert!(sdf(Vec2::new(2.0, 2.0), 0.25) > 0.0);
    }

    #[test]
    fn sdf_is_pure() {
        let a = sdf(Vec2::new(0.05, 0.0), 0.3);
        let b = sdf(Vec2::new(0.05, 0.0), 0.3);
        assert_eq!(a, b);
    }

    // T2 — render

    #[test]
    fn pivot_pixel_is_white() {
        // scale=1.5, pivot=(0,0.15) → col=16, row = (0.5 - 0.15/1.5)*32 ≈ 13
        let px = rasterize(|p| sdf(p, 0.0), 32, 32, S);
        let col = 16usize;
        let row = 13usize;
        let brightness = px[(row * 32 + col) * 3];
        assert!(brightness > 200, "pivot should be white, got {brightness}");
    }

    #[test]
    fn corner_is_dark() {
        let px = rasterize(|p| sdf(p, 0.0), 16, 16, S);
        assert!(px[0] < 100);
    }

    #[test]
    fn frames_differ() {
        // Quarter-period frames should differ (limb moved)
        let frames = rasterize_sequence(sdf, 16, 16, S, 4, 0.25);
        let differ = frames[0].iter().zip(frames[1].iter()).any(|(a, b)| a != b);
        assert!(differ, "frames at t=0 and t=0.25 should differ");
    }
}
