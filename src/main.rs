extern crate polyclops;

use std::sync::{Arc, RwLock};
use polyclops::{UNSET, World, Vec2Event, Vec2, WorldEvent, EntityEvent, EntityBaseEvent, EntityIDType, Window, WindowArgs, Game, Entity, IDManager, Being, BeingType, ID, Vertex, Vec3, Mat4, IDType, DrawMethod, DepthTestMethod, CullingMethod};

pub static RAW_TEXTURE: &'static [u8] = include_bytes!("..\\assets\\texture.png");

fn main() {
    let mut manager = polyclops::init();

    let mut window = Window::new(WindowArgs::Borderless("Polyclops".to_string()));

    let resolution = window.get_start_resolution();
    let resolution = Vec2::from([resolution.0 as f32, resolution.1 as f32]);

    let mut game = Game::<MyBeingType>::new(&mut manager, 8, World::new(resolution), resolution);

    let mut starting_events = vec!(
        WorldEvent::NewBeingBase(MyBeingType::MouseBase, vec!()),
        WorldEvent::NewBeing(MyBeingType::Mouse, vec!()),
    );



    game.run(&mut manager, &mut starting_events, &mut window);
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum MyBeingType {
    Mouse,
    MouseBase,
}

impl BeingType<MyBeingType> for MyBeingType {
    fn make_being(manager: &mut IDManager, being_type: MyBeingType, events: &mut Vec<WorldEvent<MyBeingType>>, window: &mut Window, game: &mut Game<MyBeingType>, world: Arc<RwLock<World<MyBeingType>>>){
        let mut new_events = events.to_vec();
        match being_type {
            MyBeingType::Mouse => {
                let being: Box<Being<MyBeingType>> = Box::new(BeingMouse::new(manager, false));
                new_events = Game::fix_unset(new_events, &being);
                {
                    new_events.push(WorldEvent::Entity(being.get_id(), EntityEvent::UseBaseID(MyBeingType::MouseBase, vec!(
                        EntityIDType::Vertex,
                        EntityIDType::Index,
                        EntityIDType::Texture,
                        EntityIDType::DrawParameter,
                        EntityIDType::Perspective,
                        EntityIDType::View,
                    ))));
                    new_events.push(WorldEvent::Entity(being.get_id(), EntityEvent::UseNewID(vec!(
                        EntityIDType::Model,
                    ))));
                    new_events.push(WorldEvent::Entity(being.get_id(), EntityEvent::Model(Mat4::translation_from_vec3(being.get_pos3()))));
                }
                world.write().expect("Unable to Write World in MyBeingType Make Being").add_being(being);
            },
            MyBeingType::MouseBase => {
                let being = Box::new(BeingMouse::new(manager, true));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::Vertices(vec!(
                    Vertex::from(Vec3::from([0.0, 0.0, -1.0])),
                    Vertex::from(Vec3::from([1.0, 0.0, -1.0])),
                    Vertex::from(Vec3::from([0.0, 1.0, -1.0]))
                ))));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::Indices(vec!(0, 1, 2, 2, 1, 0))));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::Texture(RAW_TEXTURE)));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::DrawMethod(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::Clockwise))));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::Perspective(Mat4::perspective(0.1, 100.0, 90.0, 16.0 / 9.0))));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::View(Mat4::view(0.0, 0.0, Vec3::from([0.0, 0.0, 0.0])))));
                new_events.push(WorldEvent::EntityBase(being.get_type(), EntityBaseEvent::Model(Mat4::translation_from_vec3(Vec3::from([0.0, 0.0, 0.0])))));
                world.write().expect("Unable to Write World in MyBeingType Make Being").set_base_being(being);
            },
        };
        game.execute_events(manager, window, &mut new_events, world);
    }
}

pub struct BeingMouse {
    being_type: MyBeingType,
    entity: Entity,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    id: ID,
    base: bool,
}

impl BeingMouse {
    pub fn new(manager: &mut IDManager, base: bool) -> BeingMouse {
        let being_type = match base {
            true => MyBeingType::MouseBase,
            false => MyBeingType::Mouse,
        };
        BeingMouse {
            being_type: being_type,
            entity: Entity::new(manager),
            pos: Vec3::zero(),
            vel: Vec3::zero(),
            acc: Vec3::zero(),
            id: ID::new(manager, IDType::Being),
            base: base,
        }
    }
}

impl Being<MyBeingType> for BeingMouse {
    fn get_type(&self) -> MyBeingType {
        self.being_type.clone()
    }

    fn get_id(&self) -> ID {
        self.id
    }

    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn tick(&self, world: &World<MyBeingType>, delta_time: &f32) -> Vec<WorldEvent<MyBeingType>> {
        if self.base {
            vec!()
        } else {
            let mut vec = vec!(WorldEvent::Entity(self.get_id(), EntityEvent::Model(Mat4::translation_from_vec3(self.get_pos3()))));
            vec.push(WorldEvent::Pos2(self.get_id(), Vec2Event::Set(world.get_mouse_pos())));
            vec
            // let mut vec = vec!(WorldEvent::Entity(self.get_id(), EntityEvent::Model(Mat4::translation_from_vec3(self.get_pos3()))), WorldEvent::Pos2(self.get_id(), Vec2Event::Add(self.get_vel2() * *delta_time)));
            // if self.get_pos3()[0] > 3.0 {
            //     vec.push(WorldEvent::Pos2(self.get_id(), Vec2Event::Set(Vec2::from([-3.0, 0.0]))));
            //     //vec.push(WorldEvent::NewBeing(MyBeingType::Mouse, vec!(WorldEvent::Vel2(UNSET, Vec2Event::Set(Vec2::from([1.0 + self.get_vel2()[0] * delta_time, 0.0]))))));
            // }
            // vec
        }
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
