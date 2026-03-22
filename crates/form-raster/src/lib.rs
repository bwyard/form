//! form-raster — 2D SDF flat rasterizer.
//!
//! No ray marching. No camera. No lighting.
//! For each pixel, map to world space, evaluate the SDF, color by sign.
//!
//! This is the simplest possible visual verification of an SDF:
//! inside → white, edge → gray, outside → dark.
//!
//! Used to prove each corruption warp in 2D before extending to 3D.

pub mod raster;
pub use raster::{rasterize, RasterSettings, to_bmp};
