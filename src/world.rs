use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use glium::glutin::ElementState as GliumElementState;
use glium::glutin::MouseButton as GliumMouseButton;
use glium::glutin::VirtualKeyCode as GliumKeyCode;

use utils::{ID, EntityIDType};
use graphics::{Vertex, Index, DrawMethod};
use math::{Vec2, Vec3, Mat4};
use being::{Being, BeingType};
use keyboard::{Keyboard};
use being_args::{BeingArgs};

pub struct World<T: BeingType<T>> {
    beings: HashMap<ID, Arc<RwLock<Box<Being<T>>>>>,
    bases: HashMap<T, Arc<RwLock<Box<Being<T>>>>>,
    mouse_pos: Vec2,
    mouse_pos_world: Vec2,
    resolution: Vec2,
    aspect_ratio: f32,
    mouse_buttons: HashMap<GliumMouseButton, GliumElementState>,
    keyboard: Keyboard,
}

impl<T: BeingType<T>> World<T> {
    pub fn new(resolution: Vec2) -> World<T> {
        World {
            beings: HashMap::new(),
            bases: HashMap::new(),
            mouse_pos: Vec2::zero(),
            mouse_pos_world: Vec2::zero(),
            resolution: resolution,
            aspect_ratio: resolution[0] / resolution[1],
            mouse_buttons: HashMap::new(),
            keyboard: Keyboard::new(),
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

    pub fn set_base(&mut self, being_type: T, base: Box<Being<T>>) {
        self.bases.insert(being_type, Arc::new(RwLock::new(base)));
    }

    pub fn get_base(&self, being_type: T) -> Option<&Arc<RwLock<Box<Being<T>>>>> {
        self.bases.get(&being_type)
    }

    pub fn update_keyboard(&mut self, key: GliumKeyCode, state: GliumElementState) {
        self.keyboard.set_key_state(key, state);
    }

    pub fn update_mouse_button(&mut self, mouse_button: GliumMouseButton, element_state: GliumElementState) {
        self.mouse_buttons.insert(mouse_button, element_state);
    }

    pub fn update_resolution(&mut self, resolution: Vec2, aspect_ratio: f32) {
        self.resolution = resolution;
        self.aspect_ratio = aspect_ratio;
    }

    pub fn update_mouse_pos(&mut self, mouse_pos: Vec2) {
        self.mouse_pos = mouse_pos;
        self.mouse_pos_world = self.screen_to_world_point(mouse_pos);
    }

    pub fn get_key(&self, key: GliumKeyCode) -> GliumElementState {
        self.keyboard.is_key_down(key)
    }

    pub fn get_mouse_button(&self, mouse_button: GliumMouseButton) -> GliumElementState {
        match self.mouse_buttons.get(&mouse_button) {
            Some(state) => *state,
            None => GliumElementState::Released,
        }
    }

    pub fn get_resolution(&self) -> Vec2 {
        self.resolution
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn get_mouse_pos_world(&self) -> Vec2 {
        self.mouse_pos_world
    }

    pub fn screen_to_world_point(&self, vec2: Vec2) -> Vec2 {
        let x = 2.0 / self.resolution[0];
        (vec2 + self.resolution * (-1.0 / 2.0)) * Vec2::from([x, -x * self.aspect_ratio])
    }
}

pub fn get_rank_tick_after<T: BeingType<T>>(event: TickAfterEvent<T>) -> u32 {
    match event {
        TickAfterEvent::EndBeing(_) => 0,
        _ => 1,
    }
}

pub fn get_rank_tick<T: BeingType<T>>(event: TickEvent<T>) -> u32 {
    match event {
        TickEvent::NewBase(_) => 200,
        TickEvent::NewBeing(_, _) => 100,
        TickEvent::Sca2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 5,
            Vec2Event::Add(_) => 6,
            Vec2Event::Mul(_) => 7,
        },
        TickEvent::Sca3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 5,
            Vec3Event::Add(_) => 6,
            Vec3Event::Mul(_) => 7,
        },
        TickEvent::Rot2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 5,
            Vec2Event::Add(_) => 6,
            Vec2Event::Mul(_) => 7,
        },
        TickEvent::Rot3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 5,
            Vec3Event::Add(_) => 6,
            Vec3Event::Mul(_) => 7,
        },
        TickEvent::Pos2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 5,
            Vec2Event::Add(_) => 6,
            Vec2Event::Mul(_) => 7,
        },
        TickEvent::Pos3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 5,
            Vec3Event::Add(_) => 6,
            Vec3Event::Mul(_) => 7,
        },
        TickEvent::Vel2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 10,
            Vec2Event::Add(_) => 11,
            Vec2Event::Mul(_) => 12,
        },
        TickEvent::Vel3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 10,
            Vec3Event::Add(_) => 11,
            Vec3Event::Mul(_) => 12,
        },
        TickEvent::Acc2(_, vec2_event) => match vec2_event {
            Vec2Event::Set(_) => 15,
            Vec2Event::Add(_) => 16,
            Vec2Event::Mul(_) => 17,
        },
        TickEvent::Acc3(_, vec3_event) => match vec3_event {
            Vec3Event::Set(_) => 15,
            Vec3Event::Add(_) => 16,
            Vec3Event::Mul(_) => 17,
        },
        TickEvent::EntityID(_, _, event) => match event {
            EntityIDEvent::UseNewID(_) => 4,
            EntityIDEvent::UseOldID(_, _, _) => 3,
            EntityIDEvent::UseBaseID(_, _, _) => 2,
        },
        TickEvent::EntityIDBase(_, _, event) => match event {
            EntityIDEvent::UseNewID(_) => 4,
            EntityIDEvent::UseOldID(_, _, _) => 3,
            EntityIDEvent::UseBaseID(_, _, _) => 2,
        },
        TickEvent::Transform(_, _, _) => 1,
        TickEvent::TransformBase(_, _, _) => 1,
    }
}

