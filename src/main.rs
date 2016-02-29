extern crate polyclops;

use polyclops::{World, WorldEvent, EntityEvent, Window, WindowArgs, Game, Entity, IDManager, Being, BeingType, ID, Vertex, Vec3, Mat4, IDType, DrawMethod, DepthTestMethod, CullingMethod};
use std::sync::{Arc, RwLock};

pub static RAW_TEXTURE: &'static [u8] = include_bytes!("..\\assets\\texture.png");

fn main() {
    let mut manager = polyclops::init();

    let mut game = Game::<MyBeingType>::new(&mut manager, 8,
        World::new(),
        Window::new(WindowArgs::Borderless("Polyclops".to_string()))
    );

    let mut starting_events = vec!(WorldEvent::NewBeing(MyBeingType::Mouse, vec!()));

    game.run(&mut manager, &mut starting_events);
}

#[derive(Clone)]
pub enum MyBeingType {
    Mouse,
}

impl BeingType<MyBeingType> for MyBeingType {
    fn make_being(manager: &mut IDManager, being_type: MyBeingType, events: &mut Vec<WorldEvent<MyBeingType>>, game: &mut Game<MyBeingType>, world: Arc<RwLock<World<MyBeingType>>>){
        let mut new_events = events.to_vec();
        let being = match being_type {
            MyBeingType::Mouse => {
                let being = Box::new(BeingMouse::new(manager));
                new_events.push(WorldEvent::Entity(EntityEvent::Vertices(being.get_id(), vec!(Vertex::from(Vec3::from([0.0, 0.0, -1.0])), Vertex::from(Vec3::from([1.0, 0.0, -1.0])), Vertex::from(Vec3::from([0.0, 1.0, -1.0]))))));
                new_events.push(WorldEvent::Entity(EntityEvent::Indices(being.get_id(), vec!(0, 1, 2))));
                new_events.push(WorldEvent::Entity(EntityEvent::Texture(being.get_id(), RAW_TEXTURE)));
                new_events.push(WorldEvent::Entity(EntityEvent::DrawMethod(being.get_id(), DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::Clockwise))));
                new_events.push(WorldEvent::Entity(EntityEvent::Perspective(being.get_id(), Mat4::perspective(0.1, 100.0, 90.0, 16.0 / 9.0))));
                new_events.push(WorldEvent::Entity(EntityEvent::View(being.get_id(), Mat4::view(0.0, 0.0, Vec3::from([0.0, 0.0, 0.0])))));
                new_events.push(WorldEvent::Entity(EntityEvent::Model(being.get_id(), Mat4::translation_from_vec3(Vec3::from([0.0, 0.0, 0.0])))));
                being
            },
        };
        world.write().expect("Unable to Write World in MyBeingType Make Being").add_being(being);
        game.execute_events(manager, &mut new_events, world);
    }
}

pub struct BeingMouse {
    entity: Entity,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    id: ID,
}

impl BeingMouse {
    pub fn new(manager: &mut IDManager) -> BeingMouse {
        BeingMouse {
            entity: Entity::new(manager),
            pos: Vec3::zero(),
            vel: Vec3::zero(),
            acc: Vec3::zero(),
            id: ID::new(manager, IDType::Being),
        }
    }
}

impl Being<MyBeingType> for BeingMouse {
    fn get_id(&self) -> ID {
        self.id
    }

    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn tick(&self, world: &World<MyBeingType>, delta_time: &f32) -> Vec<WorldEvent<MyBeingType>> {
        vec!()
    }

    fn get_pos3(&self) -> Vec3 {
        self.pos
    }

    fn get_vel3(&self) -> Vec3 {
        self.vel
    }

    fn get_acc3(&self) -> Vec3 {
        self.acc
    }

    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn set_pos3(&mut self, vec3: Vec3) {
        self.pos = vec3;
    }

    fn set_vel3(&mut self, vec3: Vec3) {
        self.vel = vec3;
    }

    fn set_acc3(&mut self, vec3: Vec3) {
        self.acc = vec3;
    }
}
