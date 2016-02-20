#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;
extern crate rand;
extern crate nalgebra;

mod graphics;
mod utils;

use graphics::{Window, WindowArgs, Entity, Vertex};

use utils::{Index, IDManager, IDType};

pub static RAW_TEXTURE: &'static [u8] = include_bytes!("..\\assets\\texture.png");

fn main() {
    graphics::init_vertex();
    let mut manager = IDManager::new();

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

    let texture = image::load_from_memory(RAW_TEXTURE).expect("Error Loading Image").to_rgba();

    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    {
        let mut frame = window.frame();
        frame.set_entity_vertices(&entity, vertices);
        frame.set_entity_indices(&entity, indices);
        frame.set_entity_texture(&entity, texture);
        frame.set_entity_draw_parameters(&entity, draw_parameters);
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
