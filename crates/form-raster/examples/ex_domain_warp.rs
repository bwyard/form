/// Phase 4 — 2D corruption: domain warp
///
/// Demonstrates the difference between CSG and domain warps:
///
///   CSG  — combine two shapes:    shape = union(a, b)
///   Warp — bend space before SDF: shape = sdf(warp(p))
///
/// This example applies warp::twist to the square-with-two-spikes scene.
/// The warp is applied to `p` before any SDF is evaluated. Nothing about
/// the primitives changes — only the input point is transformed.
///
/// Composition reads as: twist(p) → evaluate scene at warped point.
/// Each primitive is still a pure function. The warp is a pure function.
/// The whole thing is a pure function.
///
/// T3 Expected: the square-with-spikes shape, visibly rotated/sheared
///              in a smooth curve — top twisted to the right, bottom to left,
///              the two spikes still visible but skewed, gray edge, dark background.
///
/// Run: `cargo run --example ex_domain_warp -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::{box_2d, isosceles_triangle, smooth_union, warp};
use glam::Vec2;

fn sdf(p: Vec2) -> f32 {
    // Warp first — bend space before touching any primitive
    let q = warp::twist(p, 0.8);

    // Evaluate the scene at the warped point — same composition as ex_square_with_spike
    let body   = box_2d(q, Vec2::ZERO, Vec2::new(0.5, 0.5));
    let spike0 = isosceles_triangle(q, Vec2::new(-0.22, 0.5), 0.1, 0.3);
    let spike1 = isosceles_triangle(q, Vec2::new( 0.22, 0.5), 0.1, 0.3);
    let spikes = smooth_union(spike0, spike1, 0.05);
    smooth_union(body, spikes, 0.08)
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    println!("Rendering ex_domain_warp ({width}x{height})...");
    println!("T3 Expected: square-with-spikes, smoothly twisted in space, spikes skewed, gray edge, dark background");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_domain_warp.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_domain_warp.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::{box_2d, isosceles_triangle, smooth_union, warp};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    fn sdf(p: Vec2) -> f32 {
        let q      = warp::twist(p, 0.8);
        let body   = box_2d(q, Vec2::ZERO, Vec2::new(0.5, 0.5));
        let spike0 = isosceles_triangle(q, Vec2::new(-0.22, 0.5), 0.1, 0.3);
        let spike1 = isosceles_triangle(q, Vec2::new( 0.22, 0.5), 0.1, 0.3);
        let spikes = smooth_union(spike0, spike1, 0.05);
        smooth_union(body, spikes, 0.08)
    }

    // T1 — warp correctness

    #[test]
    fn twist_zero_y_leaves_centre_unchanged() {
        // At y=0 the twist angle is 0 — the twist has no effect at equator
        // so the centre of the box (0,0) should still be inside
        assert!(sdf(Vec2::ZERO) < 0.0);
    }

    #[test]
    fn far_outside_is_positive() {
        assert!(sdf(Vec2::new(3.0, 0.0)) > 0.0);
        assert!(sdf(Vec2::new(0.0, 3.0)) > 0.0);
    }

    #[test]
    fn twist_moves_material_off_axis() {
        // With twist=0.8, a point that was inside the unwarped box above the equator
        // should now require evaluating at its twisted position.
        // The box centre-top at (0, 0.3) is inside the untwisted box.
        // Twist at y=0.3: angle = 0.3*0.8 = 0.24 rad. The point is rotated,
        // but (0,0.3) on-axis stays on-axis at y=0.3 (twist rotates x, not y).
        // So (0, 0.3) should still be inside.
        assert!(sdf(Vec2::new(0.0, 0.3)) < 0.0);
    }

    #[test]
    fn warp_is_pure() {
        // Same input always gives same output
        let a = sdf(Vec2::new(0.5, 0.5));
        let b = sdf(Vec2::new(0.5, 0.5));
        assert_eq!(a, b);
    }

    // T2 — render

    #[test]
    fn centre_is_white() {
        let px = rasterize(sdf, 16, 16, S);
        assert_eq!(px[(8 * 16 + 8) * 3], 255, "centre should be white");
    }

    #[test]
    fn corner_is_dark() {
        let px = rasterize(sdf, 16, 16, S);
        assert!(px[0] < 100, "corner should be dark");
    }

    #[test]
    fn warped_scene_differs_from_unwarped() {
        // With twist applied, the scene is not left-right symmetric at y≠0.
        // Compare pixels at symmetric x positions above the equator — they should differ.
        let px = rasterize(sdf, 32, 32, S);
        // row 10 is above centre (y > 0), columns 8 and 24 are symmetric in x
        let left  = px[(10 * 32 + 8)  * 3];
        let right = px[(10 * 32 + 24) * 3];
        assert_ne!(left, right, "twist should break left-right symmetry above equator");
    }
}
