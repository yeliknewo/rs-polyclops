use std::sync::{Arc, RwLock};
use polyclops::{init, Window, WindowArgs, Game, World, BeingType, IDManager, WorldEvent};

mod cube;
mod cube_base;

pub fn main() {
    let mut manager = init();

    let mut window = Window::new(WindowArgs::Borderless("Cubes".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game = Game::<CubeBeingType>::new(&mut manager, thread_count, World::new(resolution), resolution);

    let mut starting_events = vec!(

    );

    game.run(&mut manager, &mut starting_events, &mut window);
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum CubeBeingType {
    Cube,
    CubeBase,
}

impl BeingType<CubeBeingType> for CubeBeingType {
    fn make_being(manager: &mut IDManager, being_type: CubeBeingType, events: &mut Vec<WorldEvent<CubeBeingType>>, window: &mut Window, game: &mut Game<CubeBeingType>, world: Arc<RwLock<World<CubeBeingType>>>) {
        let mut events = match being_type {
            CubeBeingType::Cube => cube::make_cube(manager, events, world.clone()),
            CubeBeingType::CubeBase => cube_base::make_cube_base(manager, events, world.clone()),
        };
        game.execute_events(manager, window, &mut events, world);
    }
}
