/// Phase 3 — 2D primitive gallery: box_2d
///
/// T3 Expected: white filled square centered in frame,
///              sharp 90-degree corners (not rounded),
///              thin gray edge, dark background.
///
/// Run: `cargo run --example ex_box_2d -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::box_2d;
use glam::Vec2;

fn main() {
    let width  = 512u32;
    let height = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    println!("Rendering box_2d ({width}x{height})...");
    println!("T3 Expected: white square with sharp corners, gray edge, dark background");

    let pixels = rasterize(
        |p| box_2d(p, Vec2::ZERO, Vec2::new(0.8, 0.8)),
        width, height, settings,
    );
    std::fs::write("ex_box_2d.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_box_2d.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::box_2d;
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };
    fn sdf(p: Vec2) -> f32 { box_2d(p, Vec2::ZERO, Vec2::new(0.8, 0.8)) }

    // T1
    #[test] fn inside_is_negative()  { assert!(sdf(Vec2::ZERO) < 0.0); }
    #[test] fn outside_is_positive() { assert!(sdf(Vec2::new(2.0, 0.0)) > 0.0); }
    #[test] fn surface_near_zero()   { assert!(sdf(Vec2::new(0.8, 0.0)).abs() < 1e-5); }
    #[test] fn corner_is_outside()   { assert!(sdf(Vec2::new(1.2, 1.2)) > 0.0); }

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
