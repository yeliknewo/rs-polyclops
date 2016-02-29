use std::collections::{HashMap};
use glium::glutin::{ElementState, VirtualKeyCode};

pub struct Keyboard {
    keys: HashMap<VirtualKeyCode, ElementState>,
}

impl Keyboard {
    pub fn new() -> Keyboard{
        Keyboard{
            keys: HashMap::new(),
        }
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> ElementState {
        match self.keys.get(&key) {
            Some(down) => *down,
            None => ElementState::Released,
        }
    }

    pub fn set_key_state(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.keys.insert(key, state);
    }
}
