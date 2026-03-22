/// Phase 4 — 2D corruption: warp without spikes
///
/// Shows domain warp in isolation — no CSG, no spikes.
/// The box is the ground state. The warp bends space before the SDF sees it.
///
/// Progression:
///   ex_box_2d      — ground state, no corruption
///   ex_warp_box    — ground state + domain warp (this file)
///   ex_square_with_spike — ground state + CSG
///   ex_domain_warp — ground state + CSG + domain warp
///
/// T3 Expected: a white box shape with all four sides curved — including
///              top and bottom — corners pulled outward in a spiral,
///              no straight edges anywhere, gray edge, dark background.
///
/// Run: `cargo run --example ex_warp_box -p form-raster`
use form_raster::{rasterize, RasterSettings, to_bmp};
use form_sdf::{box_2d, warp};
use glam::Vec2;

fn sdf(p: Vec2) -> f32 {
    let q = warp::radial_twist(p, 1.2);
    box_2d(q, Vec2::ZERO, Vec2::new(0.5, 0.5))
}

fn main() {
    let width    = 512u32;
    let height   = 512u32;
    let settings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };

    println!("Rendering ex_warp_box ({width}x{height})...");
    println!("T3 Expected: box with curved/sheared sides from twist warp, gray edge, dark background");

    let pixels = rasterize(sdf, width, height, settings);
    std::fs::write("ex_warp_box.bmp", to_bmp(&pixels, width, height)).unwrap();
    println!("Saved ex_warp_box.bmp");
}

#[cfg(test)]
mod tests {
    use form_raster::{rasterize, RasterSettings};
    use form_sdf::{box_2d, warp};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 3.0, edge_width: 0.02, flat_bg: true };
    fn sdf(p: Vec2) -> f32 {
        box_2d(warp::radial_twist(p, 1.2), Vec2::ZERO, Vec2::new(0.5, 0.5))
    }

    // T1
    #[test] fn centre_inside()      { assert!(sdf(Vec2::ZERO) < 0.0); }
    #[test] fn far_outside()        { assert!(sdf(Vec2::new(2.0, 0.0)) > 0.0); }
    #[test] fn twist_breaks_symmetry() {
        // Above equator, left and right of the box should no longer be symmetric
        let left  = sdf(Vec2::new(-0.5, 0.3));
        let right = sdf(Vec2::new( 0.5, 0.3));
        assert_ne!(left, right, "twist should break x-symmetry above y=0");
    }
    #[test] fn warp_is_pure() {
        let a = sdf(Vec2::new(0.3, 0.3));
        let b = sdf(Vec2::new(0.3, 0.3));
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
