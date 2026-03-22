//! Lighting — normal estimation, diffuse, shadow.
//!
//! All functions are LOAD + COMPUTE only. No STORE, no JUMP.
//! Same inputs → same outputs, always.

use glam::Vec3;
use crate::march::{march, MarchSettings};

/// Estimate the surface normal at a point using finite differences.
///
/// # Math
///
/// ```text
/// normal = normalize(
///   sdf(p + (e,0,0)) - sdf(p - (e,0,0)),
///   sdf(p + (0,e,0)) - sdf(p - (0,e,0)),
///   sdf(p + (0,0,e)) - sdf(p - (0,0,e)),
/// )
/// ```
///
/// # Arguments
/// * `sdf`     — the signed distance function
/// * `point`   — surface point to estimate normal at
/// * `epsilon` — finite difference step size (typical: 0.0001)
///
/// # Returns
/// Approximate unit outward normal at `point`.
///
/// # Example
/// ```rust
/// use glam::Vec3;
/// use form_render::light::normal_at;
///
/// // Normal on unit sphere points radially outward
/// let sdf = |p: Vec3| p.length() - 1.0;
/// let n = normal_at(sdf, Vec3::X, 0.0001);
/// assert!((n - Vec3::X).length() < 0.01);
/// ```
pub fn normal_at<F>(sdf: F, point: Vec3, epsilon: f32) -> Vec3
where
    F: Fn(Vec3) -> f32,
{
    let e = Vec3::new(epsilon, 0.0, 0.0);
    let ex = Vec3::new(epsilon, 0.0, 0.0);
    let ey = Vec3::new(0.0, epsilon, 0.0);
    let ez = Vec3::new(0.0, 0.0, epsilon);
    let _ = e; // suppress unused warning

    Vec3::new(
        sdf(point + ex) - sdf(point - ex),
        sdf(point + ey) - sdf(point - ey),
        sdf(point + ez) - sdf(point - ez),
    )
    .normalize_or_zero()
}

/// Lambert diffuse shading.
///
/// # Math
///
/// ```text
/// diffuse = clamp(dot(normal, light_dir), 0.0, 1.0)
/// ```
///
/// # Arguments
/// * `normal`    — unit surface normal
/// * `light_dir` — unit direction toward the light source
///
/// # Returns
/// Diffuse intensity in [0, 1].
///
/// # Example
/// ```rust
/// use glam::Vec3;
/// use form_render::light::diffuse;
///
/// // Normal facing light directly → full intensity
/// assert_eq!(diffuse(Vec3::Y, Vec3::Y), 1.0);
/// // Normal facing away → zero
/// assert_eq!(diffuse(Vec3::Y, Vec3::NEG_Y), 0.0);
/// ```
pub fn diffuse(normal: Vec3, light_dir: Vec3) -> f32 {
    normal.dot(light_dir).clamp(0.0, 1.0)
}

/// Hard shadow test — cast a ray from `point` toward the light.
///
/// Returns 0.0 if the point is in shadow (something occludes the light),
/// 1.0 if fully lit.
///
/// # Arguments
/// * `sdf`       — the signed distance function
/// * `point`     — surface point to test (should be offset slightly along normal)
/// * `light_dir` — unit direction toward the light
/// * `settings`  — march settings (max_dist used as light distance)
///
/// # Returns
/// `0.0` = fully in shadow, `1.0` = fully lit.
///
/// # Example
/// ```rust
/// use glam::Vec3;
/// use form_render::light::hard_shadow;
/// use form_render::march::MarchSettings;
///
/// let sdf = |p: Vec3| p.length() - 1.0;
/// // Light is directly above; point on top of sphere — not self-shadowed
/// let point = Vec3::new(0.0, 1.001, 0.0);
/// let lit = hard_shadow(sdf, point, Vec3::Y, MarchSettings::default());
/// assert_eq!(lit, 1.0);
/// ```
pub fn hard_shadow<F>(sdf: F, point: Vec3, light_dir: Vec3, settings: MarchSettings) -> f32
where
    F: Fn(Vec3) -> f32,
{
    let result = march(point, light_dir, sdf, settings);
    if result.hit { 0.0 } else { 1.0 }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn unit_sphere(p: Vec3) -> f32 {
        p.length() - 1.0
    }

    // --- normal_at ---

    #[test]
    fn normal_on_sphere_points_outward() {
        for axis in [Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z] {
            let n = normal_at(unit_sphere, axis, EPSILON);
            assert!(
                (n - axis).length() < 0.01,
                "axis={axis:?} got normal={n:?}"
            );
        }
    }

    #[test]
    fn normal_is_unit_length() {
        let n = normal_at(unit_sphere, Vec3::X, EPSILON);
        assert!((n.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn normal_deterministic() {
        let a = normal_at(unit_sphere, Vec3::X, EPSILON);
        let b = normal_at(unit_sphere, Vec3::X, EPSILON);
        assert_eq!(a, b);
    }

    // --- diffuse ---

    #[test]
    fn diffuse_facing_light_is_one() {
        assert_eq!(diffuse(Vec3::Y, Vec3::Y), 1.0);
    }

    #[test]
    fn diffuse_away_from_light_is_zero() {
        assert_eq!(diffuse(Vec3::Y, Vec3::NEG_Y), 0.0);
    }

    #[test]
    fn diffuse_clamped_non_negative() {
        for angle in [45.0_f32, 90.0, 135.0, 180.0] {
            let light = Vec3::new(angle.to_radians().cos(), angle.to_radians().sin(), 0.0);
            let d = diffuse(Vec3::Y, light);
            assert!(d >= 0.0 && d <= 1.0, "angle={angle} d={d}");
        }
    }

    // --- hard_shadow ---

    #[test]
    fn no_shadow_toward_open_space() {
        let point = Vec3::new(0.0, 1.001, 0.0);
        let lit = hard_shadow(unit_sphere, point, Vec3::Y, MarchSettings::default());
        assert_eq!(lit, 1.0);
    }
}
