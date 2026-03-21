//! form-animate — Procedural animation as pure functions of time.
//!
//! # Assembly rule
//!
//! No STORE. No JUMP. Every public function is LOAD + COMPUTE only.
//! `animate(t) -> Pose` is the complete formal statement.
//! Same `t`, same `Pose`, every time.
//!
//! # Modules
//!
//! - `pose`  — the `Pose` type (pure data, no methods that mutate)
//! - `gait`  — walk cycle math from pure trig
//! - `lorenz` — chaos-driven gait (pending prime-dynamics)
//! - `ik`    — inverse kinematics (pending prime-dynamics splines)
//! - `spring` — spring/damper secondary motion (pending prime-dynamics)

pub mod gait;
pub mod pose;

pub use pose::Pose;

/// Evaluate the full animation at time `t`.
///
/// # Math
///
/// ```text
/// animate(t) = Pose {
///   lateral_sway:      gait::lateral_sway(t, ...)
///   vertical_bob:      gait::vertical_bob(t, ...)
///   sagittal_rotation: gait::sagittal_rotation(t, ...)
/// }
/// ```
///
/// # Arguments
/// * `t` — time in seconds (any finite `f32`)
///
/// # Returns
/// A [`Pose`] fully derived from `t`. No stored state.
///
/// # Example
/// ```rust
/// use form_animate::animate;
/// let pose = animate(0.0);
/// assert_eq!(pose.lateral_sway, 0.0);
/// ```
pub fn animate(t: f32) -> Pose {
    Pose {
        lateral_sway: gait::lateral_sway(t, 0.05, 1.0),
        vertical_bob: gait::vertical_bob(t, 0.03, 1.0),
        sagittal_rotation: gait::sagittal_rotation(t, 0.08, 1.0),
    }
}
