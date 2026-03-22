//! 2D domain warp functions — pure input-space transforms.
//!
//! Assembly rule: warps transform the query point `p` before it reaches the SDF.
//! They NEVER touch the SDF output value.
//!
//!   `sdf(warp(p))` — compose freely.
//!   `sdf(warp_a(warp_b(p)))` — stack warps.
//!
//! All functions are pure: `f(p: Vec2, params...) -> Vec2`.

use glam::Vec2;

/// Non-uniform stretch of 2D space.
///
/// Scale x by `sx`, y by `sy`. Use `> 1.0` to expand, `< 1.0` to compress.
///
/// # Example
/// ```rust
/// use glam::Vec2;
/// use form_sdf::warp::stretch;
/// // Double x-axis width, leave y unchanged
/// let q = stretch(Vec2::new(1.0, 1.0), 2.0, 1.0);
/// assert!((q.x - 0.5).abs() < 1e-6);
/// assert!((q.y - 1.0).abs() < 1e-6);
/// ```
pub fn stretch(p: Vec2, sx: f32, sy: f32) -> Vec2 {
    Vec2::new(p.x / sx, p.y / sy)
}

/// Radial pinch — pull space toward the origin.
///
/// Points within `radius` are pulled inward by `strength` (0 = identity, 1 = full collapse).
/// Points outside `radius` are unaffected.
///
/// # Arguments
/// * `p`        - query point
/// * `radius`   - influence radius (> 0)
/// * `strength` - pinch amount, clamped to [0, 1]
pub fn pinch(p: Vec2, radius: f32, strength: f32) -> Vec2 {
    let dist = p.length();
    if dist >= radius || dist < 1e-7 {
        return p;
    }
    let t = 1.0 - (dist / radius).min(1.0);
    let factor = 1.0 - strength * t * t;
    p * factor
}

/// Twist — rotate each point by an angle proportional to its Y position.
///
/// `angle_per_unit` is in radians per world-unit of Y.
///
/// # Example
/// ```rust
/// use glam::Vec2;
/// use form_sdf::warp::twist;
/// // At y=0 the point is unchanged
/// let q = twist(Vec2::new(1.0, 0.0), 1.0);
/// assert!((q - Vec2::new(1.0, 0.0)).length() < 1e-5);
/// ```
pub fn twist(p: Vec2, angle_per_unit: f32) -> Vec2 {
    let angle = p.y * angle_per_unit;
    let (s, c) = angle.sin_cos();
    Vec2::new(c * p.x - s * p.y, s * p.x + c * p.y)
}

/// Radial twist — rotate each point by an angle proportional to its distance from origin.
///
/// Unlike `twist` (which rotates by Y-position), this rotates by radius.
/// Points further from center twist more — corners twist more than edge midpoints.
/// Produces a spiral/whirlpool effect where all edges curve, including horizontal ones.
///
/// # Arguments
/// * `p`        - query point
/// * `strength` - radians of twist per world unit of distance from origin
pub fn radial_twist(p: Vec2, strength: f32) -> Vec2 {
    let angle = p.length() * strength;
    let (s, c) = angle.sin_cos();
    Vec2::new(c * p.x - s * p.y, s * p.x + c * p.y)
}

/// Sinusoidal wave — displace Y by a sine wave based on X position.
///
/// Creates an undulating/ripple effect on horizontal surfaces.
/// Pure displacement: `q = (p.x, p.y + sin(p.x * frequency) * amplitude)`
///
/// # Arguments
/// * `p`         - query point
/// * `frequency` - wave cycles per world unit (higher = tighter waves)
/// * `amplitude` - peak displacement in world units
pub fn wave(p: Vec2, frequency: f32, amplitude: f32) -> Vec2 {
    Vec2::new(p.x, p.y + (p.x * frequency).sin() * amplitude)
}

/// Directional wave — displace along an arbitrary axis.
///
/// Same as `wave` but the displacement direction is configurable.
/// `axis` is the displacement direction (need not be normalised — magnitude scales amplitude).
///
/// # Arguments
/// * `p`         - query point
/// * `frequency` - wave cycles per world unit along X
/// * `amplitude` - peak displacement magnitude
/// * `axis`      - direction of displacement
pub fn wave_dir(p: Vec2, frequency: f32, amplitude: f32, axis: Vec2) -> Vec2 {
    p + axis * (p.x * frequency).sin() * amplitude
}

