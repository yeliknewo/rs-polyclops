extern crate polyclops;

mod cubes;

fn main() {
    cubes::main();
}

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub enum MyBeingType {
//     Mouse,
//     MouseBase,
// }
//
// impl BeingType<MyBeingType> for MyBeingType {
//     fn make_being(manager: &mut IDManager, being_type: MyBeingType, events: &mut Vec<WorldEvent<MyBeingType>>, window: &mut Window, game: &mut Game<MyBeingType>, world: Arc<RwLock<World<MyBeingType>>>) {
//         let mut new_events = events.to_vec();
//         match being_type {
//             MyBeingType::Mouse => {
//                 let being: Box<Being<MyBeingType>> = Box::new(BeingMouse::new(manager, false));
//                 new_events = Game::fix_unset(new_events, &being);
//                 {
//                     new_events.push(WorldEvent::Entity(being.get_id(), EntityEvent::UseBaseID(MyBeingType::MouseBase, vec!(
//                         EntityIDType::Vertex,
//                         EntityIDType::Index,
//                         EntityIDType::Texture,
//                         EntityIDType::DrawParameter,
//                         EntityIDType::Perspective,
//                         EntityIDType::View,
//                     ))));
//                     new_events.push(WorldEvent::Entity(being.get_id(), EntityEvent::UseNewID(vec!(
//                         EntityIDType::Model,
//                     ))));
//                     let mat4 = Mat4::identity();
//                     new_events.push(WorldEvent::Entity(being.get_id(), EntityEvent::Model(mat4, mat4.to_inverse())));
//                 }
//                 world.write().expect("Unable to Write World in MyBeingType Make Being").add_being(being);
//             },
//             MyBeingType::MouseBase => {
//                 let being = Box::new(BeingMouse::new(manager, true));
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::Vertices(vec!(
//                     Vertex::from(Vec3::from([0.0, 0.0, 0.0])),
//                     Vertex::from(Vec3::from([1.0, -1.0, 0.0])),
//                     Vertex::from(Vec3::from([0.0, -1.0, 0.0]))
//                 ))));
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::Indices(vec!(0, 1, 2, 2, 1, 0))));
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::Texture(RAW_TEXTURE)));
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::DrawMethod(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::Clockwise))));
//                 //let mat4 = Mat4::perspective(0.1, 10.0, 75.0, world.read().expect("Unable to Read World in MyBeingType Make Being").get_aspect_ratio());
//                 //let mat4 = Mat4::orthographic(0.1, 10.0, 75.0, world.read().expect("Unable to Read World in MyBeingType Make Being").get_aspect_ratio());
//                 let mat4 = Mat4::identity();
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::Perspective(mat4, mat4.to_inverse())));
//                 let mat4 = Mat4::view(0.0, 0.0, Vec3::from([0.0, 0.0, 0.0]));
//                 //let mat4 = Mat4::identity();
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::View(mat4, mat4.to_inverse())));
//                 //let mat4 = Mat4::identity();
//                 new_events.push(WorldEvent::EntityBase(being.get_type(), EntityEvent::Model(mat4, mat4.to_inverse())));
//                 world.write().expect("Unable to Write World in MyBeingType Make Being").set_base(being);
//             },
//         };
//         game.execute_events(manager, window, &mut new_events, world);
//     }
// }
//
// pub struct BeingMouse {
//     being_type: MyBeingType,
//     entity: Entity,
//     pos: Vec3,
//     vel: Vec3,
//     acc: Vec3,
//     id: ID,
//     base: bool,
// }
//
// impl BeingMouse {
//     pub fn new(manager: &mut IDManager, base: bool) -> BeingMouse {
//         let being_type = match base {
//             true => MyBeingType::MouseBase,
//             false => MyBeingType::Mouse,
//         };
//         BeingMouse {
//             being_type: being_type,
//             entity: Entity::new(manager),
//             pos: Vec3::zero(),
//             vel: Vec3::zero(),
//             acc: Vec3::zero(),
//             id: ID::new(manager, IDType::Being),
//             base: base,
//         }
//     }
// }
//
// impl Being<MyBeingType> for BeingMouse {
//     fn get_type(&self) -> MyBeingType {
//         self.being_type.clone()
//     }
//
//     fn get_id(&self) -> ID {
//         self.id
//     }
//
//     fn get_entity(&self) -> &Entity {
//         &self.entity
//     }
//
//     fn tick(&self, world: &World<MyBeingType>, delta_time: &f32, transforms: &Transforms) -> Vec<WorldEvent<MyBeingType>> {
//         if self.base {
//             vec!()
//         } else {
//             let mat4 = Mat4::translation_from_vec3(self.get_pos3());
//             let mut vec = vec!(WorldEvent::Entity(self.get_id(), EntityEvent::Model(mat4, mat4.to_inverse())));
//             vec.push(WorldEvent::Pos2(self.get_id(), Vec2Event::Set(transforms.backwards2(world.get_mouse_pos_world(), &self.entity))));
//             vec
//         }
//     }
//
//     fn get_pos3(&self) -> Vec3 {
//         self.pos
//     }
//
//     fn get_vel3(&self) -> Vec3 {
//         self.vel
//     }
//
//     fn get_acc3(&self) -> Vec3 {
//         self.acc
//     }
//
//     fn get_entity_mut(&mut self) -> &mut Entity {
//         &mut self.entity
//     }
//
//     fn set_pos3(&mut self, vec3: Vec3) {
//         self.pos = vec3;
//     }
//
//     fn set_vel3(&mut self, vec3: Vec3) {
//         self.vel = vec3;
//     }
//
//     fn set_acc3(&mut self, vec3: Vec3) {
//         self.acc = vec3;
//     }
// }
