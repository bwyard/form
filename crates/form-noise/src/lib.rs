// form-noise is a thin re-export of prime-noise.
// All noise implementations live in prime/crates/prime-noise.
pub use prime_noise::*;

// ---------------------------------------------------------------------------
// WASM bindings
// ---------------------------------------------------------------------------
// Explicit wrappers required — wasm-bindgen cannot directly export re-exports.
// Each function mirrors the prime-noise signature exactly.
// When prime-noise gains 3D functions and simplex, add corresponding entries here.

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn value_noise_2d(x: f32, y: f32) -> f32 {
        prime_noise::value_noise_2d(x, y)
    }

    #[wasm_bindgen]
    pub fn perlin_2d(x: f32, y: f32) -> f32 {
        prime_noise::perlin_2d(x, y)
    }

    #[wasm_bindgen]
    pub fn fbm_2d(x: f32, y: f32, octaves: u32, lacunarity: f32, gain: f32) -> f32 {
        prime_noise::fbm_2d(x, y, octaves, lacunarity, gain)
    }

    #[wasm_bindgen]
    pub fn worley_2d(x: f32, y: f32, seed: u32) -> f32 {
        prime_noise::worley_2d(x, y, seed)
    }
}
