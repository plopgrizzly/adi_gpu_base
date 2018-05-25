// "adi_gpu_base" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//
//! This library is the base library for implementations of the adi_gpu api.
//! If you would like to make your own implementation of the api, you can use
//! this library as a dependency.

extern crate ami;
extern crate awi;

pub use awi::{
	afi, afi::Graphic, Input, Window, WindowConnection
};
pub use ami::{ Mat4, Vec3, Vec4, BBox };

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
	fn transform(&mut self, shape: &mut Shape, transform: Mat4);

	/// Check for collision before translate.
	fn collision(&self, shape: &Shape, force: &mut Vec3) -> Option<u32>;

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
pub fn projection(ratio: f64, fov: f64) -> Mat4 {
	let scale = (fov * 0.5 * ::std::f64::consts::PI / 180.).tan().recip();
	let yscale = scale * ratio;

	Mat4([
		scale,	0.,	0.,	0.,
		0.,	yscale,	0.,	0.,
		0.,	0.,	1.,	1.,
		0.,	0.,	0., 	1.,
	])
}

/// Turn model vertices into a bbox.
pub fn vertices_to_bbox(vertices: &[f32], mat4: Mat4) -> BBox {
	let mat = mat4.to_f32_array();

	let mut xmin = vertices[0];
	let mut ymin = vertices[1];
	let mut zmin = vertices[2];
	let mut xmax = vertices[0];
	let mut ymax = vertices[1];
	let mut zmax = vertices[2];

	for i in 4..vertices.len() {
		match i % 4 {
			0 => {
				let x = vertices[i];
				let y = vertices[i + 1];
				let z = vertices[i + 2];
				let w = vertices[i + 3];
				let x = mat[0]*x+mat[4]*y+mat[8]*z+mat[12]*w;

				if x < xmin {
					xmin = x;
				} else if x > xmax {
					xmax = x;
				}
			},
			1 => {
				let x = vertices[i - 1];
				let y = vertices[i];
				let z = vertices[i + 1];
				let w = vertices[i + 2];
				let y = mat[1]*x+mat[5]*y+mat[9]*z+mat[13]*w;

				if y < ymin {
					ymin = y;
				} else if y > ymax {
					ymax = y;
				}
			},
			2 => {
				let x = vertices[i - 2];
				let y = vertices[i - 1];
				let z = vertices[i];
				let w = vertices[i + 1];
				let z = mat[2]*x+mat[6]*y+mat[10]*z+mat[14]*w;

				if z < zmin {
					zmin = z;
				} else if z > zmax {
					zmax = z;
				}
			},
			_ => { },
		}
	}

	BBox::new(ami::Vec3::new(xmin, ymin, zmin),
		ami::Vec3::new(xmax, ymax, zmax))
}
