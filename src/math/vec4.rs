use glium::uniforms::{AsUniformValue, UniformValue};

use std::ops::{Index, IndexMut, Add, Sub, Mul};

use math::{Mat4};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec4 {
	vals: [f32; 4],
}

impl Vec4 {
	pub fn from_vals(vals: [f32; 4]) -> Vec4 {
		Vec4{
			vals: vals,
		}
	}

	pub fn zero() -> Vec4 {
		Vec4::from_vals([0.0; 4])
	}

	pub fn one() -> Vec4 {
		Vec4::from_vals([1.0; 4])
	}

	pub fn x_unit() -> Vec4 {
		Vec4::from_vals([1.0, 0.0, 0.0, 0.0])
	}

	pub fn y_unit() -> Vec4 {
		Vec4::from_vals([0.0, 1.0, 0.0, 0.0])
	}

	pub fn z_unit() -> Vec4 {
		Vec4::from_vals([0.0, 0.0, 1.0, 0.0])
	}

	pub fn w_unit() -> Vec4 {
		Vec4::from_vals([0.0, 0.0, 0.0, 1.0])
	}

	pub fn get_vals(&self) -> [f32; 4] {
		self.vals
	}
}

impl AsUniformValue for Vec4 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Vec4(self.vals)
	}
}

impl Index<usize> for Vec4 {
	type Output = f32;

	fn index(&self, index: usize) -> &f32 {
		&self.vals[index]
	}
}

impl IndexMut<usize> for Vec4 {
	fn index_mut(&mut self, index: usize) -> &mut f32 {
		&mut self.vals[index]
	}
}

impl Add<Vec4> for Vec4{
	type Output = Vec4;

	fn add(self, other: Vec4) -> Vec4 {
		Vec4::from_vals([self[0] + other[0], self[1] + other[1], self[2] + other[2], self[3] + other[3]])
	}
}

impl Sub<Vec4> for Vec4 {
	type Output = Vec4;

	fn sub(self, other: Vec4) -> Vec4 {
		Vec4::from_vals([self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]])
	}
}

impl Mul<Mat4> for Vec4 {
	type Output = Vec4;

	fn mul(self, other: Mat4) -> Vec4 {
		let mut new = Vec4::zero();
		for y in 0..4 {
			let mut sum = 0.0;
			for x in 0..4 {
				sum = self[y] * other[x][y];
			}
			new[y] = sum;
		}
		new
	}
}

impl Mul<Vec4> for Vec4 {
	type Output = Vec4;

	fn mul(self, other: Vec4) -> Vec4 {
		Vec4::from_vals([self[0] * other[0], self[1] * other[1], self[2] * other[2], self[3] * other[3]])
	}
}
