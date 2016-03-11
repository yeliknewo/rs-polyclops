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
mod keyboard;

pub use self::graphics::{method_to_parameters, Transforms, Window, WindowArgs, Entity, Vertex, Index, DrawMethod, DepthTestMethod, CullingMethod};
pub use self::utils::{UNSET, ID, IDManager, IDType, EntityIDType};
pub use self::math::{Mat4, Vec2, Vec3, Vec4};
pub use self::being::{Being, BeingType};
pub use self::world::{World, WorldEvent, TickEvent, TickAfterEvent, EntityGraphicsEvent, Vec2Event, Vec3Event};
pub use self::game::{Game};
pub use self::keyboard::{Keyboard};

pub fn init() -> IDManager {
    graphics::init_vertex();
    IDManager::new()
}