/// Taper — compress X proportionally as Y increases.
///
/// At `origin_y` the X scale is 1.0; at `origin_y + 1/rate` it reaches 0.
///
/// # Arguments
/// * `p`        - query point
/// * `origin_y` - Y coordinate where taper begins (X is full width here)
/// * `rate`     - compression rate per unit of Y above `origin_y`
pub fn taper_y(p: Vec2, origin_y: f32, rate: f32) -> Vec2 {
    let dy = (p.y - origin_y).max(0.0);
    let scale = (1.0 - dy * rate).max(1e-6);
    Vec2::new(p.x / scale, p.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    // stretch
    #[test]
    fn stretch_identity() {
        let q = stretch(Vec2::new(2.0, 3.0), 1.0, 1.0);
        assert!((q - Vec2::new(2.0, 3.0)).length() < EPSILON);
    }

    #[test]
    fn stretch_doubles_x_range() {
        // stretching by 2 in X compresses the query point by ½
        let q = stretch(Vec2::new(1.0, 0.0), 2.0, 1.0);
        assert!((q.x - 0.5).abs() < EPSILON);
        assert!((q.y - 0.0).abs() < EPSILON);
    }

    #[test]
    fn stretch_y_only() {
        let q = stretch(Vec2::new(1.0, 4.0), 1.0, 2.0);
        assert!((q.x - 1.0).abs() < EPSILON);
        assert!((q.y - 2.0).abs() < EPSILON);
    }

    // pinch
    #[test]
    fn pinch_outside_radius_unchanged() {
        let p = Vec2::new(3.0, 0.0);
        let q = pinch(p, 1.0, 0.9);
        assert!((q - p).length() < EPSILON);
    }

    #[test]
    fn pinch_pulls_inward() {
        let p = Vec2::new(0.5, 0.0); // inside radius=1.0
        let q = pinch(p, 1.0, 0.5);
        // q should be closer to origin than p
        assert!(q.length() < p.length());
    }

    #[test]
    fn pinch_zero_strength_identity() {
        let p = Vec2::new(0.5, 0.3);
        let q = pinch(p, 2.0, 0.0);
        assert!((q - p).length() < EPSILON);
    }

    // twist
    #[test]
    fn twist_zero_y_unchanged() {
        let p = Vec2::new(1.0, 0.0);
        let q = twist(p, std::f32::consts::PI);
        assert!((q - p).length() < EPSILON);
    }

    #[test]
    fn twist_preserves_length() {
        let p = Vec2::new(1.0, 1.0);
        let q = twist(p, 0.5);
        assert!((q.length() - p.length()).abs() < EPSILON);
    }

    // wave
    #[test]
    fn wave_zero_amplitude_identity() {
        let p = Vec2::new(1.0, 2.0);
        let q = wave(p, 3.0, 0.0);
        assert!((q - p).length() < EPSILON);
    }

    #[test]
    fn wave_x_unchanged() {
        let p = Vec2::new(1.0, 0.0);
        let q = wave(p, 1.0, 0.5);
        assert!((q.x - p.x).abs() < EPSILON);
    }

    #[test]
    fn wave_y_displaced_by_sine() {
        // At x=0: sin(0) = 0 → no y displacement
        let p = Vec2::new(0.0, 0.0);
        let q = wave(p, 1.0, 0.5);
        assert!((q - p).length() < EPSILON);
    }

    #[test]
    fn wave_quarter_cycle_peak() {
        // At x = π/2 with frequency=1, sin(π/2)=1 → y displaced by amplitude
        let p = Vec2::new(std::f32::consts::FRAC_PI_2, 0.0);
        let q = wave(p, 1.0, 0.5);
        assert!((q.y - 0.5).abs() < EPSILON, "expected y=0.5, got {}", q.y);
    }

    // wave_dir
    #[test]
    fn wave_dir_zero_amplitude_identity() {
        let p = Vec2::new(1.0, 2.0);
        let q = wave_dir(p, 3.0, 0.0, Vec2::new(0.0, 1.0));
        assert!((q - p).length() < EPSILON);
    }

    #[test]
    fn wave_dir_y_axis_matches_wave() {
        // wave_dir with axis=(0,1) should match wave
        let p = Vec2::new(1.0, 0.5);
        let q1 = wave(p, 2.0, 0.3);
        let q2 = wave_dir(p, 2.0, 0.3, Vec2::new(0.0, 1.0));
        assert!((q1 - q2).length() < EPSILON);
    }

    #[test]
    fn wave_dir_x_axis_displaces_x() {
        // wave_dir with axis=(1,0): displaces x, not y
        let p = Vec2::new(std::f32::consts::FRAC_PI_2, 0.0);
        let q = wave_dir(p, 1.0, 0.5, Vec2::new(1.0, 0.0));
        assert!((q.y - 0.0).abs() < EPSILON, "y should be unchanged");
        assert!((q.x - (p.x + 0.5)).abs() < EPSILON, "x should be displaced by amplitude");
    }

    // taper_y
    #[test]
    fn taper_below_origin_unchanged() {
        let p = Vec2::new(1.0, -1.0);
        let q = taper_y(p, 0.0, 1.0);
        assert!((q - p).length() < EPSILON);
    }

    #[test]
    fn taper_compresses_x_above_origin() {
        // at y=0.5, dy=0.5, scale = 1 - 0.5*1.0 = 0.5 → x/scale = x*2
        let p = Vec2::new(1.0, 0.5);
        let q = taper_y(p, 0.0, 1.0);
        assert!((q.x - 2.0).abs() < EPSILON, "expected x=2.0, got {}", q.x);
        assert!((q.y - 0.5).abs() < EPSILON);
    }
}
