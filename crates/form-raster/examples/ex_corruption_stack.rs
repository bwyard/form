/// Phase 4 — 2D corruption: full layered corruption stack
///
/// Demonstrates the complete corruption model in one scene:
///   Layer 1 — CSG: circle + spike merged with smooth_union
///   Layer 2 — Warp: sinusoidal wave applied to the entire scene
///
/// Composition:
///   q     = warp::wave(p, frequency=5.0, amplitude=0.05)
///   body  = circle(q, origin, 0.4)
///   spike = isosceles_triangle(q, (0.0, 0.4), half_width=0.08, height=0.25)
///   shape = smooth_union(body, spike, k=0.06)
///
/// The CSG layer adds the spike to the top of the circle.
/// The warp layer then ripples the entire assembled shape.
/// The spike and the circle boundary both undulate — the wave
/// corrupts the combined form, not just one primitive.
///
/// No SDF output is modified. The corruption is entirely in input-space.
///
/// T3 Expected (T3 deferred — run manually):
///   White circle with a spike at top, entire boundary rippling with ~5 wave
///   cycles. Spike is visibly skewed by the wave. Smooth blend at circle/spike
///   junction. Gray edge. Dark background.
///
/// Run: `cargo run --example ex_corruption_stack -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::{circle, isosceles_triangle, smooth_union, warp};
use glam::Vec2;

fn sdf(p: Vec2) -> f32 {
    let q     = warp::wave(p, 5.0, 0.05);
    let body  = circle(q, Vec2::ZERO, 0.4);
    let spike = isosceles_triangle(q, Vec2::new(0.0, 0.4), 0.08, 0.25);
    smooth_union(body, spike, 0.06)
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    println!("Rendering ex_corruption_stack ({width}x{height})...");
    println!("T3 Expected: circle+spike, entire shape rippled by wave warp, gray edge, dark background");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_corruption_stack.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_corruption_stack.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::{circle, isosceles_triangle, smooth_union, warp};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    fn sdf(p: Vec2) -> f32 {
        let q     = warp::wave(p, 5.0, 0.05);
        let body  = circle(q, Vec2::ZERO, 0.4);
        let spike = isosceles_triangle(q, Vec2::new(0.0, 0.4), 0.08, 0.25);
        smooth_union(body, spike, 0.06)
    }

    // T1 — CSG + warp correctness

    #[test]
    fn centre_inside() {
        assert!(sdf(Vec2::ZERO) < 0.0);
    }

    #[test]
    fn far_outside() {
        assert!(sdf(Vec2::new(3.0, 0.0)) > 0.0);
        assert!(sdf(Vec2::new(0.0, -3.0)) > 0.0);
    }

    #[test]
    fn spike_apex_reachable() {
        // Spike apex at q=(0, 0.65) — wave displacement at x=0 is sin(0)=0 → q=p
        // So unwarped spike apex at (0, 0.4 + 0.25) = (0, 0.65) maps to same q
        assert!(sdf(Vec2::new(0.0, 0.64)) < 0.0, "near spike apex should be inside");
    }

    #[test]
    fn below_circle_outside() {
        // Bottom of scene — circle radius 0.4, below (0, -0.5) is outside
        // Wave at x=0 has zero displacement → SDF behaves like unwrapped here
        assert!(sdf(Vec2::new(0.0, -0.55)) > 0.0);
    }

    #[test]
    fn csg_union_is_inside_both() {
        // Centre is inside circle — union result must also be inside
        let d_circle = {
            let q = warp::wave(Vec2::ZERO, 5.0, 0.05);
            circle(q, Vec2::ZERO, 0.4)
        };
        let d_shape = sdf(Vec2::ZERO);
        assert!(d_circle < 0.0);
        assert!(d_shape < 0.0);
    }

    #[test]
    fn warp_is_pure() {
        let a = sdf(Vec2::new(0.3, 0.2));
        let b = sdf(Vec2::new(0.3, 0.2));
        assert_eq!(a, b);
    }

    #[test]
    fn wave_breaks_horizontal_symmetry() {
        // wave(p, 5.0, 0.05): at x=π/10 sin=1, at x=-π/10 sin=-1
        // Points at same radius but different x will have different y warp
        let a = sdf(Vec2::new( std::f32::consts::PI / 10.0, 0.38));
        let b = sdf(Vec2::new(-std::f32::consts::PI / 10.0, 0.38));
        assert_ne!(a, b, "wave warp should break left-right symmetry on boundary");
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
    fn spike_region_not_background() {
        // With scale=3 and 32×32, pixel at top-centre should be inside (spike)
        // Top-centre in world: y ≈ 0.55 (scale 3 → y range ±0.5*32/32*... → need ~row 6)
        // At x=0, wave has zero y-displacement → spike apex at (0, 0.65) is accessible
        let px = rasterize(sdf, 32, 32, S);
        // Row 6 is near top of image (y ≈ +0.56 in world space with scale=3, 32px)
        let brightness = px[(6 * 32 + 16) * 3];
        assert!(brightness > 100, "spike region should not be background, got {brightness}");
    }
}
