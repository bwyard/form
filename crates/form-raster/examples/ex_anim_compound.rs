/// Phase 6 — 2D animation: compound animated scene
///
/// Two circles joined with smooth_union, each moving independently.
/// Proves that multi-part scenes work as pure f(p, t) — each piece
/// evaluates at its own position derived from time, then they are combined.
///
/// This is the direct precursor to the figure: a torso + head, or
/// two limbs attached to a body, are the same pattern.
///
///   circle_a: orbits counterclockwise, radius 0.25, speed 1.0
///   circle_b: orbits clockwise, radius 0.20, speed 1.5  (different phase + direction)
///   shape:    smooth_union(a, b, k=0.1)
///
/// When the two circles are close, smooth_union blends them into one shape.
/// When they are far, they appear as two separate blobs.
/// The blend boundary is emergent — never explicitly computed.
///
/// T3 Expected (deferred — run manually):
///   Two white blobs orbiting independently. When they pass near each other
///   they merge into one shape with a smooth neck, then separate again.
///
/// Run: `cargo run --example ex_anim_compound -p form-raster`
use form_raster::{rasterize_sequence, RasterSettings, to_bmp};
use form_sdf::smooth_union;
use glam::Vec2;

fn sdf(p: Vec2, t: f32) -> f32 {
    let tau = std::f32::consts::TAU;

    let centre_a = Vec2::new(
        ( t * tau       ).cos() * 0.25,
        ( t * tau       ).sin() * 0.25,
    );
    let centre_b = Vec2::new(
        (-t * tau * 1.5 + std::f32::consts::PI).cos() * 0.20,
        (-t * tau * 1.5 + std::f32::consts::PI).sin() * 0.20,
    );

    let d_a = (p - centre_a).length() - 0.12;
    let d_b = (p - centre_b).length() - 0.10;
    smooth_union(d_a, d_b, 0.10)
}

fn main() {
    let width       = 256u32;
    let height      = 256u32;
    let frame_count = 48usize;
    let dt          = 1.0 / frame_count as f32;
    let settings    = RasterSettings { scale: 1.2, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_anim_compound — {frame_count} frames ({width}x{height})...");

    let frames = rasterize_sequence(sdf, width, height, settings, frame_count, dt);

    for (i, pixels) in frames.iter().enumerate() {
        let path = format!("ex_anim_compound_{i:03}.bmp");
        std::fs::write(&path, to_bmp(pixels, width, height)).unwrap();
    }

    println!(
        "Saved {frame_count} frames: ex_anim_compound_000.bmp .. ex_anim_compound_{:03}.bmp",
        frame_count - 1
    );
    println!("T3 Expected: two blobs orbiting independently, merging when close, separating when far");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, rasterize_sequence, RasterSettings};
    use form_sdf::smooth_union;
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 1.2, edge_width: 0.02, flat_bg: false };

    fn sdf(p: Vec2, t: f32) -> f32 {
        let tau = std::f32::consts::TAU;
        let centre_a = Vec2::new(( t * tau      ).cos() * 0.25, ( t * tau      ).sin() * 0.25);
        let centre_b = Vec2::new(
            (-t * tau * 1.5 + std::f32::consts::PI).cos() * 0.20,
            (-t * tau * 1.5 + std::f32::consts::PI).sin() * 0.20,
        );
        let d_a = (p - centre_a).length() - 0.12;
        let d_b = (p - centre_b).length() - 0.10;
        smooth_union(d_a, d_b, 0.10)
    }

    // T1 — independent motion

    #[test]
    fn centre_a_at_t0_inside() {
        // t=0: centre_a = (0.25, 0.0)
        assert!(sdf(Vec2::new(0.25, 0.0), 0.0) < 0.0);
    }

    #[test]
    fn centre_b_at_t0_inside() {
        // t=0: centre_b = (-0.20, 0.0) (cos(0)=1 but negated → (-0.20, 0.0))
        assert!(sdf(Vec2::new(-0.20, 0.0), 0.0) < 0.0);
    }

    #[test]
    fn far_corner_always_outside() {
        assert!(sdf(Vec2::new(2.0,  2.0), 0.0)  > 0.0);
        assert!(sdf(Vec2::new(2.0, -2.0), 0.25) > 0.0);
    }

    #[test]
    fn shapes_move_independently() {
        // centre_a at t=0.25: angle=π/2, centre=(0.0, 0.25) — top
        // centre_a at t=0:    centre=(0.25, 0.0)             — right
        // The region around (0.0, 0.25) should be inside at t=0.25 but not t=0
        let at_top_t0   = sdf(Vec2::new(0.0, 0.25), 0.0);
        let at_top_t025 = sdf(Vec2::new(0.0, 0.25), 0.25);
        // Not guaranteed to be outside at t=0 (circle_b might be near there),
        // but the two evaluations should differ.
        assert_ne!(at_top_t0, at_top_t025, "same point should have different SDF at different times");
    }

    #[test]
    fn union_is_at_most_min_of_parts() {
        // smooth_union(a, b) <= min(a, b) + some k-dependent slack
        // At centre_a (inside a, outside b), result should be negative
        assert!(sdf(Vec2::new(0.25, 0.0), 0.0) < 0.0);
        // At centre_b (inside b, outside a), result should also be negative
        assert!(sdf(Vec2::new(-0.20, 0.0), 0.0) < 0.0);
    }

    #[test]
    fn sdf_is_pure() {
        let a = sdf(Vec2::new(0.1, 0.2), 0.3);
        let b = sdf(Vec2::new(0.1, 0.2), 0.3);
        assert_eq!(a, b);
    }

    // T2 — render

    #[test]
    fn t0_right_blob_visible() {
        // circle_a at (0.25, 0.0) — should be white in right half
        let px = rasterize(|p| sdf(p, 0.0), 32, 32, S);
        // scale=1.2, x=0.25 → col = (0.25/1.2 + 0.5) * 32 ≈ 22
        let brightness = px[(16 * 32 + 22) * 3];
        assert!(brightness > 200, "right blob should be white at t=0, got {brightness}");
    }

    #[test]
    fn t0_left_blob_visible() {
        // circle_b at (-0.20, 0.0) — should be white in left half
        let px = rasterize(|p| sdf(p, 0.0), 32, 32, S);
        // x=-0.20 → col = (-0.20/1.2 + 0.5) * 32 ≈ 10
        let brightness = px[(16 * 32 + 10) * 3];
        assert!(brightness > 200, "left blob should be white at t=0, got {brightness}");
    }

    #[test]
    fn frames_differ() {
        let frames = rasterize_sequence(sdf, 16, 16, S, 8, 1.0 / 8.0);
        let differ = frames[0].iter().zip(frames[2].iter()).any(|(a, b)| a != b);
        assert!(differ, "frames at different times should differ");
    }

    #[test]
    fn sequence_length_correct() {
        let frames = rasterize_sequence(sdf, 8, 8, S, 48, 1.0 / 48.0);
        assert_eq!(frames.len(), 48);
    }
}
