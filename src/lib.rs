// Aldaron's Device Interface / GPU / Base
// Copyright (c) 2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
// lib.rs

//! This library is the base library for implementations of the adi_gpu api.
//! If you would like to make your own implementation of the api, you can use
//! this library as a dependency.

extern crate afi;
extern crate ami;
extern crate awi;

use afi::*;
use ami::Mat4;

pub trait BaseTypes {
	type Gradient;
	type Texture;
	type TexCoords;
	type Model;
	type Shape;
}

/// A trait for a `Display`
pub trait Display: Sized {
	type Model;
	type Texture;
	type Gradient;
	type TexCoords;
	type Shape;

	/// Create a new GPU-Accelerated `Display`.  If it can't be created,
	/// return None.
	///
	/// * `window`: The window to make a GPU-Accelerated `Display`.
	fn new(window: &awi::Window) -> Option<Self>;

	/// Set the background color for the `Display`.
	///
	/// * `color`: The background color for the display.
	fn color(&mut self, color: (f32, f32, f32)) -> ();

	/// Set the fog for the display.
	///
	/// * `fog`: `None` for no fog, otherwise set fog begin distance and fog
	///	end distance.
	fn fog(&mut self, fog: Option<(f32, f32)>) -> ();

	/// Update the `Display`.
	fn update(&mut self) -> ();

	/// Move the camera.
	///
	/// * `position`: position of the camera.
	/// * `rotation`: rotation of the camera.
	fn camera(&mut self, position: (f32, f32, f32),
		rotation: (f32, f32, f32)) -> ();

	/// Create a new `Model` for this `Display`.
	fn model(&mut self, vertices: &[f32], indices: &[u32]) -> Self::Model;

	/// Create a new `Texture` for this `Display`.
	fn texture(&mut self, graphic: Graphic) -> Self::Texture;

	/// Create a new `Gradient` for this `Display`.
	fn gradient(&mut self, colors: &[f32]) -> Self::Gradient;

	/// Create new `TexCoords` for this `Display`.
	fn texcoords(&mut self, texcoords: &[f32]) -> Self::TexCoords;

	/// Set the pixels for a `Texture`.
	fn set_texture(&mut self, texture: &mut Self::Texture, pixels: &[u32])
		-> ();

	/// Create a new shape with a solid color.
	fn shape_solid(&mut self, model: &Self::Model, transform: Mat4,
		color: [f32; 4], blending: bool, fancy: bool, fog: bool,
		camera: bool) -> Self::Shape;

	/// Create a new shape shaded by a gradient (1 color per vertex).
	fn shape_gradient(&mut self, model: &Self::Model, transform: Mat4,
		gradient: Self::Gradient, blending: bool, fancy: bool,
		fog: bool, camera: bool) -> Self::Shape;

	/// Create a new shape shaded by a texture using texture coordinates.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_texture(&mut self, model: &Self::Model, transform: Mat4,
		texture: Self::Texture, tc: Self::TexCoords, blending: bool,
		fancy: bool, fog: bool, camera: bool) -> Self::Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and alpha.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_faded(&mut self, model: &Self::Model, transform: Mat4,
		texture: Self::Texture, tc: Self::TexCoords, alpha: f32,
		fancy: bool, fog: bool, camera: bool) -> Self::Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_tinted(&mut self, model: &Self::Model, transform: Mat4,
		texture: Self::Texture, tc: Self::TexCoords, tint: [f32; 4],
		blending: bool, fancy: bool, fog: bool, camera: bool)
		-> Self::Shape;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint per vertex.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn shape_complex(&mut self, model: &Self::Model, transform: Mat4,
		texture: Self::Texture, tc: Self::TexCoords,
		gradient: Self::Gradient, blending: bool, fancy: bool,
		fog: bool, camera: bool) -> Self::Shape;

	/// Transform the shape.
	fn transform(&mut self, shape: &mut Self::Shape, transform: &Mat4);

	/// Resize the display.
	fn resize(&mut self, wh: (u32, u32)) -> ();
}

/// Trait for a `Texture`.
pub trait Texture {
	/// Get the width and height of this `Texture`.
	fn wh(&self) -> (u32, u32);
}

/// Trait for `Model`
pub trait Model {
}

/// Trait for `Shape`
pub trait Shape {
}

/// Trait for `Gradient`
pub trait Gradient {
}

/// Trait for `TexCoords`
pub trait TexCoords {
}
