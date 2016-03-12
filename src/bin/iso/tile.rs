use std::sync::{Arc, RwLock};
use std::collections::{HashMap};
use polyclops::{Being, ID, IDManager, IDType, Entity, TickEvent, TickAfterEvent, TransformEvent, World, Transforms, Vec3, Mat4};

use iso::iso_being_type::IsoBeingType as IBT;

pub const ENTITY_TILE_ID: u32 = 0;

pub struct Tile {
    entities: HashMap<u32, Arc<RwLock<Entity>>>,
    id: ID,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Tile {
    pub fn new_base(manager: Arc<RwLock<IDManager>>) -> Tile {
        let mut entities = HashMap::new();
        let tile_entity = Entity::new(manager.clone());
        entities.insert(ENTITY_TILE_ID, Arc::new(RwLock::new(tile_entity)));
        Tile {
            entities: entities,
            id: ID::new(manager.clone(), IDType::Being),
            pos: Vec3::zero(),
            vel: Vec3::zero(),
            acc: Vec3::zero(),
        }
    }

    pub fn new_from_base(manager: Arc<RwLock<IDManager>>, base: &Arc<RwLock<Box<Being<IBT>>>>) -> Tile {
        let base = base.read().expect("Unable to Read Base in New From Base in Tile");
        let mut entities: HashMap<u32, Arc<RwLock<Entity>>> = HashMap::new();
        for entry in base.get_entities() {
            entities.insert(*entry.0, Arc::new(RwLock::new(Entity::new_from(entry.1))));
        }
        Tile {
            entities: entities,
            id: ID::new(manager, IDType::Being),
            pos: base.get_pos3(),
            vel: base.get_vel3(),
            acc: base.get_acc3(),
        }
    }
}

impl Being<IBT> for Tile {
    fn get_type(&self) -> IBT {
        IBT::Tile
    }

    fn get_id(&self) -> ID {
        self.id
    }

    fn get_entities(&self) -> &HashMap<u32, Arc<RwLock<Entity>>> {
        &self.entities
    }

    fn tick(&self, world: &World<IBT>, transforms: &Transforms, delta_time: &f32) -> Vec<TickEvent<IBT>> {
        vec!()
    }

    fn tick_after(&self, world: &World<IBT>, transforms: &Transforms) -> Vec<TickAfterEvent<IBT>> {
        let mut events = vec!();
        for entry in self.get_entities() {
            let mat4 = Mat4::translation_from_vec3(self.get_pos3());
            events.push(TickAfterEvent::Transform(self.get_id(), *entry.0, TransformEvent::Model(mat4, mat4.to_inverse())))
        }
        events
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
