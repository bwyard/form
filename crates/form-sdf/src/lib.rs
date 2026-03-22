// form-sdf re-exports prime-sdf primitives, CSG ops, and domain transforms,
// then extends them with form-local additions.
pub use prime_sdf::*;

/// Additional 2D primitives not yet in prime-sdf (e.g. isosceles_triangle).
pub mod extras;
pub use extras::*;

/// 2D domain warp functions — pure Vec2 → Vec2 input-space transforms.
/// Compose with any SDF: `sdf(warp::stretch(p, sx, sy))`
pub mod warp;
