use polyclops::{init, Window, WindowArgs, Game, World, BeingType};

pub fn main() {
    let mut manager = init();

    let mut window = Window::new(WindowArgs::Borderless("iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game: Game<IBT> = Game::<IBT>::new(&mut manager, thread_count, World::new(resolution), resolution);
}

use self::iso_being_type::IsoBeingType as IBT;

mod iso_being_type {
    #[derive(Clone, Hash, Eq, PartialEq)]
    enum IsoBeingType {
        Tile,
    }
}

impl BeingType<IBT> for IBT {

}
