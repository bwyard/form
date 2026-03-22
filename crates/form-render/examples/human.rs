/// Running human demo — corruption stack + pure trig gait.
///
/// Renders a human figure at a given time `t` and writes it to `human_NNN.bmp`.
/// Run: `cargo run --example human -- 0.0`
///
/// Architecture:
///   Ground state: sphere (head) + elongated sphere (neck)
///   Corruption: domain warps applied to input point before SDF eval
///   Gait: lateral sway + vertical bob translate the whole figure
use form_render::{Camera, MarchSettings, render_image, to_bmp};
use form_animate::gait;
use glam::Vec3;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn smooth_union(a: f32, b: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (b - a) / k).clamp(0.0, 1.0);
    b + h * (a - b) - k * h * (1.0 - h)
}

fn smooth_subtract(a: f32, b: f32, k: f32) -> f32 {
    let h = (0.5 - 0.5 * (a + b) / k).clamp(0.0, 1.0);
    a - h * (a + b) + k * h * (1.0 - h)
}

// ---------------------------------------------------------------------------
// Corruption — all as domain warps (shift input p, not output SDF value)
// ---------------------------------------------------------------------------

/// Bilateral asymmetry — tiny offset by hemisphere, models natural skull drift.
fn asym_warp(p: Vec3) -> Vec3 {
    let drift = if p.x > 0.0 { 0.015 } else { -0.008 };
    Vec3::new(p.x, p.y + drift, p.z + drift * 0.5)
}

/// Jaw widening — scale x outward in the lower face region.
fn jaw_warp(p: Vec3) -> Vec3 {
    let mask = ((-p.y - 0.15) / 0.5).clamp(0.0, 1.0);
    Vec3::new(p.x / (1.0 + mask * 0.18), p.y, p.z)
}

/// Occipital flattening — push back of skull slightly inward.
fn occipital_warp(p: Vec3) -> Vec3 {
    let back_mask = (p.z / 0.8).clamp(0.0, 1.0);
    Vec3::new(p.x, p.y - back_mask * 0.06, p.z * (1.0 + back_mask * 0.1))
}

/// Brow ridge — push forehead forward in a band at y ≈ 0.35.
fn brow_warp(p: Vec3) -> Vec3 {
    let y_band  = (-(p.y - 0.35).powi(2) / 0.02).exp(); // Gaussian centred at brow
    let z_front = (-p.z).max(0.0).min(1.0);              // front hemisphere only
    Vec3::new(p.x, p.y, p.z - y_band * z_front * 0.07)  // push forward (−z)
}

/// Micro surface — sub-millimetre skin noise via cheap sin hash.
/// Placeholder until prime-noise ships (same API, just swap implementation).
fn micro_surface(p: Vec3) -> f32 {
    ((p.x * 47.3 + p.y * 31.7).sin()
     * (p.y * 71.9 + p.z * 19.3).sin()).abs() * 0.003
}

// ---------------------------------------------------------------------------
// Full figure SDF — human(p, t) -> signed distance
// ---------------------------------------------------------------------------

fn human(p: Vec3, t: f32) -> f32 {
    // Gait — translate the whole figure; t is a pure parameter
    let sway = gait::lateral_sway(t, 0.06, 1.0);
    let bob  = gait::vertical_bob(t, 0.03, 1.0);
    let lean = gait::sagittal_rotation(t, 0.03, 1.0);
    let p = Vec3::new(p.x - sway, p.y - bob, p.z - lean * 0.1);

    // --- Head ---
    // Apply corruption as domain warps (shift p, then evaluate sphere)
    let ph = brow_warp(occipital_warp(jaw_warp(asym_warp(p))));
    let head = ph.length() - 1.0;

    // Brow ridge: a small torus-shaped bulge across y ≈ 0.35, front only
    let brow_p   = Vec3::new(p.x, p.y - 0.35, p.z);
    let brow_r   = (Vec3::new(brow_p.x, 0.0, brow_p.z).length() - 0.55).abs();
    let brow_sdf = (brow_r * brow_r + brow_p.y * brow_p.y).sqrt() - 0.09;
    let front    = (-p.z).max(0.0).min(1.0);
    let head_browed = smooth_union(head, brow_sdf * front + head * (1.0 - front), 0.04);

    // Temporal hollows: small concave dents at temples
    let hollow_l = (p - Vec3::new(-0.82, 0.1, -0.1)).length() - 0.32;
    let hollow_r = (p - Vec3::new( 0.82, 0.1, -0.1)).length() - 0.32;
    let head_hollowed = smooth_subtract(head_browed, hollow_l.min(hollow_r), 0.05);

    // Micro-surface noise
    let head_final = head_hollowed + micro_surface(p);

    // --- Neck ---
    let neck_p = Vec3::new(p.x * 0.65, p.y + 1.45, p.z * 0.78);
    let neck = neck_p.length() - 0.40;

    smooth_union(head_final, neck, 0.22)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let t: f32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let width  = 512u32;
    let height = 512u32;

    let camera = Camera::look_at(
        Vec3::new(0.0, 0.2, -4.0),
        Vec3::new(0.0, -0.2, 0.0),
        Vec3::Y,
        50.0,
        width as f32 / height as f32,
    );

    let light_dir = Vec3::new(1.0, 2.0, -1.5).normalize();

    let settings = MarchSettings {
        max_steps:   256,
        max_dist:    50.0,
        hit_epsilon: 0.001,
    };

    println!("Rendering human at t={t:.2} ({width}x{height})...");
    let pixels = render_image(|p| human(p, t), camera, light_dir, width, height, settings);
    let bmp    = to_bmp(&pixels, width, height);

    let filename = format!("human_{:05.2}.bmp", t);
    std::fs::write(&filename, &bmp).expect("failed to write bmp");
    println!("Saved {filename}");
}
