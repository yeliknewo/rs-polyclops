use std::collections::HashMap;

pub type Index = u32;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct ID {
    id: IDSize,
}

impl ID {
    pub fn new(manager: &mut IDManager, id_type: IDType) -> ID {
        ID {
            id: manager.get_id(id_type),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum IDType {
    Vertex = 0,
    Index = 1,
    Texture = 2,
    DrawParameter = 3,
    Transform = 4,
}

pub struct IDManager {
    map: HashMap<IDType, IDSize>,
}

impl IDManager {
    pub fn new() -> IDManager {
        IDManager {
            map: HashMap::new(),
        }
    }

    fn get_id(&mut self, id_type: IDType) -> IDSize {
        let id = match self.map.get(&id_type) {
            Some(id) => *id,
            None => 0,
        };
        self.map.insert(id_type, id + 1);
        id
    }
}

pub type IDSize = u32;
