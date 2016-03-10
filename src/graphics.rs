use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::texture::texture2d::{Texture2d};
use glium::texture::{RawImage2d};
use glium::glutin::{WindowBuilder, get_primary_monitor};
use glium::{Surface, DisplayBuild, Program, VertexBuffer, IndexBuffer, DrawParameters};
use glium;
use image::{load_from_memory};
use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use math::{Mat4, Vec2, Vec3, Vec4};
use utils::{ID, IDType, EntityIDType, IDManager};

pub type Index = u32;

pub struct Window {
    facade: GlutinFacade,
    program: Program,
    texture_buffers: HashMap<ID, Texture2d>,
    vertex_buffers: HashMap<ID, VertexBuffer<Vertex>>,
    index_buffers: HashMap<ID, IndexBuffer<Index>>,
    draw_parameters: HashMap<ID, DrawParameters<'static>>,
    resolution: (u32, u32),
}

impl Window {
    pub fn new(args: WindowArgs) -> Window {
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coord;
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            out vec2 v_tex_coord;

            void main() {
                v_tex_coord = tex_coord;
                gl_Position = perspective * view * model * vec4(position, 1.0);
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

        let facade = match args {
            WindowArgs::Windowed(width, height, title) => {
                let facade = WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(width, height)
                    .with_decorations(true)
                    .with_depth_buffer(24)
                    .with_vsync()
                    .build_glium()
                    .expect("Unable to make Facade");
                facade.get_window()
                    .expect("Unable to find the Window")
                    .set_position(((resolution.0 - width) / 2) as i32, ((resolution.1 - height) / 2) as i32);
                facade
            },
            WindowArgs::Borderless(title) => {
                let facade = WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(resolution.0, resolution.1)
                    .with_decorations(false)
                    .with_depth_buffer(24)
                    .with_vsync()
                    .build_glium()
                    .expect("Unable to make Facade");
                facade.get_window()
                    .expect("Unable to find Window")
                    .set_position(0, 0);
                facade
            },
        };
        Window {
            program: Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None).expect("Unable to make Shader Program"),
            facade: facade,
            texture_buffers: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            draw_parameters: HashMap::new(),
            resolution: resolution,
        }
    }

    pub fn get_resolution_vec2(&self) -> Vec2 {
        Vec2::from([self.resolution.0 as f32, self.resolution.1 as f32])
    }

    pub fn frame(&mut self) -> Frame {
        Frame::new(&mut self.facade, &mut self.program, &mut self.texture_buffers, &mut self.vertex_buffers, &mut self.index_buffers, &mut self.draw_parameters)
    }

    pub fn poll_events(&self) -> PollEventsIter {
        self.facade.poll_events()
    }

    pub fn set_vertices(&mut self, entity: &Arc<RwLock<Entity>>, vertices: Vec<Vertex>) {
        self.vertex_buffers.insert(entity.read().expect("Unable to Read Entity in Set Vertices").vertex_id, VertexBuffer::new(&self.facade, &vertices).expect("Failed to Create Vertex Buffer"));
    }

    pub fn set_indices(&mut self, entity: &Arc<RwLock<Entity>>, indices: Vec<Index>) {
        self.index_buffers.insert(entity.read().expect("Unable to Read Entity in Set Indices").index_id, IndexBuffer::new(&self.facade, glium::index::PrimitiveType::TrianglesList, &indices).expect("Failed to Create Index Buffer"));
    }

    pub fn set_texture(&mut self, entity: &Arc<RwLock<Entity>>, data: &[u8]) {
        let texture = load_from_memory(data).expect("Error Loading Image").to_rgba();
        self.texture_buffers.insert(entity.read().expect("Unable to Read Entity in Set Texture").texture_id, Texture2d::new(&self.facade, RawImage2d::from_raw_rgba_reversed(texture.clone().into_raw(), texture.dimensions())).expect("Unable to make Texture"));
    }

