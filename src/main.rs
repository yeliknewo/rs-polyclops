extern crate polyclops;

use polyclops::{Window, WindowArgs, Entity, Vertex, Index, Mat4, Vec2, Vec3, IDType};

pub static RAW_TEXTURE: &'static [u8] = include_bytes!("..\\assets\\texture.png");

fn main() {
    let mut manager = polyclops::init();

    let mut window: Window = Window::new(WindowArgs::Borderless("Polyclops".to_string()));

    let entity = Entity::new(&mut manager);

    let vertices: Vec<Vertex> = vec!{
        Vertex::new([-1.0, -1.0, -1.0], [0.0, 0.0]),
        Vertex::new([1.0, -1.0, -1.0], [1.0, 0.0]),
        Vertex::new([-1.0, 1.0, -1.0], [0.0, 1.0]),
    };

    let indices: Vec<Index> = vec!{
        0, 1, 2,
        2, 1, 0,
    };

    let mut e2 = Entity::new_from(&entity);

    e2.use_new_id(&mut manager, IDType::Vertex);
    e2.use_new_id(&mut manager, IDType::Index);

    {
        let mut frame = window.frame();

        frame.set_vertices(&entity, vertices);
        frame.set_indices(&entity, indices);
        frame.set_texture(&entity, RAW_TEXTURE);
        frame.set_default_draw_parameters(&entity);
        frame.set_perspective_matrix(&entity, Mat4::perspective(0.1, 100.0, 90.0, 16.0 / 9.0));
        frame.set_view_matrix(&entity, Mat4::view(0.0, 0.0, Vec3::from_vals([0.0, 0.0, 0.0])));
        frame.set_model_matrix(&entity, Mat4::translation_from_vec3(Vec3::from_vals([0.0, 0.0, -1.0])));
        frame.draw_entity(&entity);

        frame.set_entity_as_polygon(&e2, vec!(Vec2::from_vals([0.0, 0.0]), Vec2::from_vals([0.0, 1.0]), Vec2::from_vals([1.0, 1.0])));

        frame.end();
    }

    loop {
        window.poll_events();
        let mut frame = window.frame();
        //frame.draw_entity(&entity);
        frame.draw_entity(&e2);
        frame.end();
    }
}
