//! WASM bindings for form — SDF rendering in the browser.
//!
//! Exports `render_frame(t, width, height) -> RGBA bytes`.
//! The scene is the running figure from Phase 6 (ex_anim_figure).
//!
//! Assembly rule: no STORE, no JUMP.
//! Same (t, width, height) always produces the same pixel buffer.

use form_raster::{rasterize, RasterSettings};
use form_sdf::{circle, capsule_2d, smooth_union};
use glam::Vec2;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// ── Skeleton constants ────────────────────────────────────────────────────────

const HEAD_CENTRE:  Vec2 = Vec2::new(0.0,  0.42);
const HEAD_RADIUS:  f32  = 0.09;
const SHOULDER:     Vec2 = Vec2::new(0.0,  0.30);
const HIP:          Vec2 = Vec2::new(0.0, -0.05);
const TORSO_RADIUS: f32  = 0.055;
const L_SHOULDER:   Vec2 = Vec2::new(-0.06, 0.26);
const R_SHOULDER:   Vec2 = Vec2::new( 0.06, 0.26);
const ARM_LENGTH:   f32  = 0.22;
const ARM_RADIUS:   f32  = 0.038;
const ARM_SWING:    f32  = 0.45;
const L_HIP:        Vec2 = Vec2::new(-0.05, -0.05);
const R_HIP:        Vec2 = Vec2::new( 0.05, -0.05);
const LEG_LENGTH:   f32  = 0.35;
const LEG_RADIUS:   f32  = 0.050;
const LEG_SWING:    f32  = 0.60;
const SPEED:        f32  = 1.0;

// ── Scene ────────────────────────────────────────────────────────────────────

fn joint_end(pivot: Vec2, t: f32, phase: f32, length: f32, swing: f32) -> Vec2 {
    let angle = (t * std::f32::consts::TAU * SPEED + phase).sin() * swing;
    pivot + Vec2::new(angle.sin(), -angle.cos()) * length
}

fn figure(p: Vec2, t: f32) -> f32 {
    let pi = std::f32::consts::PI;

    let head  = circle(p, HEAD_CENTRE, HEAD_RADIUS);
    let torso = capsule_2d(p, SHOULDER, HIP, TORSO_RADIUS);

    let l_arm = capsule_2d(p, L_SHOULDER, joint_end(L_SHOULDER, t, pi,  ARM_LENGTH, ARM_SWING), ARM_RADIUS);
    let r_arm = capsule_2d(p, R_SHOULDER, joint_end(R_SHOULDER, t, 0.0, ARM_LENGTH, ARM_SWING), ARM_RADIUS);
    let l_leg = capsule_2d(p, L_HIP, joint_end(L_HIP, t, 0.0, LEG_LENGTH, LEG_SWING), LEG_RADIUS);
    let r_leg = capsule_2d(p, R_HIP, joint_end(R_HIP, t, pi,  LEG_LENGTH, LEG_SWING), LEG_RADIUS);

    let body = smooth_union(head,  torso, 0.04);
    let body = smooth_union(body,  l_arm, 0.03);
    let body = smooth_union(body,  r_arm, 0.03);
    let body = smooth_union(body,  l_leg, 0.03);
    smooth_union(body, r_leg, 0.03)
}

// ── WASM export ───────────────────────────────────────────────────────────────

/// Render one frame of the running figure at time `t`.
///
/// Returns a flat RGBA byte buffer — `width * height * 4` bytes.
/// Suitable for direct use with `ImageData` on an HTML canvas.
///
/// `t` is in seconds. The gait cycle period is 1.0 second.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn render_frame(t: f32, width: u32, height: u32) -> Vec<u8> {
    let settings = RasterSettings { scale: 1.8, edge_width: 0.02, flat_bg: false };
    let rgb = rasterize(|p| figure(p, t), width, height, settings);
    // Convert RGB → RGBA for ImageData
    rgb.chunks_exact(3)
        .flat_map(|c| [c[0], c[1], c[2], 255u8])
        .collect()
}
