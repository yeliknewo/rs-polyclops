use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use utils::{ID};
use graphics::{Vertex, Index, DrawMethod};
use math::{Vec2, Vec3, Mat4};
use being::{Being, BeingType};

pub struct World<T: BeingType<T>> {
    beings: HashMap<ID, Arc<RwLock<Box<Being<T>>>>>,
}

impl<T: BeingType<T>> World<T> {
    pub fn new() -> World<T> {
        World {
            beings: HashMap::new(),
        }
    }

    pub fn get_beings(&self) -> &HashMap<ID, Arc<RwLock<Box<Being<T>>>>> {
        &self.beings
    }

    pub fn add_being(&mut self, being: Box<Being<T>>) {
        self.beings.insert(being.get_id(), Arc::new(RwLock::new(being)));
    }

    pub fn get_being(&mut self, id: ID) -> &Arc<RwLock<Box<Being<T>>>> {
        self.beings.get_mut(&id).expect("Unable to Get Mut Being in Get Being")
    }
}

pub fn get_rank<T: BeingType<T>>(event: WorldEvent<T>) -> u32 {
    match event {
        WorldEvent::NewBeing(_, _) => 100,
        WorldEvent::Pos2(vec2_event) => match vec2_event {
            Vec2Event::Set(_, _) => 5,
            Vec2Event::Add(_, _) => 6,
            Vec2Event::Mul(_, _) => 7,
        },
        WorldEvent::Pos3(vec3_event) => match vec3_event {
            Vec3Event::Set(_, _) => 5,
            Vec3Event::Add(_, _) => 6,
            Vec3Event::Mul(_, _) => 7,
        },
        WorldEvent::Vel2(vec2_event) => match vec2_event {
            Vec2Event::Set(_, _) => 10,
            Vec2Event::Add(_, _) => 11,
            Vec2Event::Mul(_, _) => 12,
        },
        WorldEvent::Vel3(vec3_event) => match vec3_event {
            Vec3Event::Set(_, _) => 10,
            Vec3Event::Add(_, _) => 11,
            Vec3Event::Mul(_, _) => 12,
        },
        WorldEvent::Acc2(vec2_event) => match vec2_event {
            Vec2Event::Set(_, _) => 15,
            Vec2Event::Add(_, _) => 16,
            Vec2Event::Mul(_, _) => 17,
        },
        WorldEvent::Acc3(vec3_event) => match vec3_event {
            Vec3Event::Set(_, _) => 15,
            Vec3Event::Add(_, _) => 16,
            Vec3Event::Mul(_, _) => 17,
        },
        WorldEvent::Entity(entity_event) => match entity_event {
            EntityEvent::Vertices(_, _) => 1,
            EntityEvent::Indices(_, _) => 1,
            EntityEvent::Texture(_, _) => 1,
            EntityEvent::DrawMethod(_, _) => 1,
            EntityEvent::Perspective(_, _) => 1,
            EntityEvent::View(_, _) => 1,
            EntityEvent::Model(_, _) => 1,
        },
    }
}

#[derive(Clone)]
pub enum WorldEvent<T: BeingType<T>> {
    NewBeing(T, Vec<WorldEvent<T>>),
    Pos2(Vec2Event),
    Pos3(Vec3Event),
    Vel2(Vec2Event),
    Vel3(Vec3Event),
    Acc2(Vec2Event),
    Acc3(Vec3Event),
    Entity(EntityEvent),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum EntityEvent {
    Vertices(ID, Vec<Vertex>),
    Indices(ID, Vec<Index>),
    Texture(ID, &'static [u8]),
    DrawMethod(ID, DrawMethod),
    Perspective(ID, Mat4),
    View(ID, Mat4),
    Model(ID, Mat4),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Vec2Event {
    Set(ID, Vec2),
    Add(ID, Vec2),
    Mul(ID, Vec2),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Vec3Event {
    Set(ID, Vec3),
    Add(ID, Vec3),
    Mul(ID, Vec3),
}
