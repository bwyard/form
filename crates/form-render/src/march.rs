//! Sphere tracing — the ADVANCE loop for form-render.
//!
//! The ray march is the spatial equivalent of prime-render's time scan:
//!
//! ```text
//! prime-render:  fold over n        → output[n] = f(state_n, t_n)
//! form-render:   fold over steps    → position  = f(state_k, ray_k)
//! ```
//!
//! Assembly rule: no STORE, no JUMP.
//! State is (position, total_distance, hit). Folded forward, never mutated.

use glam::Vec3;

/// Settings for the sphere tracing loop.
///
/// # Fields
/// * `max_steps`    — maximum ray march iterations (typical: 64–256)
/// * `max_dist`     — maximum ray travel distance before miss (typical: 100.0)
/// * `hit_epsilon`  — distance threshold for surface hit (typical: 0.001)
#[derive(Debug, Clone, Copy)]
pub struct MarchSettings {
    pub max_steps:   u32,
    pub max_dist:    f32,
    pub hit_epsilon: f32,
}

impl Default for MarchSettings {
    fn default() -> Self {
        MarchSettings {
            max_steps:   128,
            max_dist:    100.0,
            hit_epsilon: 0.001,
        }
    }
}

/// Result of a sphere tracing march along a ray.
#[derive(Debug, Clone, Copy)]
pub struct MarchResult {
    /// Whether the ray hit a surface within `max_dist` and `max_steps`.
    pub hit: bool,
    /// Final ray position — on or near the surface if `hit`, else the miss point.
    pub position: Vec3,
    /// Total distance travelled along the ray.
    pub distance: f32,
    /// Number of march steps taken.
    pub steps: u32,
}

/// Sphere trace a ray through an SDF scene.
///
/// This is the ADVANCE operation for form-render. The ray marches forward
/// through space, advancing by the distance returned by the SDF at each step.
/// The SDF guarantees no surface lies closer than that distance — so the step
/// is always safe.
///
/// # Math
///
/// ```text
/// State: (position, total_dist, hit)
///
/// LOAD    ← origin, direction, sdf, settings
/// COMPUTE ← d = sdf(position)
/// ADVANCE ← position += direction * d;  total_dist += d
/// APPEND  ← hit = true  if d < hit_epsilon
///           miss = true if total_dist > max_dist
/// ```
///
/// Implemented as a fold over step indices — no mutable loop variable.
///
/// # Arguments
/// * `sdf`       — signed distance function: `Vec3 → f32`
/// * `origin`    — ray start position
/// * `direction` — unit ray direction
/// * `settings`  — march parameters
///
/// # Returns
/// [`MarchResult`] describing the outcome.
///
/// # Example
/// ```rust
/// use glam::Vec3;
/// use form_render::march::{march, MarchSettings};
///
/// // SDF: unit sphere at origin
/// let sdf = |p: Vec3| p.length() - 1.0;
/// let result = march(Vec3::new(0.0, 0.0, -3.0), Vec3::Z, sdf, MarchSettings::default());
/// assert!(result.hit);
/// assert!((result.position.z - (-1.0)).abs() < 0.01);
/// ```
pub fn march<F>(origin: Vec3, direction: Vec3, sdf: F, settings: MarchSettings) -> MarchResult
where
    F: Fn(Vec3) -> f32,
{
    let (pos, dist, steps, hit) = (0..settings.max_steps).fold(
        (origin, 0.0_f32, 0_u32, false),
        |(pos, dist, steps, hit), _| {
            if hit || dist > settings.max_dist {
                (pos, dist, steps, hit)
            } else {
                let d = sdf(pos);
                if d < settings.hit_epsilon {
                    (pos, dist, steps + 1, true)
                } else {
                    (pos + direction * d, dist + d, steps + 1, false)
                }
            }
        },
    );
    MarchResult { hit, position: pos, distance: dist, steps }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SETTINGS: MarchSettings = MarchSettings {
        max_steps:   128,
        max_dist:    100.0,
        hit_epsilon: 0.001,
    };

    fn unit_sphere(p: Vec3) -> f32 {
        p.length() - 1.0
    }

    #[test]
    fn hits_unit_sphere_head_on() {
        let result = march(Vec3::new(0.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        assert!(result.hit);
    }

    #[test]
    fn hit_position_near_surface() {
        let result = march(Vec3::new(0.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        let dist_to_surface = (result.position.length() - 1.0).abs();
        assert!(dist_to_surface < 0.01, "dist_to_surface={dist_to_surface}");
    }

    #[test]
    fn misses_when_ray_avoids_sphere() {
        // Ray pointing away from sphere
        let result = march(Vec3::new(0.0, 0.0, -3.0), Vec3::NEG_Z, unit_sphere, SETTINGS);
        assert!(!result.hit);
    }

    #[test]
    fn misses_when_ray_passes_beside_sphere() {
        // Ray offset far enough to miss
        let result = march(Vec3::new(5.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        assert!(!result.hit);
    }

    #[test]
    fn deterministic() {
        let a = march(Vec3::new(0.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        let b = march(Vec3::new(0.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        assert_eq!(a.hit, b.hit);
        assert_eq!(a.position, b.position);
        assert_eq!(a.distance, b.distance);
        assert_eq!(a.steps, b.steps);
    }

    #[test]
    fn steps_bounded_by_max_steps() {
        let result = march(Vec3::new(0.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        assert!(result.steps <= SETTINGS.max_steps);
    }

    #[test]
    fn miss_distance_near_max_dist() {
        // Ray that misses should travel close to max_dist
        let result = march(Vec3::new(5.0, 0.0, -3.0), Vec3::Z, unit_sphere, SETTINGS);
        assert!(!result.hit);
        assert!(result.distance >= SETTINGS.max_dist - 1.0);
    }

    #[test]
    fn inside_sphere_hits_immediately() {
        // Starting inside — SDF is negative, d < hit_epsilon immediately
        let result = march(Vec3::ZERO, Vec3::Z, unit_sphere, SETTINGS);
        assert!(result.hit);
        assert_eq!(result.steps, 1);
    }
}
