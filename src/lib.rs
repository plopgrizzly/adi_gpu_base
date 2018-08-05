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

extern crate cgmath;
extern crate euler;
extern crate awi;

use std::cmp::Ordering;

pub use awi::{
	afi, afi::VFrame, Input, Window, WindowConnection
};
pub use euler::*;
pub use std::f32::consts::PI;

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
	fn camera(&mut self, position: Vec3, rotation: Vec3) -> ();

	/// Create a new `Model` for this `Display`.
	fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>) -> Model;

	/// Create a new `Texture` for this `Display`.
	fn texture(&mut self, wh: (u16,u16), graphic: &VFrame) -> Texture;

	/// Create a new `Gradient` for this `Display`.
	fn gradient(&mut self, colors: &[f32]) -> Gradient;

	/// Create new `TexCoords` for this `Display`.
	fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords;

	/// Set the pixels for a `Texture`.
	fn set_texture(&mut self, texture: &mut Texture, wh: (u16,u16),
		graphic: &VFrame) -> ();

	/// Create a new shape with a solid color.
	fn shape_solid(&mut self, model: &Model, transform: Transform,
		color: [f32; 4], blending: bool, fog: bool, camera: bool)
		-> Shape;

	/// Create a new shape shaded by a gradient (1 color per vertex).
	fn shape_gradient(&mut self, model: &Model, transform: Transform,
		gradient: Gradient, blending: bool, fog: bool, camera: bool)
		-> Shape;

	/// Create a new shape shaded by a texture using texture coordinates.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_texture(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, blending: bool,
		fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and alpha.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_faded(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, alpha: f32,
		fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_tinted(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, tint: [f32; 4],
		blending: bool, fog: bool, camera: bool) -> Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint per vertex.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_complex(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords,
		gradient: Gradient, blending: bool,
		fog: bool, camera: bool) -> Shape;

	/// Drop a shape (don't draw it anymore).
	fn drop_shape(&mut self, shape: &Shape);

	/// Transform the shape.
	fn transform(&mut self, shape: &Shape, transform: Transform);

	/// Resize the display.
	fn resize(&mut self, wh: (u16, u16)) -> ();

	/// Get the width and height of the window, as a tuple.
	fn wh(&self) -> (u16, u16);
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
pub struct Texture(pub usize, pub u16, pub u16); // TODO: unsafe

/// Create a new shape
pub fn new_shape(i: ShapeHandle) -> Shape {
	Shape(i)
}

/// Get the index of a shape
pub fn get_shape(s: &Shape) -> ShapeHandle {
	s.0.clone()
}

/// Generate a projection matrix.
pub fn projection(ratiox: f32, fovy: f32) -> Transform {
	let a: [[f32;4];4] = cgmath::perspective(
		cgmath::Rad(fovy),
	// TODO: euler crate have this at some point hopefully?
//	let a: [[f32;4];4] = euler::Mat4::finite_perspective_projection(
//		fovy,
		ratiox,
		0.1, // Near
		100.0, // Far
	).into();
	Transform::IDENTITY
		.m(mat4!(
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, -1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		))
		.m(Mat4::from(a))
}

pub trait Point {
	fn point(&self) -> Vec3;
}

/// Sort by distance.  nr => true if Near Sort, nr => false if Far Sort
pub fn zsort<T: Point>(sorted: &mut Vec<u32>, points: &Vec<T>, nr: bool,
	position: Vec3)
{
	sorted.sort_unstable_by(|a, b| {
		let p1 = points[*a as usize].point() - position;
		let p2 = points[*b as usize].point() - position;

		if p1.length() > p2.length() {
			if nr { Ordering::Greater } else { Ordering::Less }
		} else if p1.length() < p2.length() {
			if nr { Ordering::Less } else { Ordering::Greater }
		} else {
			Ordering::Equal
		}
	});
}

/// A transformation matrix.
#[derive(Copy, Clone)]
pub struct Transform(pub Mat4);

impl Transform {
	/// A constant for the identity matrix.
	pub const IDENTITY: Transform = Transform(Mat4 {
		m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
		m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
		m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
		m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
	});

	/// Scale, then rotate (x: yaw, y: pitch, z: roll), then translate.
	#[inline(always)]
	pub fn srt(self, scale: Vec3, rotate: Vec3, translate: Vec3) -> Self {
		self.s(scale).r(rotate).t(translate)
	}

	/// Rotate (x: yaw, y: pitch, z: roll), then translate.
	#[inline(always)]
	pub fn rt(self, rotate: Vec3, translate: Vec3) -> Transform {
		self.r(rotate).t(translate)
	}

	/// Scale, then translate.
	#[inline(always)]
	pub fn st(self, scale: Vec3, translate: Vec3) -> Transform {
		self.s(scale).t(translate)
	}

	/// Translate.
	#[inline(always)]
	pub fn t(self, translate: Vec3) -> Transform {
		self.m(Trs::new(
			translate, // Translation
			quat!(), // No Rotation
			vec3!(1.0, 1.0, 1.0), // No scaling
		).matrix())
	}

	/// Scale.
	#[inline(always)]
	pub fn s(self, scale: Vec3) -> Transform {
		self.m(Trs::new(
			vec3!(0.0, 0.0, 0.0), // No translation
			quat!(), // No Rotation
			scale, // Scaling
		).matrix())
	}

	/// Rotate (x: yaw, y: pitch, z: roll).
	#[inline(always)]
	pub fn r(self, rotate: Vec3) -> Transform {
		self.m(Trs::new(
			vec3!(0.0, 0.0, 0.0), // No translation
			Quat::euler(rotate), // Rotation
			vec3!(1.0, 1.0, 1.0), // No scaling
		).matrix())
	}

	/// Multiply by a custom matrix
	#[inline(always)]
	pub fn m(self, matrix: Mat4) -> Transform {
		Transform(matrix * self.0)
	}

/*	/// Multiply by a projection that scales width and height by the
	/// smallest widget size. The widget is put at position pos. Position
	/// isn't affected by aspect ratio.
	#[inline(always)]
	pub fn auto(self, window: &mut Window, pos: (f32, f32)) -> Transform {
		let size = window.unit_size();
		self.ts(vec3!(pos.0, pos.1, 0.0), vec3!(size.0, size.1, 1.0))
	}*/
}

impl Into<[f32;16]> for Transform {
	fn into(self) -> [f32; 16] {
		let matrix = self.0;
		[
			matrix.m00, matrix.m01, matrix.m02, matrix.m03,
			matrix.m10, matrix.m11, matrix.m12, matrix.m13,
			matrix.m20, matrix.m21, matrix.m22, matrix.m23,
			matrix.m30, matrix.m31, matrix.m32, matrix.m33,
		]
	}
}
