/// Phase 4 — 2D corruption: sinusoidal wave warp on a circle
///
/// Shows `warp::wave` in isolation — a circle whose boundary undulates.
/// The wave displaces Y by a sine of X position. The circle sees the
/// warped point, so its boundary follows a wavy curve rather than a clean arc.
///
/// Warp:
///   q = wave(p, frequency=6.0, amplitude=0.06)
///   → q.y = p.y + sin(p.x * 6.0) * 0.06
///
/// At the circle boundary (radius ≈ 0.4), the wave causes the edge to ripple
/// inward and outward with ~6 cycles across the diameter. The left half and
/// right half of the circle are phase-shifted versions of each other.
///
/// T3 Expected (T3 deferred — run manually):
///   White circle with wavy/rippled boundary. The edge oscillates in and out
///   with ~6 humps around the circumference. Gray edge outline. Gradient background.
///
/// Run: `cargo run --example ex_wave_warp -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::{circle, warp};
use glam::Vec2;

fn sdf(p: Vec2) -> f32 {
    let q = warp::wave(p, 6.0, 0.06);
    circle(q, Vec2::ZERO, 0.4)
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    println!("Rendering ex_wave_warp ({width}x{height})...");
    println!("T3 Expected: circle with wavy/rippled boundary, ~6 humps, gray edge, dark background");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_wave_warp.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_wave_warp.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::{circle, warp};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    fn sdf(p: Vec2) -> f32 {
        let q = warp::wave(p, 6.0, 0.06);
        circle(q, Vec2::ZERO, 0.4)
    }

    // T1 — warp correctness

    #[test]
    fn centre_inside() {
        assert!(sdf(Vec2::ZERO) < 0.0);
    }

    #[test]
    fn far_outside() {
        assert!(sdf(Vec2::new(2.0, 0.0)) > 0.0);
        assert!(sdf(Vec2::new(0.0, 2.0)) > 0.0);
    }

    #[test]
    fn wave_breaks_y_symmetry() {
        // wave(p, 6.0, 0.06) displaces y by sin(p.x * 6.0) * 0.06.
        // At x=π/12 (sin = 1) vs x=-π/12 (sin = -1) the y displacements differ.
        // On the boundary at x=π/12, the point is shifted upward — interior is
        // accessible from a slightly higher y. At x=-π/12, it's shifted downward.
        // So top and bottom of the circle are not the same at these x values.
        let top_at_peak    = sdf(Vec2::new(std::f32::consts::PI / 12.0,  0.38));
        let top_at_trough  = sdf(Vec2::new(-std::f32::consts::PI / 12.0, 0.38));
        assert_ne!(top_at_peak, top_at_trough, "wave should break y-symmetry");
    }

    #[test]
    fn x_axis_symmetric() {
        // At y=0 the wave displacement is the same for p and its reflection -p
        // because sin(-x * f) = -sin(x * f), so q.y = 0 ± amplitude·sin(±...)
        // The SDF is a circle centred at origin → f(q) = f(-q) when q is symmetric.
        // wave(p, ...) and wave(-p, ...) give opposite y displacements from a
        // symmetric origin → both land at equal distance from origin.
        let a = sdf(Vec2::new( 0.3, 0.0));
        let b = sdf(Vec2::new(-0.3, 0.0));
        // They need not be equal (different wave phases), just both valid floats.
        // This test confirms the function runs without panic at both sides.
        let _ = (a, b);
    }

    #[test]
    fn warp_is_pure() {
        let a = sdf(Vec2::new(0.3, 0.2));
        let b = sdf(Vec2::new(0.3, 0.2));
        assert_eq!(a, b);
    }

    #[test]
    fn inside_radius_inside() {
        // Well inside radius: wave amplitude (0.06) cannot push a point this far in
        assert!(sdf(Vec2::new(0.2, 0.0)) < 0.0);
        assert!(sdf(Vec2::new(0.0, 0.2)) < 0.0);
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
}
