use std::sync::{Arc, RwLock};
use polyclops::{IDManager, WorldEvent, Game, World, Entity, Vec3, ID, IDType, Being, Transforms, EntityEvent, EntityIDType, Mat4, Window, DrawMethod, method_to_parameters, DepthTestMethod, CullingMethod};
use cubes::cbt::CubeBeingType as CBT;

static CUBE_TEXTURE: &'static [u8] = include_bytes!("..\\..\\assets\\texture.png");

pub fn make_base(manager: &mut IDManager, window: &mut Window, transforms: Arc<RwLock<Transforms>>, aspect_ratio: f32) -> Entity {
    let entity = Entity::new(manager);
    window.set_vertices(&entity, vec!(

    ));
    window.set_indices(&entity, vec!(

    ));
    window.set_texture(&entity, CUBE_TEXTURE);
    window.set_draw_parameters(&entity, method_to_parameters(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::CounterClockwise)));
    let mut trans = transforms.write().expect("Unable to Write Transforms in Make Base in Cube");
    let mat4 = Mat4::perspective(0.1, 10.0, 75.0, aspect_ratio);
    trans.set_perspective_matrix(&entity, mat4, mat4.to_inverse());
    let mat4 = Mat4::view(0.0, 0.0, Vec3::from([0.0, 0.0, 0.0]));
    trans.set_view_matrix(&entity, mat4, mat4.to_inverse());
    let mat4 = Mat4::identity();
    trans.set_model_matrix(&entity, mat4, mat4.to_inverse());
    entity
}

pub fn make_cube(manager: &mut IDManager, events: &mut Vec<WorldEvent<CBT>>) -> (Box<Being<CBT>>, Vec<WorldEvent<CBT>>) {
    let being: Box<Being<CBT>> = Box::new(BeingCube::new(manager));
    let mut events = events.to_vec();
    events = Game::fix_unset(events, &being);
    events.push(WorldEvent::Entity(being.get_id(), EntityEvent::UseBaseID(CBT::Cube, vec!(
        EntityIDType::Vertex,
        EntityIDType::Index,
        EntityIDType::Texture,
        EntityIDType::DrawParameter,
        EntityIDType::Perspective,
        EntityIDType::View,
    ))));
    let mat4 = Mat4::identity();
    events.push(WorldEvent::Entity(being.get_id(), EntityEvent::Model(mat4, mat4.to_inverse())));
    (being, events)
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

impl Being<CBT> for BeingCube {
    fn get_type(&self) -> CBT {
        CBT::Cube
    }

    fn get_id(&self) -> ID {
        self.id
    }

    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn tick(&self, world: &World<CBT>, delta_time: &f32, transforms: &Transforms) -> Vec<WorldEvent<CBT>> {
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
