#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;
extern crate rand;

mod graphics;

use graphics::{Window, WindowArgs, Triangle, Vertex};

mod utils;

use utils::Index;

pub static RAW_TEXTURE: &'static [u8] = include_bytes!("..\\assets\\texture.png");

fn main() {
    graphics::init_vertex();

    let mut window: Window = Window::new(WindowArgs::Borderless("Polyclops".to_string()));

    let triangle = Triangle::new();

    let vertices: Vec<Vertex> = vec!{
        Vertex::new([-1.0, -1.0, -1.0], [0.0, 0.0]),
        Vertex::new([1.0, -1.0, -1.0], [1.0, 0.0]),
        Vertex::new([-1.0, 1.0, -1.0], [0.0, 1.0]),
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
        frame.set_triangle_vertices(&triangle, vertices);
        frame.set_triangle_indices(&triangle, indices);
        frame.set_triangle_texture(&triangle, texture);
        frame.set_draw_parameters(&triangle, draw_parameters);
        frame.draw_triangle(&triangle);
        frame.end();
    }

    let mut i = 0;
    loop {
        window.poll_events();
        let mut frame = window.frame();
        frame.draw_triangle(&triangle);
        frame.end();
        println!("{}", i);
        i += 1;
    }
}