    pub fn set_draw_parameters(&mut self, entity: &Arc<RwLock<Entity>>, draw_parameters: DrawParameters<'static>) {
        self.draw_parameters.insert(entity.read().expect("Unable to Read Entity in Set Draw Parameters").draw_parameters_id, draw_parameters);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum DrawMethod {
    Both(DepthTestMethod, CullingMethod),
    Depth(DepthTestMethod),
    Culling(CullingMethod),
    Neither,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum DepthTestMethod {
    IfLess,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum CullingMethod {
    Clockwise,
    CounterClockwise,
}

pub fn method_to_parameters(method: DrawMethod) -> DrawParameters<'static> {
    match method {
        DrawMethod::Both(depth, cull) => {
            let depth_glium = match depth {
                DepthTestMethod::IfLess => glium::draw_parameters::DepthTest::IfLess,
            };
            let cull_glium = match cull {
                CullingMethod::Clockwise => glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                CullingMethod::CounterClockwise => glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            };
            glium::DrawParameters {
                depth: glium::Depth {
                    test: depth_glium,
                    write: true,
                    .. Default::default()
                },
                backface_culling: cull_glium,
                .. Default::default()
            }
        },
        DrawMethod::Depth(depth) => {
            let depth_glium = match depth {
                DepthTestMethod::IfLess => glium::draw_parameters::DepthTest::IfLess,
            };
            glium::DrawParameters {
                depth: glium::Depth {
                    test: depth_glium,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            }
        },
        DrawMethod::Culling(cull) => {
            let cull_glium = match cull {
                CullingMethod::Clockwise => glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                CullingMethod::CounterClockwise => glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            };
            glium::DrawParameters {
                backface_culling: cull_glium,
                .. Default::default()
            }
        },
        DrawMethod::Neither => {
            glium::DrawParameters {
                .. Default::default()
            }
        },
    }
}

pub enum WindowArgs {
    Windowed(u32, u32, String),
    Borderless(String),
}

pub struct Frame<'a> {
    program: &'a mut Program,
    texture_buffers: &'a mut HashMap<ID, Texture2d>,
    vertex_buffers: &'a mut HashMap<ID, VertexBuffer<Vertex>>,
    index_buffers: &'a mut HashMap<ID, IndexBuffer<Index>>,
    draw_parameters: &'a mut HashMap<ID, DrawParameters<'static>>,
    frame: glium::Frame,
}

impl<'a> Frame<'a> {
    fn new(
        facade: &'a mut GlutinFacade,
        program: &'a mut Program,
        texture_buffers: &'a mut HashMap<ID, Texture2d>,
        vertex_buffers: &'a mut HashMap<ID, VertexBuffer<Vertex>>,
        index_buffers: &'a mut HashMap<ID, IndexBuffer<Index>>,
        draw_parameters: &'a mut HashMap<ID, DrawParameters<'static>>,
    ) -> Frame<'a> {
        let mut frame = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            program: program,
            texture_buffers: texture_buffers,
            vertex_buffers: vertex_buffers,
            index_buffers: index_buffers,
            draw_parameters: draw_parameters,
        }
    }

    pub fn draw_entity(&mut self, entity_arc: &Arc<RwLock<Entity>>, transforms: &Arc<RwLock<Transforms>>) {
        let entity = entity_arc.read().expect("Unable to Read Entity in Draw Entity");
        self.frame.draw(
            &self.vertex_buffers[&entity.vertex_id],
            &self.index_buffers[&entity.index_id],
            &self.program,
            &uniform!(
                tex: &self.texture_buffers[&entity.texture_id],
                perspective: transforms.read().expect("Unable to Read Transforms in Draw Entity in Frame").get_perspective_matrix(&entity),
                view: transforms.read().expect("Unable to Read Transforms in Draw Entity In Frame").get_view_matrix(&entity),
                model: transforms.read().expect("Unable to Read Transforms in Draw Entity in Frame").get_model_matrix(&entity),
            ),
            &self.draw_parameters[&entity.draw_parameters_id])
            .expect("Unable to draw Entity");
    }

    pub fn end(self) {
        self.frame.finish().expect("Unable to Finish Frame");
    }
}

pub struct Transforms {
    perspective_mat4s: Arc<RwLock<HashMap<ID, Mat4>>>,
    perspective_mat4s_inverse: Arc<RwLock<HashMap<ID, Mat4>>>,
    view_mat4s: Arc<RwLock<HashMap<ID, Mat4>>>,
    view_mat4s_inverse: Arc<RwLock<HashMap<ID, Mat4>>>,
    model_mat4s: Arc<RwLock<HashMap<ID, Mat4>>>,
    model_mat4s_inverse: Arc<RwLock<HashMap<ID, Mat4>>>,
}

impl Transforms {
    pub fn new() -> Transforms {
        Transforms {
            perspective_mat4s: Arc::new(RwLock::new(HashMap::new())),
            perspective_mat4s_inverse: Arc::new(RwLock::new(HashMap::new())),
            view_mat4s: Arc::new(RwLock::new(HashMap::new())),
            view_mat4s_inverse: Arc::new(RwLock::new(HashMap::new())),
            model_mat4s: Arc::new(RwLock::new(HashMap::new())),
            model_mat4s_inverse: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn backwards2(&self, vec2: Vec2, entity: &Entity) -> Vec2 {
        Vec2::from(self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec2.to_vec4(0.0, 0.0))
    }

    pub fn backwards3(&self, vec3: Vec3, entity: &Entity) -> Vec3 {
        Vec3::from(self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec3.to_vec4(0.0))
    }

    pub fn backwards4(&self, vec4: Vec4, entity: &Entity) -> Vec4 {
        self.get_perspective_inverse(entity) * self.get_view_inverse(entity) * self.get_model_inverse(entity) * vec4
    }

    pub fn get_perspective_matrix(&self, entity: &Entity) -> Mat4 {
        self.perspective_mat4s.read().expect("Unable to Read Perspective Matrix in Transforms")[&entity.perspective_id]
    }

    pub fn get_perspective_inverse(&self, entity: &Entity) -> Mat4 {
        self.perspective_mat4s_inverse.read().expect("Unable to Read Perspective Inverse in Transforms")[&entity.perspective_id]
    }

    pub fn set_perspective_matrix(&self, entity: &Arc<RwLock<Entity>>, perspective: Mat4, inverse: Mat4) {
        self.perspective_mat4s.write().expect("Unable to Write Perspective Matrix in Set Perspective Matrix in Transforms").insert(entity.read().expect("Unable to Read Entity in Set Perspective Matrix").perspective_id, perspective);
        self.perspective_mat4s_inverse.write().expect("Unable to Write Perspective Inverse in Set Perspective Matrix in Transforms").insert(entity.read().expect("Unable to Read Entity in Set Perspective Matrix").perspective_id, inverse);
    }

    pub fn get_view_matrix(&self, entity: &Entity) -> Mat4 {
        self.view_mat4s.read().expect("Unable to Read View Matrix in Get View Matrix in Transforms")[&entity.view_id]
    }

    pub fn get_view_inverse(&self, entity: &Entity) -> Mat4 {
        self.view_mat4s_inverse.read().expect("Unable to Read View Inverse in Get View Inverse in Transforms")[&entity.view_id]
    }

    pub fn set_view_matrix(&self, entity: &Arc<RwLock<Entity>>, view: Mat4, inverse: Mat4) {
        self.view_mat4s.write().expect("Unable to Write View Matrix in Set View Matrix in Transforms").insert(entity.read().expect("Unable to Read Entity in Set View Matrix").view_id, view);
        self.view_mat4s_inverse.write().expect("Unable to Write View Inverse in Set View Matrix in Transforms").insert(entity.read().expect("Unable to Read Entity in Set View Matrix").view_id, inverse);
    }

    pub fn get_model_matrix(&self, entity: &Entity) -> Mat4 {
        self.model_mat4s.read().expect("Unable to Read Model Matrix in Get Model Matrix in Transforms")[&entity.model_id]
    }

    pub fn get_model_inverse(&self, entity: &Entity) -> Mat4 {
        self.model_mat4s_inverse.read().expect("Unable to Read Model Inverse in Get Model Inverse in Transforms")[&entity.model_id]
    }

    pub fn set_model_matrix(&self, entity: &Arc<RwLock<Entity>>, model: Mat4, inverse: Mat4) {
        self.model_mat4s.write().expect("Unable to Write Model Matrix in Set Model Matrix in Transforms").insert(entity.read().expect("Unable to Read Entity in Set Model Matrix").model_id, model);
        self.model_mat4s_inverse.write().expect("Unable to Write Model Inverse in Set Model Matrix in Transforms").insert(entity.read().expect("Unable to Read Entity in Set Model Matrix").model_id, inverse);
    }
}

pub struct Entity {
    texture_id: ID,
    vertex_id: ID,
    index_id: ID,
    draw_parameters_id: ID,
    perspective_id: ID,
    view_id: ID,
    model_id: ID,
}

impl Entity {
    pub fn new(manager: &mut IDManager) -> Entity {
        Entity {
            texture_id: ID::new(manager, IDType::Entity(EntityIDType::Texture)),
            vertex_id: ID::new(manager, IDType::Entity(EntityIDType::Vertex)),
            index_id: ID::new(manager, IDType::Entity(EntityIDType::Index)),
            draw_parameters_id: ID::new(manager, IDType::Entity(EntityIDType::DrawParameter)),
            perspective_id: ID::new(manager, IDType::Entity(EntityIDType::Perspective)),
            view_id: ID::new(manager, IDType::Entity(EntityIDType::View)),
            model_id: ID::new(manager, IDType::Entity(EntityIDType::Model)),
        }
    }

    pub fn new_from(entity: &Entity) -> Entity {
        Entity {
            texture_id: entity.texture_id,
            vertex_id: entity.vertex_id,
            index_id: entity.index_id,
            draw_parameters_id: entity.draw_parameters_id,
            perspective_id: entity.perspective_id,
            view_id: entity.view_id,
            model_id: entity.model_id,
        }
    }

    pub fn use_other_id(&mut self, other_arc: &Arc<RwLock<Entity>>, id_type: EntityIDType) {
        let other = other_arc.read().expect("Unable to Read Other in Use Other ID");
        match id_type {
            EntityIDType::Vertex => {
                self.vertex_id = other.vertex_id;
            },
            EntityIDType::Index => {
                self.index_id = other.index_id;
            },
            EntityIDType::Texture => {
                self.texture_id = other.texture_id;
            },
            EntityIDType::DrawParameter => {
                self.draw_parameters_id = other.draw_parameters_id;
            },
            EntityIDType::Perspective => {
                self.perspective_id = other.perspective_id;
            },
            EntityIDType::View => {
                self.view_id = other.view_id;
            },
            EntityIDType::Model => {
                self.model_id = other.model_id;
            }
        };
    }

    pub fn use_new_id(&mut self, manager: &mut IDManager, id_type: EntityIDType) {
        let id = ID::new(manager, IDType::Entity(id_type));
        match id_type {
            EntityIDType::Vertex => {
                self.vertex_id = id;
            },
            EntityIDType::Index => {
                self.index_id = id;
            },
            EntityIDType::Texture => {
                self.texture_id = id;
            },
            EntityIDType::DrawParameter => {
                self.draw_parameters_id = id;
            },
            EntityIDType::Perspective => {
                self.perspective_id = id;
            },
            EntityIDType::View => {
                self.view_id = id;
            },
            EntityIDType::Model => {
                self.model_id = id;
            },
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

impl From<Vec2> for Vertex {
    fn from(other: Vec2) -> Vertex {
        Vertex::new([other[0], other[1], 0.0], other.get_vals())
    }
}

impl From<Vec3> for Vertex {
    fn from(other: Vec3) -> Vertex {
        Vertex::new(other.get_vals(), [other[0], other[1]])
    }
}

pub fn init_vertex() {
    implement_vertex!(Vertex, position, tex_coord);
}
