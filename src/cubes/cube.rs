use std::sync::{Arc, RwLock};
use polyclops::{IDManager, WorldEvent, Game, World, Entity, Vec3, ID, IDType, Being, Transforms, EntityEvent, EntityIDType, Mat4};
use cubes::{CubeBeingType};

pub fn make_cube(manager: &mut IDManager, events: &mut Vec<WorldEvent<CubeBeingType>>, world: Arc<RwLock<World<CubeBeingType>>>) -> Vec<WorldEvent<CubeBeingType>> {
    let being: Box<Being<CubeBeingType>> = Box::new(BeingCube::new(manager));
    let mut events = events.to_vec();
    events = Game::fix_unset(events, &being);
    events.push(WorldEvent::Entity(being.get_id(), EntityEvent::UseBaseID(CubeBeingType::CubeBase, vec!(
        EntityIDType::Vertex,
        EntityIDType::Index,
        EntityIDType::Texture,
        EntityIDType::DrawParameter,
        EntityIDType::Perspective,
        EntityIDType::View,
    ))));
    let mat4 = Mat4::identity();
    events.push(WorldEvent::Entity(being.get_id(), EntityEvent::Model(mat4, mat4.to_inverse())));
    world.write().expect("Unable to Write World in Make Cube").add_being(being);
    events
}

struct BeingCube {
    entity: Entity,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    id: ID,
}

impl BeingCube {
    fn new(manager: &mut IDManager) -> BeingCube {
        BeingCube {
            entity: Entity::new(manager),
            pos: Vec3::zero(),
            vel: Vec3::zero(),
            acc: Vec3::zero(),
            id: ID::new(manager, IDType::Being),
        }
    }
}

impl Being<CubeBeingType> for BeingCube {
    fn get_type(&self) -> CubeBeingType {
        CubeBeingType::Cube
    }

    fn get_id(&self) -> ID {
        self.id
    }

    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn tick(&self, world: &World<CubeBeingType>, delta_time: &f32, transforms: &Transforms) -> Vec<WorldEvent<CubeBeingType>> {
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
