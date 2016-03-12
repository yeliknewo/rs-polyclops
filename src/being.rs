use std::sync::{Arc, RwLock};
use std::hash::{Hash};
use std::collections::{HashMap};

use utils::{ID, IDManager};
use graphics::{Entity, Transforms};
use world::{World, WorldEvent, TickEvent, TickAfterEvent};
use math::{Vec2, Vec3};
use being_args::{BeingArgs};

pub trait BeingType<T: BeingType<T>>: Send + Sync + Clone + Eq + PartialEq + Hash {
    fn make_being(Arc<RwLock<IDManager>>, T, Arc<RwLock<World<T>>>, BeingArgs) -> Vec<WorldEvent<T>>;
    fn make_base(Arc<RwLock<IDManager>>, T, Arc<RwLock<World<T>>>) -> Vec<WorldEvent<T>>;
}

pub trait Being<T: BeingType<T>>: Send + Sync {
    fn get_type(&self) -> T;
    fn get_id(&self) -> ID;
    fn get_entity(&self, id: u32) -> Option<&Arc<RwLock<Entity>>> {
        self.get_entities().get(&id)
    }
    fn get_entities(&self) -> &HashMap<u32, Arc<RwLock<Entity>>>;
    fn tick(&self, &World<T>, &Transforms, &f32) -> Vec<TickEvent<T>>;
    fn tick_after(&self, &World<T>, &Transforms) -> Vec<TickAfterEvent<T>>;
    fn get_sca2(&self) -> Vec2 {
        Vec2::from(self.get_sca3())
    }
    fn get_sca3(&self) -> Vec3;
    fn get_rot2(&self) -> Vec2 {
        Vec2::from(self.get_rot3())
    }
    fn get_rot3(&self) -> Vec3;
    fn get_pos2(&self) -> Vec2 {
        Vec2::from(self.get_pos3())
    }
    fn get_pos3(&self) -> Vec3;
    fn get_vel2(&self) -> Vec2 {
        Vec2::from(self.get_vel3())
    }
    fn get_vel3(&self) -> Vec3;
    fn get_acc2(&self) -> Vec2 {
        Vec2::from(self.get_acc3())
    }
    fn get_acc3(&self) -> Vec3;

    fn set_sca2(&mut self, vec2: Vec2) {
        let z = self.get_sca3()[2];
        self.set_sca3(vec2.to_vec3(z));
    }
    fn set_sca3(&mut self, Vec3);
    fn add_sca2(&mut self, vec2: Vec2) {
        self.add_sca3(vec2.to_vec3(0.0));
    }
    fn add_sca3(&mut self, vec3: Vec3) {
        let sca3 = self.get_sca3();
        self.set_sca3(sca3 + vec3);
    }
    fn mul_sca2(&mut self, vec2: Vec2) {
        self.mul_sca3(vec2.to_vec3(1.0));
    }
    fn mul_sca3(&mut self, vec3: Vec3) {
        let sca3 = self.get_sca3();
        self.set_sca3(sca3 * vec3);
    }
    fn set_rot2(&mut self, vec2: Vec2) {
        let z = self.get_rot3()[2];
        self.set_rot3(vec2.to_vec3(z));
    }
    fn set_rot3(&mut self, Vec3);
    fn add_rot2(&mut self, vec2: Vec2) {
        self.add_rot3(vec2.to_vec3(0.0));
    }
    fn add_rot3(&mut self, vec3: Vec3) {
        let rot3 = self.get_rot3();
        self.set_rot3(rot3 + vec3);
    }
    fn mul_rot2(&mut self, vec2: Vec2) {
        self.mul_rot3(vec2.to_vec3(1.0));
    }
    fn mul_rot3(&mut self, vec3: Vec3) {
        let rot3 = self.get_rot3();
        self.set_rot3(rot3 * vec3);
    }
    fn set_pos2(&mut self, vec2: Vec2) {
        let z = self.get_pos3()[2];
        self.set_pos3(vec2.to_vec3(z));
    }
    fn set_pos3(&mut self, Vec3);
    fn add_pos2(&mut self, vec2: Vec2) {
        self.add_pos3(vec2.to_vec3(0.0));
    }
    fn add_pos3(&mut self, vec3: Vec3) {
        let pos3 = self.get_pos3();
        self.set_pos3(pos3 + vec3);
    }
    fn mul_pos2(&mut self, vec2: Vec2) {
        self.mul_pos3(vec2.to_vec3(1.0));
    }
    fn mul_pos3(&mut self, vec3: Vec3) {
        let pos3 = self.get_pos3();
        self.set_pos3(pos3 * vec3);
    }
    fn set_vel2(&mut self, vec2: Vec2) {
        let z = self.get_vel3()[2];
        self.set_vel3(vec2.to_vec3(z));
    }
    fn set_vel3(&mut self, Vec3);
    fn add_vel2(&mut self, vec2: Vec2) {
        self.add_vel3(vec2.to_vec3(0.0));
    }
    fn add_vel3(&mut self, vec3: Vec3) {
        let vel3 = self.get_vel3();
        self.set_vel3(vel3 + vec3);
    }
    fn mul_vel2(&mut self, vec2: Vec2) {
        self.mul_vel3(vec2.to_vec3(1.0))
    }
    fn mul_vel3(&mut self, vec3: Vec3) {
        let vel3 = self.get_vel3();
        self.set_vel3(vel3 * vec3);
    }
    fn set_acc2(&mut self, vec2: Vec2) {
        let z = self.get_acc3()[2];
        self.set_acc3(vec2.to_vec3(z));
    }
    fn set_acc3(&mut self, Vec3);
    fn add_acc2(&mut self, vec2: Vec2) {
        self.add_acc3(vec2.to_vec3(0.0));
    }
    fn add_acc3(&mut self, vec3: Vec3) {
        let acc3 = self.get_acc3();
        self.set_acc3(acc3 + vec3);
    }
    fn mul_acc2(&mut self, vec2: Vec2) {
        self.mul_acc3(vec2.to_vec3(1.0));
    }
    fn mul_acc3(&mut self, vec3: Vec3) {
        let acc3 = self.get_acc3();
        self.set_acc3(acc3 * vec3);
    }
}
