extern crate polyclops;

use polyclops::{Window, WindowArgs, Entity, Vertex, Index, IDType};

pub static RAW_TEXTURE: &'static [u8] = include_bytes!("..\\assets\\texture.png");

fn main() {
    let mut manager = polyclops::init();

    let mut window: Window = Window::new(WindowArgs::Borderless("Polyclops".to_string()));

    let entity = Entity::new(&mut manager);

    let vertices: Vec<Vertex> = vec!{
        Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, -1.0, 0.0], [1.0, 0.0]),
        Vertex::new([-1.0, 1.0, 0.0], [0.0, 1.0]),
    };

    let indices: Vec<Index> = vec!{
        0, 1, 2,
        2, 1, 0,
    };

    {
        let mut frame = window.frame();
        frame.set_entity_vertices(&entity, vertices);
        frame.set_entity_indices(&entity, indices);
        frame.set_entity_texture(&entity, RAW_TEXTURE);
        frame.set_default_draw_parameters(&entity);
        frame.draw_entity(&entity);
        frame.end();
    }

    let mut t2 = Entity::new(&mut manager);

    t2.use_other_id(&entity, IDType::Vertex);
    t2.use_other_id(&entity, IDType::Index);
    t2.use_other_id(&entity, IDType::Texture);
    t2.use_other_id(&entity, IDType::DrawParameter);

    loop {
        window.poll_events();
        let mut frame = window.frame();
        frame.draw_entity(&t2);
        frame.end();
    }
}
