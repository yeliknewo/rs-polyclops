use glium::uniforms::{AsUniformValue, UniformValue};

use std::ops::{Index, IndexMut, Add, Sub, Mul};

use math::{Mat4, Vec3, Vec4};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
	vals: [f32; 2],
}

impl Vec2 {
	pub fn zero() -> Vec2 {
		Vec2::from([0.0; 2])
	}

	pub fn one() -> Vec2 {
		Vec2::from([1.0; 2])
	}

	pub fn get_vals(&self) -> [f32; 2] {
		self.vals
	}

	pub fn dot(&self, other: Vec2) -> f32 {
		let mut sum = 0.0;

		for i in 0..2 {
			sum += self[i] * other[i];
		}

		sum
	}

	pub fn to_vec3(&self, z: f32) -> Vec3 {
		Vec3::from([self.vals[0], self.vals[1], z])
	}
}

impl From<[f32; 2]> for Vec2 {
	fn from(vals: [f32;2]) -> Vec2 {
		Vec2{
			vals: vals,
		}
	}
}

impl From<Vec3> for Vec2 {
	fn from(vec3: Vec3) -> Vec2 {
		Vec2::from([vec3[0], vec3[1]])
	}
}

impl From<Vec4> for Vec2 {
	fn from(vec4: Vec4) -> Vec2 {
		Vec2::from([vec4[0], vec4[1]])
	}
}

impl AsUniformValue for Vec2 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Vec2(self.vals)
	}
}

impl Index<usize> for Vec2 {
	type Output = f32;

	fn index(&self, index: usize) -> &f32 {
		&self.vals[index]
	}
}

impl IndexMut<usize> for Vec2 {
	fn index_mut(&mut self, index: usize) -> &mut f32 {
		&mut self.vals[index]
	}
}

impl Add<Vec2> for Vec2 {
	type Output = Vec2;

	fn add(self, other: Vec2) -> Vec2 {
		Vec2::from([self[0] + other[0], self[1] + other[1]])
	}
}

impl Sub<Vec2> for Vec2 {
	type Output = Vec2;

	fn sub(self, other: Vec2) -> Vec2 {
		Vec2::from([self[0] - other[0], self[1] - other[1]])
	}
}

impl Mul<Mat4> for Vec2 {
	type Output = Vec2;

	fn mul(self, other: Mat4) -> Vec2 {
		let mut new = Vec2::zero();
		for y in 0..2 {
			let mut sum = 0.0;
			for x in 0..2 {
				sum = self[y] * other[x][y];
			}
			new[y] = sum;
		}
		new
	}
}

impl Mul<Vec2> for Vec2 {
	type Output = Vec2;

	fn mul(self, other: Vec2) -> Vec2 {
		Vec2::from([self[0] * other[0], self[1] * other[1]])
	}
}
