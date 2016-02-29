use std::ops::{Index, IndexMut, Mul};
use std::fmt;

use glium::uniforms::{AsUniformValue, UniformValue};

use math::{Vec3, Vec4};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Mat4{
	vals: [Vec4; 4],
}

impl Mat4 {
	pub fn identity() -> Mat4 {
		Mat4 {
			vals: [Vec4::x_unit(), Vec4::y_unit(), Vec4::z_unit(), Vec4::w_unit()],
		}
	}

	pub fn zero() -> Mat4 {
		Mat4{
			vals: [Vec4::zero(); 4],
		}
	}

	pub fn perspective(near: f32, far: f32, field_of_view: f32, aspect_ratio: f32) -> Mat4 {
		let d = 1.0 / ((field_of_view / 2.0).tan());
		Mat4::from([[
					d / aspect_ratio, 	0.0, 	0.0, 							0.0,
				],[
					0.0, 				d, 		0.0, 							0.0,
				],[
					0.0, 				0.0, 	(near + far) / (near - far), 	(2.0 * far) / (near - far),
				],[
					0.0, 				0.0, 	-1.0, 							0.0,
				]
			]
		)
	}

	pub fn orthographic(near: f32, far: f32, field_of_view: f32, aspect_ratio: f32) -> Mat4 {
		let d = 1.0 / ((field_of_view / 2.0).tan());
		Mat4::from([[
					d / aspect_ratio, 0.0, 0.0, 0.0,
				],[
					0.0, d, 0.0, 0.0,
				],[
					0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
				],[
					0.0, 0.0, 0.0, 1.0,
				]
			]
		)
	}

	pub fn view(pitch: f32, yaw: f32, camera_position: Vec3) -> Mat4{
		let pitch_cos = pitch.cos();
		let pitch_sin = pitch.sin();

		let yaw_cos = yaw.cos();
		let yaw_sin = yaw.sin();

		let x = Vec3::from([yaw_cos, 0.0, -yaw_sin]);
		let y = Vec3::from([yaw_sin * pitch_sin, pitch_cos, yaw_cos * pitch_sin]);
		let z = Vec3::from([yaw_sin * pitch_cos, -pitch_sin, pitch_cos * yaw_cos]);

		Mat4::from([[
					x[0], x[1], x[2], -x.dot(camera_position),
				],[
					y[0], y[1], y[2], -y.dot(camera_position),
				],[
					z[0], z[1], z[2], -z.dot(camera_position),
				],[
					0.0, 0.0, 0.0, 1.0,
				]
			]
		)
	}

	pub fn scalation_from_vec3(vec3: Vec3) -> Mat4 {
		Mat4::from([[
					vec3[0], 0.0, 0.0, 0.0,
				],[
					0.0, vec3[1], 0.0, 0.0,
				],[
					0.0, 0.0, vec3[2], 0.0,
				],[
					0.0, 0.0, 0.0, 1.0,
				]
			]
		)
	}

	pub fn translation_from_vec3(vec3: Vec3) -> Mat4 {
		Mat4::from([[
					1.0, 	0.0, 		0.0, 		0.0,
				],[
					0.0, 	1.0, 		0.0, 		0.0,
				],[
					0.0, 	0.0, 		1.0,		0.0,
				],[
					vec3[0], vec3[1], 	vec3[2], 	1.0,
				]
			]
		)
	}

	pub fn rotation_from_vec3(vec3: Vec3) -> Mat4 {
		Mat4::x_rotation(vec3[0]) * Mat4::y_rotation(vec3[1]) * Mat4::z_rotation(vec3[2])
	}

	pub fn x_rotation(x_rads: f32) -> Mat4 {
		let c = x_rads.cos();
		let s = x_rads.sin();
		let ns = -s;
		Mat4::from([[
					1.0,	0.0,	0.0, 	0.0,
				],[
					0.0, 	c,		ns,		0.0,
				],[
					0.0, 	s, 		c, 		0.0,
				],[
					0.0, 	0.0,	0.0, 	1.0,
				]
			]
		)
	}

	pub fn y_rotation(y_rads: f32) -> Mat4 {
		let c = y_rads.cos();
		let s = y_rads.sin();
		let ns = -s;
		Mat4::from([[
					c, 		0.0, 	s, 		0.0,
				],[
					0.0, 	1.0, 	0.0, 	0.0,
				],[
					ns, 	0.0, 	c, 		0.0,
				],[
					0.0, 	0.0, 	0.0, 	1.0,
				]
			]
		)
	}

	pub fn z_rotation(z_rads: f32) -> Mat4 {
		let c = z_rads.cos();
		let s = z_rads.sin();
		let ns = -s;
		Mat4::from([[
					c,		ns, 	0.0, 	0.0,
				],[
					s, 		c, 		0.0,	0.0,
				],[
					0.0, 	0.0, 	1.0, 	0.0,
				],[
					0.0, 	0.0, 	0.0, 	1.0,
				]
			]
		)
	}

	fn get_vals(&self) -> [[f32; 4]; 4] {
		[
			self.vals[0].get_vals(),
			self.vals[1].get_vals(),
			self.vals[2].get_vals(),
			self.vals[3].get_vals(),
		]
	}
}

impl From<[[f32; 4]; 4]> for Mat4 {
	fn from(vals: [[f32; 4]; 4]) -> Mat4 {
		Mat4 {
			vals: [
				Vec4::from(vals[0]),
				Vec4::from(vals[1]),
				Vec4::from(vals[2]),
				Vec4::from(vals[3]),
			]
		}
	}
}

impl AsUniformValue for Mat4 {
	fn as_uniform_value(&self) -> UniformValue {
		UniformValue::Mat4(self.get_vals())
	}
}

impl Index<usize> for Mat4 {
	type Output = Vec4;

	fn index(&self, index: usize) -> &Vec4 {
		&self.vals[index]
	}
}

impl IndexMut<usize> for Mat4 {
	fn index_mut(&mut self, index: usize) -> &mut Vec4 {
		&mut self.vals[index]
	}
}


impl Mul<Mat4> for Mat4 {
	type Output = Mat4;

	fn mul(self, other: Mat4) -> Mat4 {
		let mut new: Mat4 = Mat4::zero();
		for x in 0..4 {
			for y in 0..4 {
				let mut sum = 0.0;
				for i in 0..4 {
					sum += self[i][x] * other[y][i];
				}
				new[x][y] = sum;
			}
		}
		new
	}
}

impl fmt::Display for Mat4 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "[{},{},{},{}]\n[{},{},{},{}]\n[{},{},{},{}]\n[{},{},{},{}]",
			self[0][0], self[1][0], self[2][0], self[3][0],
			self[0][1], self[1][1], self[2][1], self[3][1],
			self[0][2], self[1][2], self[2][2], self[3][2],
			self[0][3], self[1][3], self[2][3], self[3][3])
	}
}
