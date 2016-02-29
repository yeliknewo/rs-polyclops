use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use utils::{ID, EntityIDType};
use graphics::{Vertex, Index, DrawMethod};
use math::{Vec2, Vec3, Mat4};
use being::{Being, BeingType};

pub struct World<T: BeingType<T>> {
    beings: HashMap<ID, Arc<RwLock<Box<Being<T>>>>>,
    base_beings: HashMap<T, Arc<RwLock<Box<Being<T>>>>>,
}

impl<T: BeingType<T>> World<T> {
    pub fn new() -> World<T> {
        World {
            beings: HashMap::new(),
            base_beings: HashMap::new(),
        }
    }

    pub fn get_beings(&self) -> &HashMap<ID, Arc<RwLock<Box<Being<T>>>>> {
        &self.beings
    }

    pub fn add_being(&mut self, being: Box<Being<T>>) {
        self.beings.insert(being.get_id(), Arc::new(RwLock::new(being)));
    }

    pub fn del_being(&mut self, id: ID) {
        self.beings.remove(&id);
    }

    pub fn get_being(&self, id: ID) -> Option<&Arc<RwLock<Box<Being<T>>>>> {
        self.beings.get(&id)
    }

    pub fn set_base_being(&mut self, being: Box<Being<T>>) {
        self.base_beings.insert(being.get_type(), Arc::new(RwLock::new(being)));
    }

    pub fn get_base_being(&self, being_type: T) -> Option<&Arc<RwLock<Box<Being<T>>>>> {
        self.base_beings.get(&being_type)
    }
}

pub fn get_rank<T: BeingType<T>>(event: WorldEvent<T>) -> u32 {
    match event {
        WorldEvent::NewBeing(_, _) => 100,
        WorldEvent::NewBeingBase(_, _) => 200,
        WorldEvent::EndBeing(_) => 0,
        WorldEvent::Pos2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 5,
            Vec2Event::Add(_) => 6,
            Vec2Event::Mul(_) => 7,
        },
        WorldEvent::Pos3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 5,
            Vec3Event::Add(_) => 6,
            Vec3Event::Mul(_) => 7,
        },
        WorldEvent::Vel2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 10,
            Vec2Event::Add(_) => 11,
            Vec2Event::Mul(_) => 12,
        },
        WorldEvent::Vel3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 10,
            Vec3Event::Add(_) => 11,
            Vec3Event::Mul(_) => 12,
        },
        WorldEvent::Acc2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 15,
            Vec2Event::Add(_) => 16,
            Vec2Event::Mul(_) => 17,
        },
        WorldEvent::Acc3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 15,
            Vec3Event::Add(_) => 16,
            Vec3Event::Mul(_) => 17,
        },
        WorldEvent::Entity(_, entity_event) => match entity_event {
            EntityEvent::Vertices(_) => 1,
            EntityEvent::Indices(_) => 1,
            EntityEvent::Texture(_) => 1,
            EntityEvent::DrawMethod(_) => 1,
            EntityEvent::Perspective(_) => 1,
            EntityEvent::View(_) => 1,
            EntityEvent::Model(_) => 1,
            EntityEvent::UseNewID(_) => 2,
            EntityEvent::UseOldID(_, _) => 2,
            EntityEvent::UseBaseID(_, _) => 2,
        },
        WorldEvent::EntityBase(_, entity_base_event) => match entity_base_event {
            EntityBaseEvent::Vertices(_) => 1,
            EntityBaseEvent::Indices(_) => 1,
            EntityBaseEvent::Texture(_) => 1,
            EntityBaseEvent::DrawMethod(_) => 1,
            EntityBaseEvent::Perspective(_) => 1,
            EntityBaseEvent::View(_) => 1,
            EntityBaseEvent::Model(_) => 1,
            EntityBaseEvent::UseNewID(_) => 2,
            EntityBaseEvent::UseOldID(_, _) => 2,
            EntityBaseEvent::UseBaseID(_, _) => 2,
        },
    }
}

#[derive(Clone)]
pub enum WorldEvent<T: BeingType<T>> {
    NewBeing(T, Vec<WorldEvent<T>>),
    NewBeingBase(T, Vec<WorldEvent<T>>),
    EndBeing(ID),
    Pos2(ID, Vec2Event),
    Pos3(ID, Vec3Event),
    Vel2(ID, Vec2Event),
    Vel3(ID, Vec3Event),
    Acc2(ID, Vec2Event),
    Acc3(ID, Vec3Event),
    Entity(ID, EntityEvent<T>),
    EntityBase(T, EntityBaseEvent<T>),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum EntityBaseEvent<T: BeingType<T>> {
    Vertices(Vec<Vertex>),
    Indices(Vec<Index>),
    Texture(&'static [u8]),
    DrawMethod(DrawMethod),
    Perspective(Mat4),
    View(Mat4),
    Model(Mat4),
    UseNewID(Vec<EntityIDType>),
    UseOldID(ID, Vec<EntityIDType>),
    UseBaseID(T, Vec<EntityIDType>),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum EntityEvent<T: BeingType<T>> {
    Vertices(Vec<Vertex>),
    Indices(Vec<Index>),
    Texture(&'static [u8]),
    DrawMethod(DrawMethod),
    Perspective(Mat4),
    View(Mat4),
    Model(Mat4),
    UseNewID(Vec<EntityIDType>),
    UseOldID(ID, Vec<EntityIDType>),
    UseBaseID(T, Vec<EntityIDType>),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Vec2Event {
    Set(Vec2),
    Add(Vec2),
    Mul(Vec2),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Vec3Event {
    Set(Vec3),
    Add(Vec3),
    Mul(Vec3),
}
