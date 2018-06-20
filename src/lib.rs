// "adi_gpu_base" - Aldaron's Device Interface / GPU / Base
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! This library is the base library for implementations of the adi_gpu api.
//! If you would like to make your own implementation of the api, you can use
//! this library as a dependency.

extern crate ami;
extern crate awi;

use std::cmp::Ordering;

pub use awi::{
	afi, afi::Graphic, Input, Window, WindowConnection
};
pub use ami::{ Mat4, Vec3, Vec4, BBox, IDENTITY };

/// A trait for a `Display`
pub trait Display {
	/// Set the background color for the `Display`.
	///
	/// * `color`: The background color for the display.
	fn color(&mut self, color: (f32, f32, f32)) -> ();

	/// Set the fog for the display.
	///
	/// * `fog`: `None` for no fog, otherwise set fog begin distance and fog
	///	end distance.
	fn fog(&mut self, fog: Option<(f32, f32)>) -> ();

	/// Get input, if there's any.  If there's no input, update the
	///`Display` and return `None`.
	fn update(&mut self) -> Option<Input>;

	/// Move the camera.
	///
	/// * `position`: position of the camera.
	/// * `rotation`: rotation of the camera.
	fn camera(&mut self, position: (f32, f32, f32),
		rotation: (f32, f32, f32)) -> ();

	/// Create a new `Model` for this `Display`.
	fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>) -> Model;

	/// Create a new `Texture` for this `Display`.
	fn texture(&mut self, graphic: &Graphic) -> Texture;

	/// Create a new `Gradient` for this `Display`.
	fn gradient(&mut self, colors: &[f32]) -> Gradient;

	/// Create new `TexCoords` for this `Display`.
	fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords;

	/// Set the pixels for a `Texture`.
	fn set_texture(&mut self, texture: &mut Texture, pixels: &[u32])
		-> ();

	/// Create a new shape with a solid color.
	fn shape_solid(&mut self, model: &Model, transform: Mat4,
		color: [f32; 4], blending: bool, fog: bool, camera: bool)
		-> Shape;

	/// Create a new shape shaded by a gradient (1 color per vertex).
	fn shape_gradient(&mut self, model: &Model, transform: Mat4,
		gradient: Gradient, blending: bool, fog: bool, camera: bool)
		-> Shape;

	/// Create a new shape shaded by a texture using texture coordinates.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_texture(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, blending: bool,
		fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and alpha.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_faded(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, alpha: f32,
		fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_tinted(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, tint: [f32; 4],
		blending: bool, fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint per vertex.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_complex(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords,
		gradient: Gradient, blending: bool,
		fog: bool, camera: bool) -> Shape;

	/// Transform the shape.
	fn transform(&mut self, shape: &Shape, transform: Mat4);

	/// Resize the display.
	fn resize(&mut self, wh: (u32, u32)) -> ();

	/// Get the width and height of the window, as a tuple.
	fn wh(&self) -> (u32, u32);
}

/// Handle for shape.
#[derive(Clone)]
pub enum ShapeHandle {
	Alpha(u32),
	Opaque(u32),
	Gui(u32),
}

/// A renderable object that exists on the `Display`.
pub struct Shape(ShapeHandle);

/// A list of vertices that make a shape.
#[derive(Copy, Clone)]
pub struct Model(pub usize); // TODO: unsafe

/// A list of colors to be paired with vertices.
#[derive(Copy, Clone)]
pub struct Gradient(pub usize); // TODO: unsafe

/// A list of texture coordinates to be paired with vertices.
#[derive(Copy, Clone)]
pub struct TexCoords(pub usize); // TODO: unsafe

/// A Texture
pub struct Texture(pub usize); // TODO: unsafe

/// Create a new shape
pub fn new_shape(i: ShapeHandle) -> Shape {
	Shape(i)
}

/// Get the index of a shape
pub fn get_shape(s: &Shape) -> ShapeHandle {
	s.0.clone()
}

/// Generate a projection matrix.
pub fn projection(ratio: f32, fov: f32) -> Mat4 {
	let scale = (fov * 0.5 * ::std::f32::consts::PI / 180.).tan().recip();
	let yscale = scale * ratio;

	Mat4([
		scale,	0.,	0.,	0.,
		0.,	yscale,	0.,	0.,
		0.,	0.,	1.,	1.,
		0.,	0.,	0., 	1.,
	])
}

pub trait Point {
	fn point(&self) -> ami::Vec3;
}

/// Sort by distance.  nr => true if Near Sort, nr => false if Far Sort
pub fn zsort<T: Point>(sorted: &mut Vec<u32>, points: &Vec<T>, nr: bool,
	position: Vec3)
{
	sorted.sort_unstable_by(|a, b| {
		let p1 = points[*a as usize].point() - position;
		let p2 = points[*b as usize].point() - position;

		if p1.mag() > p2.mag() {
			if nr { Ordering::Greater } else { Ordering::Less }
		} else if p1.mag() < p2.mag() {
			if nr { Ordering::Less } else { Ordering::Greater }
		} else {
			Ordering::Equal
		}
	});
}
