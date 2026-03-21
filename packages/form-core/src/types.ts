/**
 * Core types for form-core.
 *
 * Assembly rule: no STORE, no JUMP.
 * All types are pure data — no methods that mutate.
 * The SDF is the object. It does not exist as state; it exists as a description.
 */

/** A point in 3D space. Immutable tuple. */
export type Vec3 = readonly [x: number, y: number, z: number]

/** A point in 2D space. Immutable tuple. */
export type Vec2 = readonly [x: number, y: number]

/**
 * A signed distance field description — pure data, not a function.
 *
 * An SdfNode describes a shape. It is not the shape itself.
 * The shape is the result of evaluating the node at a point: `evaluate(node, point)`.
 *
 * Scenes are plain TypeScript files that construct SdfNode trees using builder functions.
 * The tree is the score. The evaluator is the render loop.
 */
export type SdfNode =
  // Primitives
  | { readonly kind: 'sphere';   readonly radius: number }
  | { readonly kind: 'box3d';    readonly half: Vec3 }
  | { readonly kind: 'capsule3d'; readonly radius: number; readonly height: number }
  | { readonly kind: 'cylinder'; readonly radius: number; readonly height: number }
  | { readonly kind: 'torus';    readonly major: number;  readonly minor: number }
  | { readonly kind: 'plane';    readonly normal: Vec3;   readonly offset: number }
  // CSG operations
  | { readonly kind: 'union';        readonly a: SdfNode; readonly b: SdfNode }
  | { readonly kind: 'intersection'; readonly a: SdfNode; readonly b: SdfNode }
  | { readonly kind: 'subtract';     readonly a: SdfNode; readonly b: SdfNode }
  | { readonly kind: 'smooth_union';        readonly a: SdfNode; readonly b: SdfNode; readonly k: number }
  | { readonly kind: 'smooth_intersection'; readonly a: SdfNode; readonly b: SdfNode; readonly k: number }
  | { readonly kind: 'smooth_subtract';     readonly a: SdfNode; readonly b: SdfNode; readonly k: number }
  // Domain operations
  | { readonly kind: 'translate'; readonly node: SdfNode; readonly offset: Vec3 }
  | { readonly kind: 'scale';     readonly node: SdfNode; readonly factor: number }
  | { readonly kind: 'repeat';    readonly node: SdfNode; readonly period: Vec3 }

/** A complete scene description. */
export type Scene = {
  readonly root: SdfNode
}
