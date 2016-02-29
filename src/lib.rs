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
mod world;
mod game;
mod being;

pub use self::graphics::{Window, WindowArgs, Entity, Vertex, Index, Frame, DrawMethod, DepthTestMethod, CullingMethod};
pub use self::utils::{ID, IDManager, IDType, EntityIDType};
pub use self::math::{Mat4, Vec2, Vec3, Vec4};
pub use self::being::{Being, BeingType};
pub use self::world::{World, WorldEvent, EntityEvent};
pub use self::game::{Game};

pub fn init() -> IDManager {
    graphics::init_vertex();
    IDManager::new()
}