#[derive(Clone)]
pub enum WorldEvent<T: BeingType<T>> {
    Tick(TickEvent<T>),
    TickAfter(TickAfterEvent<T>),
}

#[derive(Clone)]
pub enum TickAfterEvent<T: BeingType<T>> {
    Entity(ID, u32, EntityGraphicsEvent),
    EntityBase(T, u32, EntityGraphicsEvent),
    EndBeing(ID),
}

#[derive(Clone)]
pub enum TickEvent<T: BeingType<T>> {
    NewBeing(T, BeingArgs),
    NewBase(T),
    Sca2(ID, Vec2Event),
    Sca3(ID, Vec3Event),
    Rot2(ID, Vec2Event),
    Rot3(ID, Vec3Event),
    Pos2(ID, Vec2Event),
    Pos3(ID, Vec3Event),
    Vel2(ID, Vec2Event),
    Vel3(ID, Vec3Event),
    Acc2(ID, Vec2Event),
    Acc3(ID, Vec3Event),
    EntityID(ID, u32, EntityIDEvent<T>),
    EntityIDBase(T, u32, EntityIDEvent<T>),
    Transform(ID, u32, TransformEvent),
    TransformBase(T, u32, TransformEvent),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum TransformEvent {
    Perspective(Mat4, Mat4),
    View(Mat4, Mat4),
    Model(Mat4, Mat4),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum EntityIDEvent<T: BeingType<T>> {
    UseNewID(Vec<EntityIDType>),
    UseOldID(ID, u32, Vec<EntityIDType>),
    UseBaseID(T, u32, Vec<EntityIDType>),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum EntityGraphicsEvent {
    Vertices(Vec<Vertex>),
    Indices(Vec<Index>),
    Texture(&'static [u8]),
    DrawMethod(DrawMethod),
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
