use std::sync::{Arc, RwLock};
use std::collections::{HashMap};
use polyclops::{Being, ID, IDManager, IDType, Entity, TickEvent, TickAfterEvent, TransformEvent, World, Transforms, Vec3, Mat4, BeingArgs};

use iso::iso_being_type::IsoBeingType as IBT;

pub const ENTITY_TILE_ID: u32 = 0;

pub struct Tile {
    entities: HashMap<u32, Arc<RwLock<Entity>>>,
    id: ID,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    sca: Vec3,
    rot: Vec3,
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
            sca: Vec3::one(),
            rot: Vec3::zero(),
        }
    }

    pub fn new_from_base(manager: Arc<RwLock<IDManager>>, base: &Arc<RwLock<Box<Being<IBT>>>>, being_args: BeingArgs) -> Tile {
        let base = base.read().expect("Unable to Read Base in New From Base in Tile");
        let mut entities: HashMap<u32, Arc<RwLock<Entity>>> = HashMap::new();
        for entry in base.get_entities() {
            entities.insert(*entry.0, Arc::new(RwLock::new(Entity::new_from(entry.1))));
        }
        let pos = match being_args.pos {
            Some(b) => *b,
            None => base.get_pos3(),
        };
        let vel = match being_args.vel {
            Some(b) => *b,
            None => base.get_vel3(),
        };
        let acc = match being_args.acc {
            Some(b) => *b,
            None => base.get_acc3(),
        };
        let sca = match being_args.sca {
            Some(b) => *b,
            None => base.get_sca3(),
        };
        let rot = match being_args.rot {
            Some(b) => *b,
            None => base.get_rot3(),
        };
        Tile {
            entities: entities,
            id: ID::new(manager, IDType::Being),
            pos: pos,
            vel: vel,
            acc: acc,
            sca: sca,
            rot: rot,
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
        let mut events = vec!();
        for entry in self.get_entities() {
            let mat4 = Mat4::translation_from_vec3(self.get_pos3()) * Mat4::scalation_from_vec3(self.get_sca3()) * Mat4::rotation_from_vec3(self.get_rot3());
            events.push(TickEvent::Transform(self.get_id(), *entry.0, TransformEvent::Model(mat4, mat4.to_inverse())))
        }
        events
    }

    fn tick_after(&self, world: &World<IBT>, transforms: &Transforms) -> Vec<TickAfterEvent<IBT>> {
        let mut events = vec!();
        events
    }

    implement_being!(sca, get_sca3, set_sca3);
    implement_being!(rot, get_rot3, set_rot3);
    implement_being!(pos, get_pos3, set_pos3);
    implement_being!(vel, get_vel3, set_vel3);
    implement_being!(acc, get_acc3, set_acc3);
}
