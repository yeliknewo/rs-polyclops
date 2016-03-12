use std::sync::{Arc, RwLock};
use std::f32::consts::{PI};
use polyclops::{init, Window, WindowArgs, Game, World, BeingType, Being, IDManager, WorldEvent,
    TickEvent, TickAfterEvent, TransformEvent, EntityGraphicsEvent, EntityIDEvent, EntityIDType,
    DrawMethod, DepthTestMethod, CullingMethod, Mat4, Vec3, Vec2, Vertex, BeingArgs,
};

mod tile;

use self::tile::Tile;

const TILE_TEXTURE: &'static [u8] = include_bytes!("../../../assets/texture.png");
const TILE_BRICK_TEXTURE: &'static [u8] = include_bytes!("../../../assets/TileBrick.png");
const TILE_WOOD_TEXTURE: &'static [u8] = include_bytes!("../../../assets/TileWood.png");

pub fn main() {
    let manager = init();

    let mut window = Window::new(WindowArgs::Borderless("iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game: Game<IBT> = Game::<IBT>::new(manager, thread_count, World::new(resolution), resolution);
    let mut events = vec!(WorldEvent::Tick(TickEvent::NewBase(IBT::Tile)));
    for _ in 0..1 {
        events.push(WorldEvent::Tick(TickEvent::NewBeing(
            IBT::Tile, BeingArgs::new()
            .with_pos(Vec3::from([0.0, 0.0, 0.0]))
            .with_sca(Vec3::from([0.1, 0.1, 1.0]))
            .with_rot(Vec3::from([-45.0 * PI / 180.0, 0.0, 45.0 * PI / 180.0]))
        )));
    }
    game.run(events, &mut window);
}

use self::iso_being_type::IsoBeingType as IBT;

pub mod iso_being_type {
    #[derive(Clone, Hash, Eq, PartialEq)]
    pub enum IsoBeingType {
        Tile,
    }
}

impl BeingType<IBT> for IBT {
    fn make_being(manager: Arc<RwLock<IDManager>>, being_type: IBT, world: Arc<RwLock<World<IBT>>>, being_args: BeingArgs) -> Vec<WorldEvent<IBT>> {
        let mut events: Vec<WorldEvent<IBT>> = vec!();
        let being = match being_type.clone() {
            IBT::Tile => {
                let being = {
                    let world = world.read().expect("Unable to Read World in Make Being IBT");
                    let base = world.get_base(being_type.clone()).expect("Unable to Get Base in Make Being IBT");
                    Tile::new_from_base(manager, base, being_args)
                };
                let id = being.get_id();
                events.push(WorldEvent::Tick(TickEvent::EntityID(id, tile::ENTITY_TILE_ID, EntityIDEvent::UseNewID(vec!(EntityIDType::Model)))));
                let mat4 = Mat4::identity();
                events.push(WorldEvent::Tick(TickEvent::Transform(id, tile::ENTITY_TILE_ID, TransformEvent::Model(mat4, mat4.to_inverse()))));
                being
            },
        };
        world.write().expect("Unable to Write World in Make Being IBT").add_being(Box::new(being));
        events
    }

    fn make_base(manager: Arc<RwLock<IDManager>>, being_type: IBT, world: Arc<RwLock<World<IBT>>>) -> Vec<WorldEvent<IBT>> {
        let mut events: Vec<WorldEvent<IBT>> = vec!();
        let being = match being_type.clone() {
            IBT::Tile => {
                let being = Tile::new_base(manager);
                let mat4 = Mat4::orthographic(0.1, 100.0, 75.0, world.read().expect("Unable to Read World in Make Base IBT").get_aspect_ratio());
                events.push(WorldEvent::Tick(TickEvent::TransformBase(being_type.clone(), tile::ENTITY_TILE_ID, TransformEvent::Perspective(mat4, mat4.to_inverse()))));
                let mat4 = Mat4::view(0.0, 0.0, Vec3::zero());
                events.push(WorldEvent::Tick(TickEvent::TransformBase(being_type.clone(), tile::ENTITY_TILE_ID, TransformEvent::View(mat4, mat4.to_inverse()))));
                let mat4 = Mat4::identity();
                events.push(WorldEvent::Tick(TickEvent::TransformBase(being_type.clone(), tile::ENTITY_TILE_ID, TransformEvent::Model(mat4, mat4.to_inverse()))));
                events.push(WorldEvent::TickAfter(TickAfterEvent::EntityBase(being_type.clone(), tile::ENTITY_TILE_ID, EntityGraphicsEvent::Vertices(vec!(
                    Vertex::from(Vec2::from([0.0, 0.0])),
                    Vertex::from(Vec2::from([1.0, 0.0])),
                    Vertex::from(Vec2::from([1.0, 1.0])),
                    Vertex::from(Vec2::from([0.0, 1.0])),
                )))));
                events.push(WorldEvent::TickAfter(TickAfterEvent::EntityBase(being_type.clone(), tile::ENTITY_TILE_ID, EntityGraphicsEvent::Indices(vec!(
                    0, 1, 2,
                    2, 3, 0,
                )))));
                events.push(WorldEvent::TickAfter(TickAfterEvent::EntityBase(being_type.clone(), tile::ENTITY_TILE_ID, EntityGraphicsEvent::Texture(TILE_BRICK_TEXTURE))));
                events.push(WorldEvent::TickAfter(TickAfterEvent::EntityBase(being_type.clone(), tile::ENTITY_TILE_ID, EntityGraphicsEvent::DrawMethod(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::Clockwise)))));
                being
            },
        };
        world.write().expect("Unable to Write World in Make Base IBT").set_base(being_type.clone(), Box::new(being));
        events
    }
}
