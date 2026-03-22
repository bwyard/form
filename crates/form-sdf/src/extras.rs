//! Additional 2D primitives not yet in prime-sdf.
//!
//! All functions are pure: `f(p, params...) -> f32`.
//! No state, no side effects. Same input → same output.

use glam::Vec2;

/// Signed distance from point `p` to an isosceles triangle.
///
/// # Math
/// Delegates to the general triangle SDF with vertices computed from
/// `base_center`, `half_width`, and `height`.
///
/// Vertices (CCW winding):
///   a = base_center + (−half_width, 0)
///   b = base_center + ( half_width, 0)
///   c = base_center + (0, height)        ← apex
///
/// # Arguments
/// * `p`           - query point
/// * `base_center` - midpoint of the base edge
/// * `half_width`  - half the base width (> 0)
/// * `height`      - distance from base to apex (> 0 = apex above base)
///
/// # Returns
/// Signed distance: negative inside, positive outside, zero on surface.
///
/// # Example
/// ```rust
/// use glam::Vec2;
/// use form_sdf::isosceles_triangle;
/// // apex at (0, 1), base from (−0.5, 0) to (0.5, 0)
/// let d = isosceles_triangle(Vec2::new(0.0, 0.5), Vec2::ZERO, 0.5, 1.0);
/// assert!(d < 0.0, "centroid should be inside");
/// ```
pub fn isosceles_triangle(p: Vec2, base_center: Vec2, half_width: f32, height: f32) -> f32 {
    let a = base_center + Vec2::new(-half_width, 0.0);
    let b = base_center + Vec2::new( half_width, 0.0);
    let c = base_center + Vec2::new(0.0, height);

    let e0 = b - a;
    let e1 = c - b;
    let e2 = a - c;
    let v0 = p - a;
    let v1 = p - b;
    let v2 = p - c;

    let pq0 = v0 - e0 * (v0.dot(e0) / e0.dot(e0)).clamp(0.0, 1.0);
    let pq1 = v1 - e1 * (v1.dot(e1) / e1.dot(e1)).clamp(0.0, 1.0);
    let pq2 = v2 - e2 * (v2.dot(e2) / e2.dot(e2)).clamp(0.0, 1.0);

    let s = (e0.x * e2.y - e0.y * e2.x).signum();
    let d = (pq0.dot(pq0).min(pq1.dot(pq1)).min(pq2.dot(pq2))).sqrt();
    let inside = (s * (v0.x * e0.y - v0.y * e0.x))
        .min(s * (v1.x * e1.y - v1.y * e1.x))
        .min(s * (v2.x * e2.y - v2.y * e2.x));

    d * (-inside.signum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;
    fn approx_eq(a: f32, b: f32) -> bool { (a - b).abs() < EPSILON }

    // Triangle: base at y=0 from x=−0.5 to x=0.5, apex at (0, 1.0)
    fn spike(p: Vec2) -> f32 { isosceles_triangle(p, Vec2::ZERO, 0.5, 1.0) }

    // T1 — math correctness
    #[test]
    fn apex_on_surface() {
        // apex is (0, 1.0) — on the boundary → d ≈ 0
        let d = spike(Vec2::new(0.0, 1.0));
        assert!(approx_eq(d, 0.0), "apex should be on surface, got {d}");
    }

    #[test]
    fn base_midpoint_on_surface() {
        // base midpoint is (0, 0) — on the base edge → d ≈ 0
        let d = spike(Vec2::ZERO);
        assert!(approx_eq(d, 0.0), "base midpoint should be on surface, got {d}");
    }

    #[test]
    fn centroid_inside() {
        // centroid ≈ (0, 1/3)
        let d = spike(Vec2::new(0.0, 0.33));
        assert!(d < 0.0, "centroid should be inside, got {d}");
    }

    #[test]
    fn far_above_outside() {
        let d = spike(Vec2::new(0.0, 2.0));
        assert!(d > 0.0, "far above apex should be outside, got {d}");
    }

    #[test]
    fn far_beside_outside() {
        let d = spike(Vec2::new(2.0, 0.5));
        assert!(d > 0.0, "far beside should be outside, got {d}");
    }

    #[test]
    fn left_base_corner_near_zero() {
        // left base corner (−0.5, 0) is a vertex → d = 0
        let d = spike(Vec2::new(-0.5, 0.0));
        assert!(approx_eq(d, 0.0), "left corner should be on surface, got {d}");
    }

    #[test]
    fn right_base_corner_near_zero() {
        let d = spike(Vec2::new(0.5, 0.0));
        assert!(approx_eq(d, 0.0), "right corner should be on surface, got {d}");
    }

}
