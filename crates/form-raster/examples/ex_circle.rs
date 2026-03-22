/// Phase 3 — 2D primitive gallery: circle (baseline)
///
/// T3 Expected: white filled circle centered in frame,
///              thin gray edge ring visible,
///              dark blue-grey background outside,
///              background gets slightly darker toward corners.
///
/// Run: `cargo run --example ex_circle -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use glam::Vec2;

fn main() {
    let width  = 512u32;
    let height = 512u32;

    let sdf = |p: Vec2| p.length() - 1.0;

    let settings = RasterSettings { scale: 3.0, edge_width: 0.02 };

    println!("Rendering circle 2D ({width}x{height})...");
    println!("T3 Expected: white filled circle, gray edge ring, dark background");

    let pixels = rasterize(sdf, width, height, settings);
    let bmp    = to_bmp(&pixels, width, height);
    std::fs::write("ex_circle.bmp", &bmp).expect("failed to write bmp");
    println!("Saved ex_circle.bmp");
}
