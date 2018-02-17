// Aldaron's Device Interface / GPU / Base
// Copyright (c) 2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
// lib.rs

//! This library is the base library for implementations of the adi_gpu api.
//! If you would like to make your own implementation of the api, you can use
//! this library as a dependency.

extern crate afi;
extern crate ami;

use afi::*;
use ami::*;

/// A trait for a `Display`
pub trait Display where Self: Sized {
	/// Create a new GPU-Accelerated `Display`.  If it can't be created,
	/// return None.
	///
	/// * `name`: If `Display` is a window, the window title.
	/// * `graphic`: If `Display` is a window, the window icon.
	fn new(name: &str, graphic: Graphic) -> Option<Self>;

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
	fn model<M>(&mut self, vertices: &[f32], indices: &[u32])
		-> M where M: Model;

	/// Create a new `Texture` for this `Display`.
	fn texture<X>(&mut self, graphic: Graphic) -> X where X: Texture;

	/// Create a new `Gradient` for this `Display`.
	fn gradient<G>(&mut self, colors: &[f32]) -> G where G: Gradient;

	/// Create new `TexCoords` for this `Display`.
	fn texcoords<C>(&mut self, texcoords: &[f32]) -> C where C: TexCoords;
}

/// Trait for a `Texture`.
pub trait Texture {
	/// Set the pixels for this `Texture`.
	fn set<D>(&mut self, display: D, pixels: &[u32]) -> () where D: Display;

	/// Get the width and height of this `Texture`.
	fn wh(&self) -> (u32, u32);
}

/*impl Texture {
	/// Create a new empty texture for this `Display`.
	pub fn empty(display: Display, wh: (u32, u32)) -> Self {
		let graphic = GraphicBuilder::new()
			.rgba(wh.0, wh.1, vec![0; wh.0 * wh.1]);

		display.texture(graphic)
	}
}*/

/// Trait for `Model`
pub trait Model {
	/// Create a new shape with a solid color.
	fn to_solid_shape<T, D, X>(&self, display: &mut D, transform: Mat4,
		color: [f32; 4],
		blending: bool, fancy: bool, fog: bool, camera: bool) -> T
		where T: Shape, D: Display;

	/// Create a new shape shaded by a gradient (1 color per vertex).
	fn to_gradient_shape<T, D, X>(&self, display: &mut D, transform: Mat4,
		gradient: Gradient,
		blending: bool, fancy: bool, fog: bool, camera: bool) -> T
		where T: Shape, D: Display;

	/// Create a new shape shaded by a texture using texture coordinates.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn to_texture_shape<T, D, X>(&self, display: &mut D, transform: Mat4,
		texture: X, tc: TexCoords,
		blending: bool, fancy: bool, fog: bool, camera: bool) -> T
		where T: Shape, D: Display;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and alpha.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn to_faded_shape<T, D, X>(&self, display: &mut D, transform: Mat4,
		texture: X, tc: TexCoords,
		alpha: f32, fancy: bool, fog: bool, camera: bool) -> T
		where T: Shape, D: Display;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn to_tinted_shape<T, D, X>(&self, display: &mut D, transform: Mat4,
		texture: X, tc: TexCoords, tint: [f32; 4],
		blending: bool, fancy: bool, fog: bool, camera: bool) -> T
		where T: Shape, D: Display;

	/// Create a new shape shaded by a texture using texture coordinates
	/// and tint per vertex.
	///
	/// Texture Coordinates follow this format (X, Y, UNUSED(1.0), ALPHA)
	fn to_complex_shape<T, D, X>(&self, display: &mut D, transform: Mat4,
		texture: X, tc: TexCoords, gradient: Gradient,
		blending: bool, fancy: bool, fog: bool, camera: bool) -> T
		where T: Shape, D: Display;
}

/// Trait for `Shape`
pub trait Shape {
	/// Transform the shape.
	fn transform<T>(&mut self, display: &mut T, transform: &Mat4)
		where T: Display;
}

/// Trait for `Gradient`
pub trait Gradient {
}

/// Trait for `TexCoords`
pub trait TexCoords {
}
