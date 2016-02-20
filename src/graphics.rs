use glium::backend::glutin_backend::{GlutinFacade};
use glium::texture::texture2d::{Texture2d};
use glium::texture::{RawImage2d};
use glium::glutin::{WindowBuilder, get_primary_monitor};
use glium::{Surface, DisplayBuild, Program, VertexBuffer, IndexBuffer, DrawParameters};
use glium;

use image::{RgbaImage};

use std::collections::HashMap;

use utils::{Index, ID};

pub struct Window {
    facade: GlutinFacade,
    program: Program,
    texture_buffers: HashMap<ID, Texture2d>,
    vertex_buffers: HashMap<ID, VertexBuffer<Vertex>>,
    index_buffers: HashMap<ID, IndexBuffer<Index>>,
    draw_parameters: HashMap<ID, DrawParameters<'static>>,
}

impl Window {
    pub fn new(args: WindowArgs) -> Window {
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coord;

            out vec2 v_tex_coord;

            void main() {
                v_tex_coord = tex_coord;
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coord;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coord);
            }
        "#;

        let resolution: (u32, u32) = get_primary_monitor().get_dimensions();

        match args {
            WindowArgs::Windowed(width, height, title) => {
                let facade = WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(width, height)
                    .with_decorations(true)
                    .with_depth_buffer(24)
                    .build_glium()
                    .expect("Unable to make Facade");
                facade.get_window()
                    .expect("Unable to find the Window")
                    .set_position(((resolution.0 - width) / 2) as i32, ((resolution.1 - height) / 2) as i32);
                Window {
                    program: Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None).expect("Unable to make Shader Program"),
                    facade: facade,
                    texture_buffers: HashMap::new(),
                    vertex_buffers: HashMap::new(),
                    index_buffers: HashMap::new(),
                    draw_parameters: HashMap::new(),
                }
            },
            WindowArgs::Borderless(title) => {
                let facade = WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(resolution.0, resolution.1)
                    .with_decorations(false)
                    .with_depth_buffer(24)
                    .build_glium()
                    .expect("Unable to make Facade");
                facade.get_window()
                    .expect("Unable to find Window")
                    .set_position(0, 0);
                Window {
                    program: Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None).expect("Unable to make Shader Program"),
                    facade: facade,
                    texture_buffers: HashMap::new(),
                    vertex_buffers: HashMap::new(),
                    index_buffers: HashMap::new(),
                    draw_parameters: HashMap::new(),
                }
            },
        }
    }

    pub fn frame(&mut self) -> Frame {
        Frame::new(&mut self.facade, &mut self.program, &mut self.texture_buffers, &mut self.vertex_buffers, &mut self.index_buffers, &mut self.draw_parameters)
    }

    pub fn poll_events(&mut self) {
        for event in self.facade.poll_events() {
            match event {
                _ => (),
            }
        }
    }
}

pub enum WindowArgs {
    Windowed(u32, u32, String),
    Borderless(String),
}

pub struct Frame<'a> {
    facade: &'a mut GlutinFacade,
    program: &'a mut Program,
    texture_buffers: &'a mut HashMap<ID, Texture2d>,
    vertex_buffers: &'a mut HashMap<ID, VertexBuffer<Vertex>>,
    index_buffers: &'a mut HashMap<ID, IndexBuffer<Index>>,
    draw_parameters: &'a mut HashMap<ID, DrawParameters<'static>>,
    frame: glium::Frame,
}

impl<'a> Frame<'a> {
    fn new(facade: &'a mut GlutinFacade, program: &'a mut Program, texture_buffers: &'a mut HashMap<ID, Texture2d>, vertex_buffers: &'a mut HashMap<ID, VertexBuffer<Vertex>>, index_buffers: &'a mut HashMap<ID, IndexBuffer<Index>>, draw_parameters: &'a mut HashMap<ID, DrawParameters<'static>>) -> Frame<'a> {
        let mut frame = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            facade: facade,
            program: program,
            texture_buffers: texture_buffers,
            vertex_buffers: vertex_buffers,
            index_buffers: index_buffers,
            draw_parameters: draw_parameters,
        }
    }

    pub fn draw_triangle(&mut self, triangle: &Triangle) {
        self.frame.draw(&self.vertex_buffers[&triangle.vertex_id], &self.index_buffers[&triangle.index_id], &self.program, &uniform!(tex: &self.texture_buffers[&triangle.texture_id]), &self.draw_parameters[&triangle.draw_parameters_id]).expect("Unable to draw Triangle");
    }

    pub fn set_triangle_vertices(&mut self, triangle: &Triangle, vertices: Vec<Vertex>) {
        self.vertex_buffers.insert(triangle.vertex_id, VertexBuffer::new(self.facade, &vertices).expect("Failed to Create Vertex Buffer"));
    }

    pub fn set_triangle_texture(&mut self, triangle: &Triangle, texture: RgbaImage) {
        self.texture_buffers.insert(triangle.texture_id, Texture2d::new(self.facade, RawImage2d::from_raw_rgba_reversed(texture.clone().into_raw(), texture.dimensions())).expect("Unable to make Texture"));
    }

    pub fn set_triangle_indices(&mut self, triangle: &Triangle, indices: Vec<Index>) {
        self.index_buffers.insert(triangle.index_id, IndexBuffer::new(self.facade, glium::index::PrimitiveType::TrianglesList, &indices).expect("Failed to Create Index Buffer"));
    }

    pub fn set_draw_parameters(&mut self, triangle: &Triangle, draw_parameters: DrawParameters<'static>) {
        self.draw_parameters.insert(triangle.draw_parameters_id, draw_parameters);
    }

    pub fn end(self) {
        self.frame.finish().expect("Unable to Finish Frame");
    }
}

pub struct Triangle {
    texture_id: ID,
    vertex_id: ID,
    index_id: ID,
    draw_parameters_id: ID,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            texture_id: 0,
            vertex_id: 0,
            index_id: 0,
            draw_parameters_id: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coord: [f32; 2]) -> Vertex {
        Vertex{
            position: position,
            tex_coord: tex_coord,
        }
    }
}

pub fn init_vertex() {
    implement_vertex!(Vertex, position, tex_coord);
}
