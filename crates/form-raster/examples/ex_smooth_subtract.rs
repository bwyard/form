/// Phase 4 — 2D corruption: CSG smooth subtract
///
/// A circle with a rectangular notch carved out of its right side.
/// Uses `smooth_subtract` to demonstrate CSG subtraction as a corruption tool.
///
/// Scene:
///   body   = circle, radius 0.45, centered at origin
///   notch  = box_2d, 0.3×0.3, offset to the right (center at (0.4, 0.0))
///   shape  = smooth_subtract(notch, body, k=0.08)
///
/// smooth_subtract(d_remove, d_base, k) removes `d_remove` from `d_base`.
/// Interior of notch becomes exterior of the result — the right side of the
/// circle is eaten away with a smooth blend.
///
/// T3 Expected (T3 deferred — run manually):
///   White circle, right side has a smooth rectangular bite removed.
///   Left side is a clean circular arc. Gray edge outline. Dark background.
///
/// Run: `cargo run --example ex_smooth_subtract -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::{circle, box_2d, smooth_subtract};
use glam::Vec2;

fn sdf(p: Vec2) -> f32 {
    let body  = circle(p, Vec2::ZERO, 0.45);
    let notch = box_2d(p, Vec2::new(0.4, 0.0), Vec2::new(0.15, 0.15));
    smooth_subtract(notch, body, 0.08)
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_smooth_subtract ({width}x{height})...");
    println!("T3 Expected: circle with smooth rectangular bite removed from right side");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_smooth_subtract.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_smooth_subtract.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::{circle, box_2d, smooth_subtract};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    fn sdf(p: Vec2) -> f32 {
        let body  = circle(p, Vec2::ZERO, 0.45);
        let notch = box_2d(p, Vec2::new(0.4, 0.0), Vec2::new(0.15, 0.15));
        smooth_subtract(notch, body, 0.08)
    }

    // T1 — CSG correctness

    #[test]
    fn centre_inside() {
        // Centre of circle is far from notch — should remain inside
        assert!(sdf(Vec2::ZERO) < 0.0);
    }

    #[test]
    fn left_inside() {
        // Left side of circle, nowhere near notch — inside
        assert!(sdf(Vec2::new(-0.3, 0.0)) < 0.0);
    }

    #[test]
    fn far_outside() {
        assert!(sdf(Vec2::new(2.0, 0.0)) > 0.0);
        assert!(sdf(Vec2::new(0.0, 2.0)) > 0.0);
    }

    #[test]
    fn notch_centre_removed() {
        // Centre of the notch box (0.4, 0.0): inside the notch, outside the result
        assert!(sdf(Vec2::new(0.4, 0.0)) > 0.0, "notch centre should be carved out");
    }

    #[test]
    fn left_not_affected_by_notch() {
        // Left side of circle at same y as notch — notch doesn't reach here
        let left = sdf(Vec2::new(-0.35, 0.0));
        assert!(left < 0.0, "left of circle should be unaffected by right-side notch");
    }

    #[test]
    fn asymmetric_left_right() {
        // Left inside, right (at notch) outside
        let left  = sdf(Vec2::new(-0.3, 0.0));
        let right = sdf(Vec2::new( 0.4, 0.0));
        assert!(left < 0.0);
        assert!(right > 0.0);
    }

    #[test]
    fn subtract_is_pure() {
        let a = sdf(Vec2::new(0.2, 0.1));
        let b = sdf(Vec2::new(0.2, 0.1));
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
    fn left_pixel_is_white() {
        // Left centre of circle should remain white
        let px = rasterize(sdf, 32, 32, S);
        // pixel at roughly (-0.3, 0.0) — column ~10, row 16
        let col = 10usize;
        let row = 16usize;
        let brightness = px[(row * 32 + col) * 3];
        assert!(brightness > 200, "left of circle should be white, got {brightness}");
    }
}
