/// Sphere demo — the simplest possible form-render output.
///
/// Renders a lit unit sphere and writes it to `sphere.bmp`.
/// Open sphere.bmp in any image viewer to verify the renderer works.
use form_render::{Camera, MarchSettings, render_image, to_bmp};
use glam::Vec3;

fn main() {
    let width  = 512u32;
    let height = 512u32;

    let sdf = |p: Vec3| p.length() - 1.0;

    let camera = Camera::look_at(
        Vec3::new(0.0, 0.5, -3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        width as f32 / height as f32,
    );

    let light_dir = Vec3::new(1.0, 2.0, -1.0).normalize();

    let settings = MarchSettings {
        max_steps:   128,
        max_dist:    100.0,
        hit_epsilon: 0.001,
    };

    println!("Rendering {width}x{height} sphere...");
    let pixels = render_image(sdf, camera, light_dir, width, height, settings);
    let bmp    = to_bmp(&pixels, width, height);

    std::fs::write("sphere.bmp", &bmp).expect("failed to write sphere.bmp");
    println!("Saved sphere.bmp — open it to verify the renderer.");
}
