/// Phase 4 — 2D corruption: localized radial bulge
///
/// The right side of the box is pushed outward in a circular arc.
/// Left side is untouched — flat straight edge.
///
/// Warp: radial compression of `p`, fading in from left to right.
///   t = smoothstep(-0.5, 0.5, p.x)   — 0 on left, 1 on right
///   q = p / (1 + t * strength)       — compress p toward origin on right
///
/// When p is compressed, the box SDF evaluates it as closer to the interior
/// than it really is → the shape visually extends further on the right.
/// Because the compression is radial (uniform in all directions), the
/// extended boundary follows a roughly circular arc.
///
/// Left at t=0: q = p → original box. Right at t=1: q = p/1.5 → box is 1.5× wider.
///
/// No SDF output is touched. Thesis holds.
///
/// T3 Expected: white box — left edge perfectly flat and straight,
///              right side bulging outward in a smooth circular arc,
///              top and bottom corners follow the arc outward on the right,
///              gray edge outline, dark background.
///
/// Run: `cargo run --example ex_local_warp -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::box_2d;
use glam::Vec2;

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn sdf(p: Vec2) -> f32 {
    // Circular arc warp — right edge follows x² + y² = R² (radius 0.75).
    // y is never touched → top and bottom edges stay perfectly horizontal.
    // Only x is scaled, and the scale factor is y-dependent to produce a circle.
    let r    = 0.75_f32;
    let x_arc = (r * r - p.y * p.y).max(1e-4).sqrt(); // circle x at this y
    let t     = smoothstep(-0.5, 0.5, p.x);            // 0=left, 1=right
    // At t=1: scale x so that p.x = x_arc maps to q.x = 0.5 (box right edge)
    // At t=0: scale = 1 (identity — left side unchanged)
    let scale_x = t * (0.5 / x_arc) + (1.0 - t);
    let q = Vec2::new(p.x * scale_x, p.y);
    box_2d(q, Vec2::ZERO, Vec2::new(0.5, 0.5))
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    println!("Rendering ex_local_warp ({width}x{height})...");
    println!("T3 Expected: box — left edge flat and straight, right side circular arc bulge, smooth transition");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_local_warp.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_local_warp.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::box_2d;
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    fn sdf(p: Vec2) -> f32 {
        let r       = 0.75_f32;
        let x_arc   = (r * r - p.y * p.y).max(1e-4).sqrt();
        let t       = smoothstep(-0.5, 0.5, p.x);
        let scale_x = t * (0.5 / x_arc) + (1.0 - t);
        let q       = Vec2::new(p.x * scale_x, p.y);
        box_2d(q, Vec2::ZERO, Vec2::new(0.5, 0.5))
    }

    // T1 — warp locality
    #[test]
    fn centre_inside() { assert!(sdf(Vec2::ZERO) < 0.0); }

    #[test]
    fn far_outside() { assert!(sdf(Vec2::new(2.0, 0.0)) > 0.0); }

    #[test]
    fn left_edge_unchanged() {
        // Left edge of box at x=-0.5: t=0, q=p → original box surface
        let d = sdf(Vec2::new(-0.5, 0.0));
        assert!(d.abs() < 1e-4, "left edge should still be on surface, got {d}");
    }

    #[test]
    fn right_side_extends_beyond_box() {
        // At x=0.6 (outside original box right edge of 0.5): warp pulls it inside
        let d = sdf(Vec2::new(0.6, 0.0));
        assert!(d < 0.0, "right of box edge should be inside after bulge warp, got {d}");
    }

    #[test]
    fn left_right_asymmetric() {
        // Right side extends further than left, so right outside-box is inside, left is not
        let left  = sdf(Vec2::new(-0.6, 0.0));
        let right = sdf(Vec2::new( 0.6, 0.0));
        assert!(left > 0.0,  "left outside box should still be outside");
        assert!(right < 0.0, "right outside box should be inside after bulge");
    }

    #[test]
    fn warp_is_pure() {
        let a = sdf(Vec2::new(0.4, 0.3));
        let b = sdf(Vec2::new(0.4, 0.3));
        assert_eq!(a, b);
    }

    // T2
    #[test]
    fn centre_is_white() {
        let px = rasterize(sdf, 16, 16, S);
        assert_eq!(px[(8 * 16 + 8) * 3], 255);
    }
    #[test]
    fn corner_is_dark() {
        let px = rasterize(sdf, 16, 16, S);
        assert!(px[0] < 100);
    }
}
