#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;
extern crate rand;
extern crate num;

mod math;
mod graphics;
mod utils;

pub use self::graphics::{Window, WindowArgs, Entity, Vertex, Frame};
pub use self::utils::{Index, IDManager, IDType};
pub use self::math::{Mat4, Vec2, Vec3, Vec4};

pub fn init() -> IDManager {
    graphics::init_vertex();
    IDManager::new()
}
