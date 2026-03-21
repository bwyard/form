# FORM — Roadmap

Math-first procedural graphics framework. SDFs, not meshes. A scene is a pure function of space.

Last updated: 2026-03-21

---

## Thesis

**Math thesis:** Creative outputs are mathematical objects, not processes.

Form proves this for the visual domain:

> `shape = f(description, position)` — a scene is a pure function of space.

No mesh files. No geometry tables. No state. Given the same description and the same point in space, the same value comes back every time. The SDF is the object.

This is one of four simultaneous proofs across the projects:

| Project | Domain | Claim |
|---|---|---|
| Prime | Foundation | Pure functions exist with no domain — computation without side effects |
| Score | Music | `output[n] = f(song, n, sr)` — composition is a pure function of time |
| **Form** | **Visual art** | **`shape = f(description, position)` — scene is a pure function of space** |
| Stage | Experience | `experience = f(world, t, pos)` — experience is a pure function of state and time |

All four together prove that mutation is not required — only a scan loop and explicit state threading.

### No STORE. No JUMP.

Score's assembly rule is: no STORE, no JUMP, only APPEND+ADVANCE. The sample loop advances time and evaluates. Nothing is cached or branched to.

Form follows the same rule in the spatial domain:

- **No STORE** — no mesh files, no geometry tables, no stored normals, no keyframes
- **No JUMP** — the ray marcher does not branch to precomputed data
- **MARCH+EVALUATE** — advance position along the ray, evaluate the SDF at each step

| | Score | Form |
|---|---|---|
| Axis | Time (`n`) | Space (`p`) |
| Scan loop | Sample loop | Ray march |
| Rule | APPEND+ADVANCE | MARCH+EVALUATE |
| Pure function | `f(song, n, sr)` | `f(description, p)` |
| No STORE | No audio buffers in DSL layer | No mesh, no stored geometry |
| No JUMP | No conditional branches to cached samples | No lookup tables, no precomputed normals |

### One model, four axis types

At the assembly level APPEND+ADVANCE and MARCH+EVALUATE are the same four instructions:

```
LOAD    cursor          // read current position on the axis
EVAL    f(cursor)       // pure computation — no side effects
APPEND  result          // emit the output
ADVANCE cursor          // move forward
```

The axis type is the only variable:

| Project | Cursor type | Axis |
|---|---|---|
| Score | `n: u64` | time (sample index) |
| Form | `p: Vec3` | space (position along ray) |
| Stage | `(world, t, pos)` | state-time |

---

## Corruption architecture

The corruption model is Form's core conceptual contribution.

### What it is

Every scene has two layers:

1. **Ground state** — mathematical truth. A sphere. A box. A plane. Defined exactly by its SDF.
2. **Corruption stack** — a sequence of pure domain warp functions applied to the input point before SDF evaluation. Each one adds biological or physical irregularity.

The key: corruption warps the **input point**, not the **SDF output value**. `corrupt(p) -> p'`, then `sdf(p')`. This preserves SDF validity — the field stays well-formed.

The output is `sdf(corrupt(p))` — still a pure function. The corruption stack is not mutation. It is composition.

### Corruption as proof strategy

The corruption model is Form's proof strategy, the same role the scan loop plays in Score.

Score answers "surely audio needs mutable buffers" with the sample scan.
Form answers "surely geometry needs stored data" with the corruption stack.

**The claim:** organic complexity is not evidence that you need stored data. Biological shapes look complex. Complexity emerges from composing pure functions — not from storing data.

### Two separate proofs — kept deliberately separate

**Demo A — static corruption** (`head(p) -> f32`, no time parameter)
- Ground state: sphere
- Corruption stack: a sequence of domain warps applied one at a time
- Proves: organic complexity = mathematical composition of pure functions
- No animation. No time. Pure space.

**Demo B — pure animation** (`figure(p, t) -> f32`, simple body)
- Ground state: simple capsule body (no corruption)
- Time parameter drives gait: lateral sway, vertical bob, sagittal rotation
- Proves: motion = pure function of time, no keyframes, no stored poses
- Comes after Demo A. Animation is introduced only after static corruption is proven.

