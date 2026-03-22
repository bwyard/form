/// Phase 4 — 2D corruption: square with spike
///
/// Demonstrates SDF library composition:
///   shape = smooth_union(box_2d(p), isosceles_triangle(p), k)
///
/// This is the first example of corruption as a library API —
/// not a warp, but CSG: two pure SDFs composed with smooth_union.
///
/// T3 Expected: white rounded-square with a triangular spike pointing
///              upward from its top edge, blended smoothly at the join,
///              thin gray edge outline, dark background.
///
/// Run: `cargo run --example ex_square_with_spike -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::{box_2d, isosceles_triangle, smooth_union};
use glam::Vec2;

fn sdf(p: Vec2) -> f32 {
    let body   = box_2d(p, Vec2::ZERO, Vec2::new(0.5, 0.5));
    let spike0 = isosceles_triangle(p, Vec2::new(-0.22, 0.5), 0.1, 0.3);
    let spike1 = isosceles_triangle(p, Vec2::new( 0.22, 0.5), 0.1, 0.3);
    let spikes = smooth_union(spike0, spike1, 0.05);
    smooth_union(body, spikes, 0.08)
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_square_with_spike ({width}x{height})...");
    println!("T3 Expected: white square with upward spike, smooth blend at join, gray edge, dark background");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_square_with_spike.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_square_with_spike.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::{box_2d, isosceles_triangle, smooth_union};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    fn sdf(p: Vec2) -> f32 {
        let body   = box_2d(p, Vec2::ZERO, Vec2::new(0.5, 0.5));
        let spike0 = isosceles_triangle(p, Vec2::new(-0.22, 0.5), 0.1, 0.3);
        let spike1 = isosceles_triangle(p, Vec2::new( 0.22, 0.5), 0.1, 0.3);
        let spikes = smooth_union(spike0, spike1, 0.05);
        smooth_union(body, spikes, 0.08)
    }

    // T1 — math
    #[test]
    fn centre_of_box_inside() {
        assert!(sdf(Vec2::ZERO) < 0.0);
    }

    #[test]
    fn apex_of_left_spike_near_surface() {
        // left spike apex is at (-0.22, 0.8) — on boundary → near 0
        let d = sdf(Vec2::new(-0.22, 0.8));
        assert!(d.abs() < 0.15, "left apex should be near surface, got {d}");
    }

    #[test]
    fn far_outside_is_positive() {
        assert!(sdf(Vec2::new(3.0, 0.0)) > 0.0);
        assert!(sdf(Vec2::new(0.0, 3.0)) > 0.0);
    }

    #[test]
    fn left_spike_body_inside() {
        // midpoint of left spike body — confirmed inside (d ≈ -0.047)
        let d = sdf(Vec2::new(-0.22, 0.65));
        assert!(d < 0.0, "left spike body should be inside, got {d}");
    }

    #[test]
    fn right_spike_body_inside() {
        let d = sdf(Vec2::new(0.22, 0.65));
        assert!(d < 0.0, "right spike body should be inside, got {d}");
    }

    #[test]
    fn between_spikes_above_box_outside() {
        // midpoint between the two spikes, above the box — neither spike covers this
        let d = sdf(Vec2::new(0.0, 0.75));
        assert!(d > 0.0, "gap between spikes should be outside, got {d}");
    }

    #[test]
    fn beside_spikes_outside() {
        let d = sdf(Vec2::new(1.0, 0.9));
        assert!(d > 0.0, "far beside should be outside, got {d}");
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
