use std::collections::{HashMap};
use std::fmt::{Display, Formatter, Error};
use std::sync::{Arc, RwLock};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct ID {
    id: IDSize,
}

impl ID {
    pub fn new(manager: Arc<RwLock<IDManager>>, id_type: IDType) -> ID {
        ID {
            id: manager.write().expect("Unable to Write Manager in New ID").get_id(id_type),
        }
    }

    pub fn get_id(&self) -> IDSize {
        self.id
    }
}

pub const UNSET: ID = ID{
    id: UNSET_ID,
};

pub const UNSET_ID: IDSize = 0;

impl Display for ID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.id)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum IDType {
    Entity(EntityIDType),
    EntityContainer,
    Being,
    World,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum EntityIDType {
    Vertex,
    Index,
    Texture,
    DrawParameter,
    Perspective,
    View,
    Model,
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
            None => 1,
        };
        self.map.insert(id_type, id + 1);
        id
    }
}

pub type IDSize = u32;
