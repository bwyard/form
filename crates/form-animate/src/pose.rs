//! Pose — pure data output of the animation system.
//!
//! A `Pose` is the evaluated state of a figure at a single instant `t`.
//! It contains no methods that mutate — it is the result of `animate(t)`,
//! not a cursor that moves through time.

/// The animated state of a figure at a single instant in time.
///
/// Every field is derived purely from `t`. No stored state, no keyframes.
/// Two calls to `animate(t)` with the same `t` always produce identical `Pose`s.
///
/// # Fields
///
/// All values are in radians or metres as noted. Coordinate system: right-handed,
/// Y-up. Motion is relative to a neutral standing pose at `t = 0`.
///
/// # Example
/// ```rust
/// use form_animate::Pose;
/// let neutral = Pose::default();
/// assert_eq!(neutral.lateral_sway, 0.0);
/// assert_eq!(neutral.vertical_bob, 0.0);
/// assert_eq!(neutral.sagittal_rotation, 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose {
    /// Lateral (side-to-side) sway offset in metres. Positive = right.
    pub lateral_sway: f32,
    /// Vertical (up-down) bob offset in metres. Positive = up.
    pub vertical_bob: f32,
    /// Sagittal (forward-back) rotation in radians. Positive = forward lean.
    pub sagittal_rotation: f32,
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            lateral_sway: 0.0,
            vertical_bob: 0.0,
            sagittal_rotation: 0.0,
        }
    }
}
