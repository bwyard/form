/// Phase 6 — 2D animation: running figure
///
/// A stick figure running — pure f(p, t), no stored state.
///
/// Skeleton (side view):
///   Head   — circle, fixed at top of torso
///   Torso  — capsule, fixed vertical spine
///   Arms   — capsules swinging from shoulders, counter-phase to opposite leg
///   Legs   — capsules swinging from hips, π apart
///
/// Gait: natural cross-pattern
///   Left leg  forward → right arm forward (phase 0)
///   Right leg forward → left arm forward  (phase π)
///
/// Joint model (same as ex_anim_limb):
///   angle = sin(t * 2π * speed + phase) * max_swing
///   end   = pivot + (sin(angle), -cos(angle)) * length
///
/// All parts combined with smooth_union — the blend at joints
/// produces organic shoulders and hips automatically.
///
/// T3 Expected (deferred — run manually):
///   Stick figure running in place. Legs alternate, arms counter-swing.
///   Smooth organic blends at all joints. Dark background.
///
/// Run: `cargo run --example ex_anim_figure -p form-raster`
use form_raster::{rasterize_sequence, RasterSettings, to_bmp};
use form_sdf::{circle, capsule_2d, smooth_union};
use glam::Vec2;

// ── Skeleton geometry ────────────────────────────────────────────────────────

const HEAD_CENTRE:    Vec2 = Vec2::new(0.0,  0.42);
const HEAD_RADIUS:    f32  = 0.09;

const SHOULDER:       Vec2 = Vec2::new(0.0,  0.30);
const HIP:            Vec2 = Vec2::new(0.0, -0.05);
const TORSO_RADIUS:   f32  = 0.055;

const L_SHOULDER:     Vec2 = Vec2::new(-0.06, 0.26);
const R_SHOULDER:     Vec2 = Vec2::new( 0.06, 0.26);
const ARM_LENGTH:     f32  = 0.22;
const ARM_RADIUS:     f32  = 0.038;
const ARM_SWING:      f32  = 0.45; // radians

const L_HIP:          Vec2 = Vec2::new(-0.05, -0.05);
const R_HIP:          Vec2 = Vec2::new( 0.05, -0.05);
const LEG_LENGTH:     f32  = 0.35;
const LEG_RADIUS:     f32  = 0.050;
const LEG_SWING:      f32  = 0.60; // radians

const SPEED:          f32  = 1.0;  // full gait cycles per second
const K_JOINT:        f32  = 0.04; // smooth_union blend at joints
const K_BODY:         f32  = 0.03; // tighter blend between major segments

// ── Joint helper ─────────────────────────────────────────────────────────────

fn joint_end(pivot: Vec2, t: f32, phase: f32, length: f32, swing: f32) -> Vec2 {
    let angle = (t * std::f32::consts::TAU * SPEED + phase).sin() * swing;
    pivot + Vec2::new(angle.sin(), -angle.cos()) * length
}

// ── Scene SDF ────────────────────────────────────────────────────────────────