These two proofs are composed later (Phase 8 engine integration) into a corrupted running figure. Combining them prematurely would obscure both arguments.

---

## Testing philosophy

Every phase has three test layers. Nothing moves to the next phase until all three pass.

### T1 — Math tests (automated, no rendering)
Pure unit tests. Run with `cargo test`. No BMP files, no visual inspection needed.
- SDF values at known geometric points: inside → negative, outside → positive, surface → ≈ 0
- Domain warp direction and magnitude: each warp moves points in the expected direction, by the expected amount, and leaves points outside its region unchanged
- CSG correctness: union ≤ min(d1,d2), intersection ≥ max(d1,d2), subtract carves correctly
- Smooth ops: blend region returns value between the two inputs, far from blend returns correct input

### T2 — Render tests (automated, small images)
Small renders (11×11 or 16×16 pixels) with pixel-level assertions. No BMP inspection needed.
- Hit test: center pixel of a centered shape is not sky-colored
- Miss test: corner pixels that should miss are sky-dominant (B > R)
- Symmetry test: symmetric SDF produces symmetric pixel output (left half ≈ right half)
- Brightness test: lit surface pixels are brighter than ambient minimum

### T3 — Visual confirmation (user opens BMP, I write expected output)
Used only for proportion and aesthetic sign-off, not for debugging. I write an exact "Expected:" description before every render. The user opens the BMP and confirms or reports the difference. We do not proceed to the next step until T3 passes.

---

## Milestones

| Milestone | Status |
|---|---|
| Phase 0 — form-sdf | ✅ Done — 40/40 tests |
| Phase 1 — form-render | ✅ Core done — T2 render tests gap to fill |
| Phase 2 — Primitive gallery | Next |
| Phase 3 — Demo A: Corruption stack (10 steps) | Blocked on Phase 2 |
| Phase 4 — Demo B: Gait animation (5 steps) | Blocked on Phase 3 |
| Phase 5 — form-noise | Blocked on prime-noise 3D + simplex |
| Phase 6 — TypeScript API | Future |
| Phase 7 — Score bridge | Future |
| Phase 8 — Bevy integration | KEY MILESTONE |
| Phase 9 — Release | Future |

---

## PRIME dependency map

| Form phase | PRIME dependency | Status |
|---|---|---|
| Phase 0 (form-sdf) | prime-sdf | ✅ Done |
| Phase 5 (form-noise) | prime-noise (3D, simplex, FBM) | Waiting on prime |
| Phase 4 (form-animate Lorenz) | prime-dynamics (Lorenz, RK4) | Waiting on prime |
| Phase 6 (form-core) | prime-interp (spline/easing) | TBD |

---

## Phase detail

---

### Phase 0 — form-sdf ✅ Done (2026-03-18)

**Crate: `form-sdf`. Dependency: `prime-sdf` (thin re-export). 40/40 tests.**

- 2D primitives: circle, box_2d, rounded_box, capsule_2d, line_segment, triangle, ring
- 3D primitives: sphere, box_3d, capsule_3d, cylinder, torus, plane
- CSG: union, intersection, subtract, xor, smooth_union, smooth_intersection, smooth_subtract
- Domain: translate, rotate_2d, scale, repeat, mirror_x, mirror_y, elongate

---

### Phase 1 — form-render ✅ Core done — gap work needed

**Crate: `form-render`. No external PRIME dependency.**

Core components verified correct:
- [x] Sphere tracing fold (`march.rs`) — 8 tests
- [x] Normal estimation, Lambert diffuse, hard shadow (`light.rs`) — 5 tests
- [x] Camera look-at, ray generation (`camera.rs`) — 4 tests
- [x] Pixel fold, BMP output, PPM output (`image.rs`) — 5 tests
- [x] Sphere example renders correctly (T3 confirmed)

