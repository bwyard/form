//! `form-render` — Offline CPU ray marcher. No STORE. No JUMP.
//!
//! This is the MARCH+EVALUATE evaluator for form's spatial assembly thesis.
//! A rendered image is the result of folding a pure step function over space:
//!
//! ```text
//! pixel[x,y] = shade(march(sdf, camera.ray(x,y)))
//! ```
//!
//! The spatial equivalent of prime-render's temporal scan:
//!
//! ```text
//! prime-render:  output[n]   = f(state_n, t_n)          ← fold over time steps
//! form-render:   pixel[x,y]  = f(scene, camera.ray(x,y)) ← fold over pixels + march steps
//! ```
//!
//! # Assembly rule
//!
//! ```text
//! LOAD    ← sdf, camera, light, image dimensions
//! COMPUTE ← march ray through SDF; estimate normal; shade
//! APPEND  ← write RGB to output buffer
//! ADVANCE ← next pixel
//! ```
//!
//! No mutable state. No STORE. No JUMP. Same scene + same camera = same image, always.

pub mod camera;
pub mod image;
pub mod light;
pub mod march;

pub use camera::Camera;
pub use image::{render_image, to_ppm};
pub use light::{diffuse, hard_shadow, normal_at};
pub use march::{march, MarchResult, MarchSettings};