fn sdf(p: Vec2, t: f32) -> f32 {
    let pi = std::f32::consts::PI;

    // Head + torso — fixed
    let head  = circle(p, HEAD_CENTRE, HEAD_RADIUS);
    let torso = capsule_2d(p, SHOULDER, HIP, TORSO_RADIUS);

    // Arms — counter-phase to opposite leg
    let l_arm = capsule_2d(p, L_SHOULDER, joint_end(L_SHOULDER, t, pi,  ARM_LENGTH, ARM_SWING), ARM_RADIUS);
    let r_arm = capsule_2d(p, R_SHOULDER, joint_end(R_SHOULDER, t, 0.0, ARM_LENGTH, ARM_SWING), ARM_RADIUS);

    // Legs — π apart
    let l_leg = capsule_2d(p, L_HIP, joint_end(L_HIP, t, 0.0, LEG_LENGTH, LEG_SWING), LEG_RADIUS);
    let r_leg = capsule_2d(p, R_HIP, joint_end(R_HIP, t, pi,  LEG_LENGTH, LEG_SWING), LEG_RADIUS);

    // Assemble — head blends into torso, limbs blend into body
    let body = smooth_union(head,  torso, K_JOINT);
    let body = smooth_union(body,  l_arm, K_BODY);
    let body = smooth_union(body,  r_arm, K_BODY);
    let body = smooth_union(body,  l_leg, K_BODY);
    smooth_union(body, r_leg, K_BODY)
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let width       = 256u32;
    let height      = 256u32;
    let frame_count = 32usize;
    let dt          = 1.0 / frame_count as f32;
    let settings    = RasterSettings { scale: 1.8, edge_width: 0.02, flat_bg: false };

    println!("Rendering ex_anim_figure — {frame_count} frames ({width}x{height})...");

    let frames = rasterize_sequence(sdf, width, height, settings, frame_count, dt);

    for (i, pixels) in frames.iter().enumerate() {
        let path = format!("ex_anim_figure_{i:03}.bmp");
        std::fs::write(&path, to_bmp(pixels, width, height)).unwrap();
    }

    println!(
        "Saved {frame_count} frames: ex_anim_figure_000.bmp .. ex_anim_figure_{:03}.bmp",
        frame_count - 1
    );
    println!("T3 Expected: stick figure running in place, legs alternate, arms counter-swing");
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::{sdf, joint_end, L_HIP, R_HIP, L_SHOULDER, R_SHOULDER, HEAD_CENTRE, LEG_LENGTH, LEG_SWING, ARM_LENGTH, ARM_SWING, SPEED};
    use form_raster::{rasterize, rasterize_sequence, RasterSettings};
    use glam::Vec2;

    const S: RasterSettings = RasterSettings { scale: 1.8, edge_width: 0.02, flat_bg: false };

    // T1 — skeleton correctness

    #[test]
    fn head_always_inside() {
        assert!(sdf(HEAD_CENTRE, 0.0)  < 0.0);
        assert!(sdf(HEAD_CENTRE, 0.25) < 0.0);
        assert!(sdf(HEAD_CENTRE, 0.5)  < 0.0);
    }

    #[test]
    fn torso_centre_always_inside() {
        let mid = Vec2::new(0.0, 0.12); // midpoint of torso
        assert!(sdf(mid, 0.0)  < 0.0);
        assert!(sdf(mid, 0.25) < 0.0);
    }

    #[test]
    fn far_corner_always_outside() {
        assert!(sdf(Vec2::new(3.0,  3.0), 0.0)  > 0.0);
        assert!(sdf(Vec2::new(3.0, -3.0), 0.25) > 0.0);
    }

    #[test]
    fn legs_are_π_apart() {
        // At t=0.25 (quarter period): left leg should swing right, right leg left
        let l_end = joint_end(L_HIP, 0.25, 0.0, LEG_LENGTH, LEG_SWING);
        let r_end = joint_end(R_HIP, 0.25, std::f32::consts::PI, LEG_LENGTH, LEG_SWING);
        // Left leg swings forward (positive x direction for forward)
        // Right leg swings backward — ends should be on opposite x sides
        assert_ne!(l_end.x.signum(), r_end.x.signum(),
            "legs should swing in opposite directions: l={:.3}, r={:.3}", l_end.x, r_end.x);
    }

    #[test]
    fn arms_counter_phase_to_legs() {
        // Right arm (phase 0) swings same direction as left leg (phase 0)
        let r_arm_end = joint_end(R_SHOULDER, 0.25, 0.0, ARM_LENGTH, ARM_SWING);
        let l_leg_end = joint_end(L_HIP,      0.25, 0.0, LEG_LENGTH, LEG_SWING);
        // Both have phase 0 so they swing in the same x direction
        assert_eq!(r_arm_end.x.signum(), l_leg_end.x.signum(),
            "right arm and left leg should swing together");
    }

    #[test]
    fn full_period_returns_to_start() {
        let a = sdf(Vec2::new(0.05, -0.1), 0.0);
        let b = sdf(Vec2::new(0.05, -0.1), 1.0);
        assert!((a - b).abs() < 1e-4, "t=0 and t=1 should match: {a} vs {b}");
    }

    #[test]
    fn sdf_is_pure() {
        let a = sdf(Vec2::new(0.05, 0.1), 0.37);
        let b = sdf(Vec2::new(0.05, 0.1), 0.37);
        assert_eq!(a, b);
    }

    #[test]
    fn leg_endpoints_at_correct_distance() {
        // The foot should always be LEG_LENGTH from the hip (capsule endpoint, not capsule surface)
        let foot = joint_end(L_HIP, 0.25, 0.0, LEG_LENGTH, LEG_SWING);
        let dist = (foot - L_HIP).length();
        assert!((dist - LEG_LENGTH).abs() < 1e-5, "foot should be LEG_LENGTH from hip: {dist}");
    }

    // T2 — render

    #[test]
    fn head_pixel_is_white() {
        // scale=1.8, head at (0, 0.42) → row = (0.5 - 0.42/1.8)*32 ≈ 9, col=16
        let px = rasterize(|p| sdf(p, 0.0), 32, 32, S);
        let col = 16usize;
        let row = 9usize;
        let brightness = px[(row * 32 + col) * 3];
        assert!(brightness > 200, "head pixel should be white, got {brightness}");
    }

    #[test]
    fn corner_is_dark() {
        let px = rasterize(|p| sdf(p, 0.0), 16, 16, S);
        assert!(px[0] < 100);
    }

    #[test]
    fn frames_differ() {
        let frames = rasterize_sequence(sdf, 16, 16, S, 4, 0.25);
        let differ = frames[0].iter().zip(frames[1].iter()).any(|(a, b)| a != b);
        assert!(differ, "frames should differ as figure runs");
    }

    #[test]
    fn quarter_period_frame_differs_from_start() {
        // t=0: sin(0)=0 — limbs hang straight (neutral)
        // t=0.25: sin(π/2)=1 — limbs at max swing
        // These should produce visibly different images
        let frames = rasterize_sequence(sdf, 32, 32, S, 4, 0.25);
        let differ = frames[0].iter().zip(frames[1].iter()).any(|(a, b)| a != b);
        assert!(differ, "quarter-period frame (max swing) should differ from neutral start");
    }
}
