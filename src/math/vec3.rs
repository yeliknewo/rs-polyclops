use glium::uniforms::{AsUniformValue, UniformValue};

use std::ops::{Index, IndexMut, Add, Sub, Mul};

use math::{Vec4, Mat4};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
	vals: [f32; 3],
}

impl Vec3 {
	pub fn from_vals(vals: [f32; 3]) -> Vec3 {
		Vec3{
			vals: vals,
		}
	}

	pub fn zero() -> Vec3 {
		Vec3::from_vals([0.0; 3])
	}

	pub fn one() -> Vec3 {
		Vec3::from_vals([1.0; 3])
	}

	pub fn get_vals(&self) -> [f32; 3] {
		self.vals
	}

	pub fn dot(&self, other: Vec3) -> f32 {
		let mut sum = 0.0;

		for i in 0..3 {
			sum += self[i] * other[i];
		}

		sum
	}
}

impl AsUniformValue for Vec3 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Vec3(self.vals)
	}
}

impl Index<usize> for Vec3 {
	type Output = f32;

	fn index(&self, index: usize) -> &f32 {
		&self.vals[index]
	}
}

impl IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, index: usize) -> &mut f32 {
		&mut self.vals[index]
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3::from_vals([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3::from_vals([self[0] - other[0], self[1] - other[1], self[2] - other[2]])
	}
}

impl Mul<Mat4> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Mat4) -> Vec3 {
		let mut new = Vec3::zero();
		for y in 0..3 {
			let mut sum = 0.0;
			for x in 0..3 {
				sum = self[y] * other[x][y];
			}
			new[y] = sum;
		}
		new
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3 {
		Vec3::from_vals([self[0] * other[0], self[1] * other[1], self[2] * other[2]])
	}
}
