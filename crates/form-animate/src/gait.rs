//! Gait math — walk cycle derived from pure trigonometry.
//!
//! No stored state. No keyframes. Each function is a pure mapping from time `t`
//! to a scalar offset. The full walk cycle emerges from composing three sinusoids
//! at different phases and frequencies.
//!
//! # Lorenz integration (pending prime-dynamics)
//!
//! The trig-only functions below produce a regular, mechanical gait.
//! Once `prime-dynamics` ships Lorenz + RK4, the gait will be driven by
//! a Lorenz system: x=lateral, y=vertical, z=sagittal. The Lorenz attractor
//! introduces natural irregularity — the human doesn't move mechanically,
//! `t` changes the evaluation chaotically within the attractor's basin.
//!
//! Pattern for Lorenz integration:
//! ```text
//! lorenz_state(t, sigma, rho, beta) -> (x, y, z)
//! lateral_sway      = x * amplitude
//! vertical_bob      = y * amplitude
//! sagittal_rotation = z * amplitude
//! ```

use std::f32::consts::TAU;

/// Lateral (side-to-side) sway as a pure function of time.
///
/// # Math
///
/// ```text
/// lateral_sway(t) = amplitude * sin(TAU * frequency * t)
/// ```
///
/// # Arguments
/// * `t`         — time in seconds
/// * `amplitude` — peak displacement in metres (typical: 0.03–0.08)
/// * `frequency` — stride frequency in Hz (typical: 0.8–1.2 for walking)
///
/// # Returns
/// Lateral offset in metres. Positive = right, negative = left.
///
/// # Example
/// ```rust
/// use form_animate::gait::lateral_sway;
/// // At t=0 the sway is 0 (neutral position)
/// assert_eq!(lateral_sway(0.0, 0.05, 1.0), 0.0);
/// // Quarter period: peak rightward displacement
/// let peak = lateral_sway(0.25, 0.05, 1.0);
/// assert!((peak - 0.05).abs() < 1e-5);
/// ```
pub fn lateral_sway(t: f32, amplitude: f32, frequency: f32) -> f32 {
    amplitude * (TAU * frequency * t).sin()
}

/// Vertical (up-down) bob as a pure function of time.
///
/// # Math
///
/// ```text
/// vertical_bob(t) = amplitude * sin(TAU * 2 * frequency * t)
/// ```
///
/// Runs at double the stride frequency — the body bobs twice per stride
/// (once per step).
///
/// # Arguments
/// * `t`         — time in seconds
/// * `amplitude` — peak displacement in metres (typical: 0.02–0.05)
/// * `frequency` — stride frequency in Hz
///
/// # Returns
/// Vertical offset in metres. Positive = up.
///
/// # Example
/// ```rust
/// use form_animate::gait::vertical_bob;
/// assert_eq!(vertical_bob(0.0, 0.03, 1.0), 0.0);
/// // Peaks at 1/4 of the step period (1/8 of stride)
/// let peak = vertical_bob(0.125, 0.03, 1.0);
/// assert!((peak - 0.03).abs() < 1e-5);
/// ```
pub fn vertical_bob(t: f32, amplitude: f32, frequency: f32) -> f32 {
    amplitude * (TAU * 2.0 * frequency * t).sin()
}

/// Sagittal (forward-back) rotation as a pure function of time.
///
/// # Math
///
/// ```text
/// sagittal_rotation(t) = amplitude * sin(TAU * frequency * t + PI/4)
/// ```
///
/// Phase-shifted by π/4 relative to lateral sway so the forward lean
/// is slightly ahead of the lateral displacement — matching natural gait.
///
/// # Arguments
/// * `t`         — time in seconds
/// * `amplitude` — peak rotation in radians (typical: 0.05–0.12)
/// * `frequency` — stride frequency in Hz
///
/// # Returns
/// Sagittal rotation in radians. Positive = forward lean.
///
/// # Example
/// ```rust
/// use form_animate::gait::sagittal_rotation;
/// use std::f32::consts::FRAC_PI_4;
/// // At t=0, phase offset gives sin(PI/4) ≈ 0.707
/// let v = sagittal_rotation(0.0, 0.08, 1.0);
/// assert!((v - 0.08 * FRAC_PI_4.sin()).abs() < 1e-5);
/// ```
pub fn sagittal_rotation(t: f32, amplitude: f32, frequency: f32) -> f32 {
    use std::f32::consts::FRAC_PI_4;
    amplitude * (TAU * frequency * t + FRAC_PI_4).sin()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    // --- lateral_sway ---

    #[test]
    fn lateral_sway_zero_at_origin() {
        assert_eq!(lateral_sway(0.0, 0.05, 1.0), 0.0);
    }

    #[test]
    fn lateral_sway_peak_at_quarter_period() {
        let peak = lateral_sway(0.25, 0.05, 1.0);
        assert!((peak - 0.05).abs() < EPSILON);
    }

    #[test]
    fn lateral_sway_negative_at_three_quarter_period() {
        let trough = lateral_sway(0.75, 0.05, 1.0);
        assert!((trough + 0.05).abs() < EPSILON);
    }

    #[test]
    fn lateral_sway_deterministic() {
        assert_eq!(lateral_sway(1.23, 0.05, 1.0), lateral_sway(1.23, 0.05, 1.0));
    }

    #[test]
    fn lateral_sway_zero_amplitude() {
        assert_eq!(lateral_sway(0.5, 0.0, 1.0), 0.0);
    }

    #[test]
    fn lateral_sway_bounded_by_amplitude() {
        for i in 0..100 {
            let t = i as f32 * 0.1;
            assert!(lateral_sway(t, 0.05, 1.0).abs() <= 0.05 + EPSILON);
        }
    }

    // --- vertical_bob ---

    #[test]
    fn vertical_bob_zero_at_origin() {
        assert_eq!(vertical_bob(0.0, 0.03, 1.0), 0.0);
    }

    #[test]
    fn vertical_bob_double_frequency() {
        // Peaks at 1/8 stride (double frequency → half period)
        let peak = vertical_bob(0.125, 0.03, 1.0);
        assert!((peak - 0.03).abs() < EPSILON);
    }

    #[test]
    fn vertical_bob_deterministic() {
        assert_eq!(vertical_bob(2.5, 0.03, 1.0), vertical_bob(2.5, 0.03, 1.0));
    }

    #[test]
    fn vertical_bob_bounded_by_amplitude() {
        for i in 0..100 {
            let t = i as f32 * 0.1;
            assert!(vertical_bob(t, 0.03, 1.0).abs() <= 0.03 + EPSILON);
        }
    }

    // --- sagittal_rotation ---

    #[test]
    fn sagittal_rotation_phase_offset_at_origin() {
        use std::f32::consts::FRAC_PI_4;
        let v = sagittal_rotation(0.0, 0.08, 1.0);
        assert!((v - 0.08 * FRAC_PI_4.sin()).abs() < EPSILON);
    }

    #[test]
    fn sagittal_rotation_deterministic() {
        assert_eq!(
            sagittal_rotation(0.77, 0.08, 1.0),
            sagittal_rotation(0.77, 0.08, 1.0)
        );
    }

    #[test]
    fn sagittal_rotation_bounded_by_amplitude() {
        for i in 0..100 {
            let t = i as f32 * 0.1;
            assert!(sagittal_rotation(t, 0.08, 1.0).abs() <= 0.08 + EPSILON);
        }
    }
}