**Gap — T2 render tests to add before Phase 2:**
- [ ] Sky pixel is blue-dominant: corner pixels have B > R
- [ ] Hit pixel is not sky: center pixel of sphere render is brighter than sky
- [ ] Symmetric SDF produces symmetric pixels: left half ≈ right half
- [ ] BMP file dimensions are correct for given width/height

Done criteria: all gap tests pass, `cargo test -p form-render` green.

---

### Phase 2 — Primitive gallery

**One example + test block per SDF primitive. Confirms renderer handles every shape.**

For each primitive:
- T1: inside/outside/surface SDF value tests
- T2: center hits, corners miss, symmetry where applicable
- T3: written expected output, user confirms

Primitives:

| Example | T3 Expected |
|---|---|
| `ex_sphere` | White sphere, blue sky gradient at edges |
| `ex_box` | Cube with sharp edges and flat faces |
| `ex_capsule` | Vertical cylinder with rounded top and bottom caps |
| `ex_torus` | Donut ring, hole visible through center from camera angle |
| `ex_cylinder` | Flat-topped cylinder, sharp top/bottom edges |
| `ex_union` | Two spheres merged into a peanut shape |
| `ex_subtract` | Sphere with a rectangular hole carved into it |
| `ex_smooth_union` | Two spheres with a soft organic blend seam |
| `ex_smooth_subtract` | Sphere with a smooth rounded dent |

Done criteria: all 9 examples render correctly, T1+T2+T3 pass for each.

---

### Phase 3 — Demo A: Corruption stack (10 steps)

**Proves: organic complexity = mathematical composition of pure functions.**

`head(p: Vec3) -> f32` — no time parameter. Pure spatial SDF.

Each step is a separate example file and a separate test block. Every new warp function has T1 tests written before the example is rendered. T3 confirmation required before moving to the next step.

---

#### Step 0 — Bare sphere (baseline)

```rust
fn head_s0(p: Vec3) -> f32 { p.length() - 1.0 }
```

T1:
- `head_s0(Vec3::ZERO) < 0.0` (inside)
- `head_s0(Vec3::new(2.0, 0.0, 0.0)) > 0.0` (outside)
- `head_s0(Vec3::X).abs() < 0.001` (on surface)

T2: center hits, corners miss, left half == right half (symmetric)

T3 Expected: white sphere filling roughly 60% of frame, blue sky at edges, light from upper right

---

#### Step 1 — jaw_warp

Widens the lower face by scaling x outward below the equator.

```rust
fn jaw_warp(p: Vec3) -> Vec3
```

T1:
- Lower face point (x>0, y<0): `jaw_warp(p).x > p.x` (pushed outward)
- Top of head (y>0.5): `jaw_warp(p) ≈ p` (identity — not in jaw region)
- SDF at `(0.9, -0.3, 0)` is more negative after warp than bare sphere (surface further out)

T2: center hits, corners miss

T3 Expected: sphere noticeably wider at the bottom half, tapers at top — egg-like but inverted

---

#### Step 2 — asym_warp

Tiny bilateral asymmetry — models natural skull drift between hemispheres.

```rust
fn asym_warp(p: Vec3) -> Vec3
```

T1:
- `asym_warp(Vec3::new(0.5, 0.0, 0.0)).y != asym_warp(Vec3::new(-0.5, 0.0, 0.0)).y`
- Drift magnitude < 0.02 (small perturbation, not deformation)

T2: center hits, corners miss
T2: left half pixel sum ≠ right half (asymmetry visible in output)

T3 Expected: sphere with barely visible left/right imbalance — same overall shape but not perfectly symmetric

---

#### Step 3 — occipital_warp

Flattens the back of the skull slightly.

```rust
fn occipital_warp(p: Vec3) -> Vec3
```

T1:
- `occipital_warp(Vec3::new(0.0, 0.0, 0.8)).z < 0.8` (back pushed inward)
- `occipital_warp(Vec3::new(0.0, 0.0, -0.8)) ≈ Vec3::new(0.0, 0.0, -0.8)` (front unchanged)

