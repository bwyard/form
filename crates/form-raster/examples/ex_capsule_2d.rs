/// Phase 3 — 2D primitive gallery: capsule_2d
///
/// T3 Expected: white filled vertical pill shape centered in frame —
///              flat sides, two semicircular rounded caps top and bottom,
///              thin gray edge, dark background.
///
/// Run: `cargo run --example ex_capsule_2d -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::capsule_2d;
use glam::Vec2;

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };

    println!("Rendering capsule_2d ({width}x{height})...");
    println!("T3 Expected: white vertical pill shape, rounded caps top+bottom, gray edge, dark background");

    let pixels = rasterize(
        |p| capsule_2d(p, Vec2::new(0.0, -0.6), Vec2::new(0.0, 0.6), 0.4),
        width, height, settings,
    );
    std::fs::write("ex_capsule_2d.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_capsule_2d.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::capsule_2d;
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: false };
    fn sdf(p: Vec2) -> f32 { capsule_2d(p, Vec2::new(0.0, -0.6), Vec2::new(0.0, 0.6), 0.4) }

    // T1
    #[test] fn centre_inside()         { assert!(sdf(Vec2::ZERO) < 0.0); }
    #[test] fn top_cap_inside()        { assert!(sdf(Vec2::new(0.0, 0.8)) < 0.0); }
    #[test] fn bottom_cap_inside()     { assert!(sdf(Vec2::new(0.0, -0.8)) < 0.0); }
    #[test] fn wide_outside()          { assert!(sdf(Vec2::new(1.5, 0.0)) > 0.0); }
    #[test] fn far_above_outside()     { assert!(sdf(Vec2::new(0.0, 1.5)) > 0.0); }
    #[test] fn surface_side_near_zero(){ assert!(sdf(Vec2::new(0.4, 0.0)).abs() < 1e-5); }

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
    #[test]
    fn taller_than_wide() {
        // Count white pixels in centre column vs centre row — capsule is vertical
        let px = rasterize(sdf, 16, 16, S);
        let col_white: u32 = (0..16u32).map(|row| (px[((row * 16 + 8) * 3) as usize] == 255) as u32).sum();
        let row_white: u32 = (0..16u32).map(|col| (px[((8 * 16 + col) * 3) as usize] == 255) as u32).sum();
        assert!(col_white > row_white, "vertical capsule should be taller than wide: col={col_white} row={row_white}");
    }
}
