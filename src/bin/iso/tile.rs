use std::sync::{Arc, RwLock};
use std::collections::{HashMap};
use polyclops::{Being, ID, IDManager, IDType, Entity, TickEvent, TickAfterEvent, World, Transforms, Vec3};

use iso::iso_being_type::IsoBeingType as IBT;

pub struct Tile {
    entities: HashMap<ID, Arc<RwLock<Entity>>>,
    id: ID,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Tile {
    pub fn new_base(manager: Arc<RwLock<IDManager>>) -> Tile {
        Tile {
            entities: HashMap::new(),
            id: ID::new(manager, IDType::Being),
            pos: Vec3::zero(),
            vel: Vec3::zero(),
            acc: Vec3::zero(),
        }
    }

    pub fn new_from_base(tile: Tile) -> Tile {

    }
}

impl Being<IBT> for Tile {
    fn get_type(&self) -> IBT {
        IBT::Tile
    }

    fn get_id(&self) -> ID {
        self.id
    }

    fn get_entities(&self) -> &HashMap<ID, Arc<RwLock<Entity>>> {
        &self.entities
    }

    fn tick(&self, world: &World<IBT>, transforms: &Transforms, delta_time: &f32) -> Vec<TickEvent<IBT>> {
        vec!()
    }

    fn tick_after(&self, world: &World<IBT>, transforms: &Transforms) -> Vec<TickAfterEvent<IBT>> {
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