T2: center hits, corners miss

T3 Expected: sphere with slightly flattened back face — difficult to see from front camera, subtle silhouette change

---

#### Step 4 — brow_warp

Pushes the forehead forward in a horizontal band at y ≈ 0.35, front hemisphere only.

```rust
fn brow_warp(p: Vec3) -> Vec3
```

T1:
- `brow_warp(Vec3::new(0.0, 0.35, -0.8)).z < -0.8` (forehead pushed forward/further -z)
- `brow_warp(Vec3::new(0.0, -0.5, -0.8)) ≈ Vec3::new(0.0, -0.5, -0.8)` (chin unchanged)
- `brow_warp(Vec3::new(0.0, 0.35, 0.8)) ≈ Vec3::new(0.0, 0.35, 0.8)` (back hemisphere unchanged)

T2: center hits, corners miss

T3 Expected: subtle forward protrusion visible across the upper forehead — a slight shelf above the mid-face

---

#### Step 5 — composed warps

All four warps applied in sequence: `brow_warp(occipital_warp(jaw_warp(asym_warp(p))))`.

T1:
- Composed SDF still valid: inside origin < 0, outside (2,0,0) > 0
- All individual warp T1 assertions still pass when applied through composition

T2: center hits, corners miss

T3 Expected: sphere with a subtle head-like deformation — wider jaw, slight brow, flat back, tiny asymmetry. Not clearly a head yet but no longer a perfect sphere.

---

#### Step 6 — brow ridge (torus)

A torus-shaped bulge across the forehead, front hemisphere only. Smooth-unioned with the head.

T1:
- Torus SDF is negative near `(0.0, 0.35, -1.0)` (on the brow)
- Torus SDF is positive far from brow band `(0.0, -0.5, -1.0)`
- `smooth_union(head, brow)` returns a value ≤ both inputs in the blend region
- `smooth_union(head, brow)` returns ≈ head value where brow is far away

T2: center hits, brightness variation visible (ridge lighter than surrounding)

T3 Expected: a raised horizontal band visible across the upper forehead — like a brow bone

---

#### Step 7 — temporal hollows

Two small spheres subtracted from the sides of the head at the temples.

