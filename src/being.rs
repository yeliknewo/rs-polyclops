use std::sync::{Arc, RwLock};
use std::hash::{Hash};

use utils::{ID, IDManager};
use graphics::{Entity, Window, Transforms};
use world::{World, WorldEvent};
use math::{Vec2, Vec3};
use game::{Game};

pub trait BeingType<T: BeingType<T>>: Send + Sync + Clone + Eq + PartialEq + Hash {
    fn make_being(&mut IDManager, T, &mut Vec<WorldEvent<T>>, &mut Window, &mut Game<T>, Arc<RwLock<World<T>>>);
}

pub trait Being<T: BeingType<T>>: Send + Sync {
    fn get_type(&self) -> T;
    fn get_id(&self) -> ID;
    fn get_entity(&self) -> &Entity;
    fn tick(&self, &World<T>, &f32, &Transforms) -> Vec<WorldEvent<T>>;
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

    fn get_entity_mut(&mut self) -> &mut Entity;
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
