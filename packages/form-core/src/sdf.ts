/**
 * SDF builder functions — pure functions that return SdfNode descriptions.
 *
 * These mirror the Rust API in form-sdf exactly.
 * Same names, same parameters, same semantics.
 *
 * Assembly rule: no STORE, no JUMP.
 * Every function is a pure constructor — takes params, returns a node.
 * No mutation. No side effects.
 *
 * When WASM ships, an evaluator backed by form-sdf WASM will replace
 * the TypeScript evaluator in evaluate.ts. The scene descriptions
 * (these builder calls) never change — the WASM is a drop-in under the same API.
 */

import type { SdfNode, Vec3 } from './types.js'

// ---------------------------------------------------------------------------
// Primitives
// ---------------------------------------------------------------------------

/** Sphere centred at the origin with the given radius. */
export const sphere = (radius: number): SdfNode =>
  ({ kind: 'sphere', radius })

/** Axis-aligned box centred at the origin. `half` is the half-extent on each axis. */
export const box3d = (half: Vec3): SdfNode =>
  ({ kind: 'box3d', half })

/** Vertical capsule centred at the origin. */
export const capsule3d = (radius: number, height: number): SdfNode =>
  ({ kind: 'capsule3d', radius, height })

/** Vertical cylinder centred at the origin. */
export const cylinder = (radius: number, height: number): SdfNode =>
  ({ kind: 'cylinder', radius, height })

/** Torus lying in the XZ plane. `major` = ring radius, `minor` = tube radius. */
export const torus = (major: number, minor: number): SdfNode =>
  ({ kind: 'torus', major, minor })

/** Infinite plane. `normal` must be a unit vector. `offset` shifts along the normal. */
export const plane = (normal: Vec3, offset: number): SdfNode =>
  ({ kind: 'plane', normal, offset })

// ---------------------------------------------------------------------------
// CSG operations
// ---------------------------------------------------------------------------

/** Union of two shapes — the closer surface wins. */
export const union = (a: SdfNode, b: SdfNode): SdfNode =>
  ({ kind: 'union', a, b })

/** Intersection — only the region inside both shapes. */
export const intersection = (a: SdfNode, b: SdfNode): SdfNode =>
  ({ kind: 'intersection', a, b })

/** Subtract `b` from `a`. */
export const subtract = (a: SdfNode, b: SdfNode): SdfNode =>
  ({ kind: 'subtract', a, b })

/** Smooth union with blending factor `k`. Higher k = more blending. */
export const smoothUnion = (a: SdfNode, b: SdfNode, k: number): SdfNode =>
  ({ kind: 'smooth_union', a, b, k })

/** Smooth intersection with blending factor `k`. */
export const smoothIntersection = (a: SdfNode, b: SdfNode, k: number): SdfNode =>
  ({ kind: 'smooth_intersection', a, b, k })

/** Smooth subtract with blending factor `k`. */
export const smoothSubtract = (a: SdfNode, b: SdfNode, k: number): SdfNode =>
  ({ kind: 'smooth_subtract', a, b, k })

// ---------------------------------------------------------------------------
// Domain operations
// ---------------------------------------------------------------------------

/** Translate a shape by an offset vector. */
export const translate = (node: SdfNode, offset: Vec3): SdfNode =>
  ({ kind: 'translate', node, offset })

/** Uniformly scale a shape. */
export const scale = (node: SdfNode, factor: number): SdfNode =>
  ({ kind: 'scale', node, factor })

/** Repeat a shape infinitely with the given period on each axis. */
export const repeat = (node: SdfNode, period: Vec3): SdfNode =>
  ({ kind: 'repeat', node, period })