T1:
- Hollow SDF is negative inside the hollow sphere position
- After subtraction, SDF at hollow center is positive (region carved out)
- Head SDF at origin unchanged (hollow doesn't reach the center)

T2: center hits, shape still present

T3 Expected: subtle inward dents visible at left and right sides of the head where the temples are

---

#### Step 8 — micro surface noise

Sin-hash noise added to final SDF value. Amplitude 0.003 — sub-millimetre scale.

T1:
- `micro_surface(p)` returns value in `[-0.003, 0.003]`
- `micro_surface(p) != micro_surface(p + Vec3::X * 0.01)` (spatially varying)
- `micro_surface(p) != micro_surface(p + Vec3::Y * 0.01)` (varies in y too)

T2: center still hits (noise too small to break march)

T3 Expected: very faint surface grain — like skin texture at high resolution. Almost invisible at this scale.

*Note: this step will be replaced in Phase 5 when prime-noise ships FBM.*

---

#### Step 9 — neck

An elongated sphere below the head, smooth-unioned with blend.

T1:
- Neck SDF is negative at a point clearly inside the neck position
- Neck SDF is positive at origin (inside head, outside neck)
- `smooth_union(head, neck)` < `head` at the junction region (blend pulls surface inward)

T2: center hits, rendered shape is taller than bare sphere

T3 Expected: head sitting on a short cylindrical neck stub, with a smooth organic blend where they connect — no sharp seam

---

#### Step 10 — Demo A complete

All steps composed. Final `head(p: Vec3) -> f32`.

T1: all individual step T1 tests pass on the final composed function
T2: center hits, shape clearly non-spherical
T3 Expected: a recognisable simplified head shape — reads as a human head, not a sphere. Wider jaw, brow shelf, neck below, slight asymmetry.

**Demo A done: organic complexity proven as mathematical composition.**

---

### Phase 4 — Demo B: Gait animation (5 steps)

**Proves: motion = pure function of time. No keyframes. No stored poses.**

`figure(p: Vec3, t: f32) -> f32` — simple capsule body. Corruption architecture is not present here. The body is intentionally minimal — the point is the gait math, not the anatomy.

Animation comes after corruption is proven. These are independent claims.

---

#### Step 0 — Gait math baseline

Extend existing 13 tests:

T1:
- `lateral_sway(0.0, amp, freq) = 0.0`
- `lateral_sway(0.25, amp, freq) = amp` (peak at quarter period)
- `lateral_sway(0.75, amp, freq) = -amp` (trough at three-quarter)
- `vertical_bob(0.0, amp, freq) = 0.0`
- `vertical_bob(0.125, amp, freq) = amp` (double frequency peak)
- All three functions bounded by amplitude for all t in [0.0, 4.0]
- All three deterministic

---

#### Step 1 — sphere + lateral_sway

Render three BMPs: t=0.0, t=0.25, t=0.5.

T1: sway values at t=0, 0.25, 0.5 are 0, +amplitude, 0

T2:
- t=0.0: center pixel hits (sphere centered)
- t=0.25: center pixel misses or dims (sphere shifted right)
- t=0.5: center pixel hits again (sphere back at center)

T3:
- t=0.0: "sphere centered in frame"
- t=0.25: "sphere shifted to the right"
- t=0.5: "sphere back at center"

---

#### Step 2 — + vertical_bob

Render t=0.0, t=0.125, t=0.25.

T1: bob values correct at each t

T3:
- t=0.0: "sphere centered"
- t=0.125: "sphere shifted upward"
- t=0.25: "sphere back near center"

---

#### Step 3 — + sagittal_rotation

Rotation affects depth (z) — less visible from front-facing camera.

T1: sagittal values correct, non-zero at t=0 due to phase offset

T3: "subtle depth shift — sphere appears to shift slightly in z across t sequence"

---

#### Step 4 — capsule body + full gait

Simple vertical capsule replacing the sphere. All three gait components active.

T2: across t=0, 0.25, 0.5, 0.75 — center hits in some frames as body moves

T3: "capsule shape traces a walking pattern — shifts side to side and bobs up/down. Motion is clearly cyclic and driven by time alone."

---

#### Step 5 — Demo B complete

T3: "a simple body shape animated purely by trig functions. No keyframes. Motion emerges from math."

**Demo B done: motion proven as a pure function of time.**

---

### Phase 5 — form-noise (Blocked)

**Blocked on: prime-noise 3D + simplex + FBM.**

When unblocked:
- Replace sin-hash `micro_surface` in Demo A Step 8 with FBM from prime-noise
- T1: noise values in expected range, spatially varying, no clipping
- T2: noisy sphere surface reads differently from smooth sphere at same resolution
- T3: "visible organic surface texture — skin-like grain replaces sin-hash approximation"

---

### Phase 6 — TypeScript API

**Packages: `form-core`, `form-cli`.**

Scene files as plain TypeScript. Same philosophy as SCORE songs.

- `form-core`: sphere(), union(), translate() builder API — mirrors Rust API
- `form-cli`: `form dev` (hot reload), `form build` (compile), `form render` (offline)

---

### Phase 7 — Score bridge

**Package: `form-score-bridge`.**

Audio parameters → scene parameters.
- BPM → gait frequency
- Amplitude → corruption intensity
- Beat events → warp parameter changes

---

### Phase 8 — Bevy integration (KEY MILESTONE)

**Crate: `form-bevy`.**

Real-time GPU sphere tracing via Bevy render graph.
Demo A + Demo B running live. The corrupted running figure — the full proof.

---

### Phase 9 — Release

- Publish `form-sdf`, `form-noise`, `form-animate` to crates.io
- Docs site (mdBook)
- GitHub README with visual demos
- Blog post: "A human head in 50 lines of math"
