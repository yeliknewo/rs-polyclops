use std::sync::{Arc, RwLock};
use polyclops::{init, Window, WindowArgs, Game, World, BeingType, IDManager, WorldEvent, TickEvent};

mod tile;

use self::tile::Tile;

pub fn main() {
    let manager = init();

    let mut window = Window::new(WindowArgs::Borderless("iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game: Game<IBT> = Game::<IBT>::new(manager, thread_count, World::new(resolution), resolution);

    game.run(vec!(WorldEvent::TickEvent(TickEvent::NewBase(IBT::Tile)), WorldEvent::TickEvent(TickEvent::NewBeing(IBT::Tile))), &mut window);
}

use self::iso_being_type::IsoBeingType as IBT;

pub mod iso_being_type {
    #[derive(Clone, Hash, Eq, PartialEq)]
    pub enum IsoBeingType {
        Tile,
    }
}

impl BeingType<IBT> for IBT {
    fn make_being(manager: Arc<RwLock<IDManager>>, being_type: IBT, world: Arc<RwLock<World<IBT>>>) -> Vec<WorldEvent<IBT>> {
        vec!()
    }

    fn make_base(manager: Arc<RwLock<IDManager>>, being_type: IBT, world: Arc<RwLock<World<IBT>>>) -> Vec<WorldEvent<IBT>> {
        match being_type {
            IBT::Tile => {
                let tile = Tile::new_base(manager);
            },
        }
        vec!()
    }
}
