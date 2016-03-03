use std::sync::{Arc, RwLock};
use polyclops::{IDManager, WorldEvent, Game, World, Entity, EntityIDType, EntityEvent, Being, Mat4};
use cubes::{CubeBeingType};

pub fn make_cube_base(manager: &mut IDManager, events: &mut Vec<WorldEvent<CubeBeingType>>, world: Arc<RwLock<World<CubeBeingType>>>) -> Vec<WorldEvent<CubeBeingType>> {
    let being: Box<Being<CubeBeingType>> = Box::new(BeingCubeBase::new(manager));
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

struct BeingCubeBase {
    entity: Entity,
}

impl BeingCubeBase {
    fn new(manager: &mut IDManager) -> BeingCubeBase {
        BeingCubeBase {
            entity: Entity::new(manager),
        }
    }
}

impl Being<CubeBeingType> for CubeBase {
    
}
