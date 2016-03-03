use std::sync::{Arc, RwLock};
use polyclops::{init, Window, WindowArgs, Game, World, BeingType, IDManager, WorldEvent, Transforms};

mod cube;

mod cbt {
    #[derive(Clone, Hash, Eq, PartialEq)]
    pub enum CubeBeingType {
        Cube,
        CubeFace,
    }
}

use self::cbt::CubeBeingType as CBT;

pub fn main() {
    let mut manager = init();

    let mut window = Window::new(WindowArgs::Borderless("Cubes".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game = Game::<CBT>::new(&mut manager, thread_count, World::new(resolution), resolution);

    let mut starting_events = vec!(
        WorldEvent::NewBase(CBT::Cube),
        WorldEvent::NewBeing(CBT::Cube, vec!()),
    );

    game.run(&mut manager, &mut starting_events, &mut window);
}

impl BeingType<CBT> for CBT {
    fn make_being(manager: &mut IDManager, being_type: CBT, events: &mut Vec<WorldEvent<CBT>>, window: &mut Window, game: &mut Game<CBT>, world: Arc<RwLock<World<CBT>>>) {
        let mut results = match being_type {
            CBT::Cube => cube::make_cube(manager, events),
        };
        world.write().expect("Unable to Write World in Make Cube").add_being(results.0);
        game.execute_events(manager, window, &mut results.1, world);
    }

    fn make_base(manager: &mut IDManager, being_type: CBT, window: &mut Window, transforms: Arc<RwLock<Transforms>>, world: Arc<RwLock<World<CBT>>>) {
        let mut world = world.write().expect("Unable to Write World in Make Base in CBT");
        let aspect_ratio = world.get_aspect_ratio();
        world.set_base(being_type.clone(), match being_type {
            CBT::Cube => cube::make_base(manager, window, transforms, aspect_ratio),
        });
    }
}
