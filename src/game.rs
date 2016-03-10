use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use glium::glutin::Event as WindowEvent;
use glium::glutin::ElementState as GliumElementState;
use glium::glutin::MouseButton as GliumMouseButton;
use glium::glutin::VirtualKeyCode as GliumKeyCode;
use scoped_threadpool::{Pool};
use time::{precise_time_s};

use utils::{ID, IDManager, IDType};
use world::{World, WorldEvent, EntityEvent, TickAfterEvent, TransformEvent, Vec2Event, Vec3Event, get_rank};
use graphics::{Window, Transforms, method_to_parameters};
use being::{BeingType, Being};
use math::{Vec2};
use keyboard::{Keyboard};

pub struct Game<T: BeingType<T>> {
    worlds: HashMap<ID, Arc<RwLock<World<T>>>>,
    active_world_id: ID,
    thread_pool: Pool,
    resolution: Vec2,
    aspect_ratio: f32,
    mouse_pos: Vec2,
    keyboard: Keyboard,
    mouse_buttons: HashMap<GliumMouseButton, GliumElementState>,
    transforms: Arc<RwLock<Transforms>>,
    manager: Arc<RwLock<IDManager>>,
    ranked_events: Arc<RwLock<HashMap<u32, Arc<RwLock<Vec<WorldEvent<T>>>>>>>,
    ranks: Arc<RwLock<Vec<u32>>>,
}

impl<T: BeingType<T>> Game<T> {
    pub fn new(manager: IDManager, thread_count: u32, active_world: World<T>, resolution: Vec2) -> Game<T> {
        let manager = Arc::new(RwLock::new(manager));
        let id = ID::new(manager.clone(), IDType::World);
        let mut map = HashMap::new();
        map.insert(id, Arc::new(RwLock::new(active_world)));
        let keyboard = Keyboard::new();
        Game {
            worlds: map,
            active_world_id: id,
            thread_pool: Pool::new(thread_count),
            aspect_ratio: resolution[0] / resolution[1],
            resolution: resolution,
            mouse_pos: Vec2::zero(),
            keyboard: keyboard,
            mouse_buttons: HashMap::new(),
            transforms: Arc::new(RwLock::new(Transforms::new())),
            manager: manager,
            ranked_events: Arc::new(RwLock::new(HashMap::new())),
            ranks: Arc::new(RwLock::new(vec!()))
        }
    }

    fn pause(&mut self) {
        println!("Paused");
    }

    fn resume(&mut self) {
        println!("Resumed");
    }

    fn update_keyboard(&mut self, key_code: GliumKeyCode, element_state: GliumElementState) {
        self.keyboard.set_key_state(key_code, element_state);
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Keyboard").write().expect("Unable to Write Active World in Update Mouse Pos").update_keyboard(key_code, element_state);
    }

    fn update_mouse_button(&mut self, mouse_button: GliumMouseButton, element_state: GliumElementState, ) {
        self.mouse_buttons.insert(mouse_button, element_state);
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Mouse Button").write().expect("Unable to Write Active World in Update Mouse Button").update_mouse_button(mouse_button, element_state);
    }

    fn update_mouse_pos(&mut self, mouse_pos: (i32, i32)) {
        let x = mouse_pos.0 as f32;
        let y = mouse_pos.1 as f32;
        self.mouse_pos = Vec2::from([x, y]);
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Mouse Pos").write().expect("Unable to Write Active World in Update Mouse Pos").update_mouse_pos(self.mouse_pos);
    }

    fn update_resolution(&mut self, resolution: (u32, u32)) {
        let width = resolution.0 as f32;
        let height = resolution.1 as f32;
        self.resolution = Vec2::from([width, height]);
        self.aspect_ratio = width / height;
        self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Update Resolution").write().expect("Unable to Write Active World in Update Resolution").update_resolution(self.resolution, self.aspect_ratio);
    }

    pub fn run(&mut self, starting_events: Vec<WorldEvent<T>>, starting_tick_after_events: Vec<TickAfterEvent<T>>, window: &mut Window) {
        self.starting_events(window, 0.0, Arc::new(RwLock::new(starting_events)), starting_tick_after_events);

        let tps: f64 = 60.0;
        let tps_s: f64 = 1.0 / tps;

        let mut last_time: f64 = precise_time_s();
        let mut delta_time: f64 = 0.0;

        let mut i: f64 = last_time;

        let mut frames: u64 = 0;
        let mut ticks: u64 = 0;

        loop {
            let now = precise_time_s();
            delta_time += now - last_time;
            last_time = now;
            while delta_time > 0.0 {
                for event in window.poll_events(){
                    match event {
                        WindowEvent::Resized(width, height) => self.update_resolution((width, height)),
                        // WindowEvent::Moved(x, y) => {
                        //
                        // },
                        WindowEvent::Closed => return,
                        // WindowEvent::DroppedFile(path_buffer) => {
                        //
                        // },
                        // WindowEvent::ReceivedCharacter(character) => {
                        //
                        // },
                        WindowEvent::Focused(focused) => {
                            if focused {
                                self.resume();
                            } else {
                                self.pause();
                            }
                        },
                        WindowEvent::KeyboardInput(element_state, _, virtual_key_code) => match virtual_key_code {
                            Some(virtual_key_code) => self.update_keyboard(virtual_key_code, element_state),
                            None => (),
                        },
                        WindowEvent::MouseMoved(pos) => self.update_mouse_pos(pos),
                        // WindowEvent::MouseWheel(mouse_scroll_data) => {
                        //
                        // },
                        WindowEvent::MouseInput(element_state, mouse_button) => self.update_mouse_button(mouse_button, element_state),
                        // WindowEvent::Awakened => {
                        //
                        // },
                        // WindowEvent::Refresh => {
                        //
                        // },
                        // WindowEvent::Suspended(suspended) => {
                        //
                        // },
                        // WindowEvent::Touch(touch) => {
                        //
                        // },
                        _ => (),
                    }
                }
                {
                    let tps_f32 = tps_s as f32;
                    let events = self.tick(tps_f32);
                    self.expand_events(Arc::new(RwLock::new(events)));
                    self.execute_events(tps_f32);
                    self.clear_executions();
                    let events = self.tick_after();
                    self.execute_tick_after_events(events);
                }
                delta_time -= tps_s;
                ticks += 1;
            }
            self.render(window);
            frames += 1;
            if now > i + 1.0 {
                i += 1.0;
                println!("{} {}", frames.to_string(), ticks.to_string());
                frames = 0;
                ticks = 0;
            }
        }
    }

    fn render(&mut self, window: &mut Window) {
        let mut frame = window.frame();
        for entry in self.worlds[&self.active_world_id].read().expect("Unable to Read World when rendering").get_beings() {
            let being = entry.1;
            for entity in being.read().expect("Unable to Read Being when rendering").get_entities() {
                frame.draw_entity(entity.1, &self.transforms);
            }
        }
        frame.end();
    }

    fn tick(&mut self, delta_time: f32) -> Vec<WorldEvent<T>> {
        let events_arc: Arc<RwLock<Vec<WorldEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        let delta_time_arc = Arc::new(delta_time);
        let active_world = self.worlds.remove(&self.active_world_id).expect("Unable to find Active world in Tick");
        {
            let transforms = &self.transforms;
            self.thread_pool.scoped(|scope| {
                for entry in active_world.read().expect("Unable to Read active word in Tick").get_beings() {
                    let being = entry.1.clone();
                    let dt = delta_time_arc.clone();
                    let world = active_world.clone();
                    let events = events_arc.clone();
                    let transforms = transforms.clone();
                    scope.execute(move || {
                        let being_read = being.read().expect("Unable to Read Being in Tick");
                        let tick_events = being_read.tick(&world.read().expect("Unable to Read World in Tick"), &dt, &transforms.read().expect("Unable to Read Transforms in Tick"));
                        let mut events = events.write().expect("Unable to Write Events in Tick");
                        for event in tick_events {
                            events.push(event);
                        }
                    });
                }
            });
        }
        let events = match Arc::try_unwrap(events_arc) {
            Ok(rwlock) => rwlock,
            Err(_) => panic!("Unable to dereference events"),
        };
        self.worlds.insert(self.active_world_id, active_world);
        events.into_inner().expect("Unable to Dereference Events in Tick")
    }

    fn tick_after(&mut self) -> Vec<TickAfterEvent<T>> {
        vec!()
    }

    fn execute_tick_after_events(&mut self, events: Vec<TickAfterEvent<T>>) {

    }

    pub fn starting_events(&mut self, window: &mut Window, delta_time: f32, events: Arc<RwLock<Vec<WorldEvent<T>>>>, ticks_after_events: Vec<TickAfterEvent<T>>) {
        self.expand_events(events);
        self.execute_events(delta_time);
        self.clear_executions();
        self.execute_tick_after_events(ticks_after_events);
    }

    pub fn clear_executions(&mut self) {
        self.ranked_events.write().expect("Unable to Write Ranked Events in Clear Executions").clear();
        self.ranks.write().expect("Unable to Write Ranks in Clear Executions").clear();
    }

    fn expand_events(&mut self, events: Arc<RwLock<Vec<WorldEvent<T>>>>) {
        let mut events = events.write().unwrap();
        loop {
            match events.pop() {
                Some(event) => {
                    let rank = get_rank(event.clone());
                    self.ranks.write().expect("Unable to Write Ranks in Execute Events").push(rank);
                    if !self.ranked_events.read().expect("Unable to Read Ranked Events in Execute Events").contains_key(&rank) {
                        self.ranked_events.write().expect("Unable to Write Ranked Events in Execute Events").insert(rank, Arc::new(RwLock::new(vec!())));
                    }
                    let ranked = self.ranked_events.read().expect("Unable to Read Ranked Events in Execute Events");
                    ranked[&rank].write().expect("Unable to Write Ranked Events in Execute Events").push(event);
                },
                None => break,
            }
        }
    }

    fn execute_events(&mut self, delta_time: f32) {
        {
            let mut ranks_write = self.ranks.write().expect("Unable to Write Ranks for Sorting in Execute Events");
            ranks_write.sort_by(|a, b| b.cmp(a));
            ranks_write.dedup();
        }
        let mut re_execute = false;
        let re_execute_buffer: Arc<RwLock<Vec<WorldEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        {
            let ranks_read = self.ranks.read().expect("Unable to Read Ranks in Execute Events");
            let world = self.worlds.remove(&self.active_world_id).expect("Unable to Remove Active World in Execute Events");
            let transforms = &self.transforms;
            let manager = &self.manager;
            let ranked_events = &self.ranked_events;
            let mut executing = true;

            self.thread_pool.scoped(|scope| {
                while executing {
                    for rank in ranks_read.iter() {
                        let events_arc = ranked_events.clone();
                        let active_world = world.clone();
                        let transforms = transforms.clone();
                        let manager = manager.clone();
                        let re_execute_buffer = re_execute_buffer.clone();
                        scope.execute(move || {
                            let events = events_arc.read().expect("Unable to Read Events in Execute Events");
                            let events_vec = events.get(&rank).expect("Unable to Get Event in Execute Events");
                            let event_option  = events_vec.write().expect("Unable to Write Events Vec in Execute Events").pop();
                            let events_new_option = match event_option {
                                Some(event) => match event {
                                    WorldEvent::NewBeing(being_type) => {
                                        Some(T::make_being(manager, being_type, active_world))
                                    },
                                    WorldEvent::NewBase(being_type) => {
                                        Some(T::make_base(manager, being_type, active_world))
                                    },
                                    WorldEvent::EndBeing(id) => {
                                        let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                        world.del_being(id);
                                        None
                                    },
                                    WorldEvent::Pos2(id, vec2_event) => {
                                        match vec2_event {
                                            Vec2Event::Set(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Set Pos2 in Execute Events").write().expect("Unable to Write Being in Set Pos2 in Execute Events")
                                                .set_pos2(vec2);
                                            },
                                            Vec2Event::Add(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Add Pos2 in Execute Events").write().expect("Unable to Write Being in Add Pos2 in Execute Events")
                                                .add_pos2(vec2);
                                            },
                                            Vec2Event::Mul(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Mul Pos2 in Execute Events").write().expect("Unable to Write Being in Mul Pos2 in Execute Events")
                                                .mul_pos2(vec2);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::Pos3(id, vec3_event) => {
                                        match vec3_event {
                                            Vec3Event::Set(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Set Pos3 in Execute Events").write().expect("Unable to Write Being in Set Pos3 in Execute Events")
                                                .set_pos3(vec3);
                                            },
                                            Vec3Event::Add(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Add Pos3 in Execute Events").write().expect("Unable to Write Being in Add Pos3 in Execute Events")
                                                .add_pos3(vec3);
                                            },
                                            Vec3Event::Mul(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Mul Pos3 in Execute Events").write().expect("Unable to Write Being in Mul Pos3 in Execute Events")
                                                .mul_pos3(vec3);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::Vel2(id, vec2_event) => {
                                        match vec2_event {
                                            Vec2Event::Set(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Set Vel2 in Execute Events").write().expect("Unable to Write Being in Set Vel2 in Execute Events")
                                                .set_vel2(vec2);
                                            },
                                            Vec2Event::Add(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Add Vel2 in Execute Events").write().expect("Unable to Write Being in Add Vel2 in Execute Events")
                                                .add_vel2(vec2);
                                            },
                                            Vec2Event::Mul(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Mul Vel2 in Execute Events").write().expect("Unable to Write Being in Mul Vel2 in Execute Events")
                                                .mul_vel2(vec2);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::Vel3(id, vec3_event) => {
                                        match vec3_event {
                                            Vec3Event::Set(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Set Vel3 in Execute Events").write().expect("Unable to Write Being in Set Vel3 in Execute Events")
                                                .set_vel3(vec3);
                                            },
                                            Vec3Event::Add(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Add Vel3 in Execute Events").write().expect("Unable to Write Being in Add Vel3 in Execute Events")
                                                .add_vel3(vec3);
                                            },
                                            Vec3Event::Mul(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Mul Vel3 in Execute Events").write().expect("Unable to Write Being in Mul Vel3 in Execute Events")
                                                .mul_vel3(vec3);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::Acc2(id, vec2_event) => {
                                        match vec2_event {
                                            Vec2Event::Set(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Set Acc2 in Execute Events").write().expect("Unable to Write Being in Set Acc2 in Execute Events")
                                                .set_acc2(vec2);
                                            },
                                            Vec2Event::Add(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Add Acc2 in Execute Events").write().expect("Unable to Write Being in Add Acc2 in Execute Events")
                                                .add_acc2(vec2);
                                            },
                                            Vec2Event::Mul(vec2) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Mul Acc2 in Execute Events").write().expect("Unable to Write Being in Mul Acc2 in Execute Events")
                                                .mul_acc2(vec2);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::Acc3(id, vec3_event) => {
                                        match vec3_event {
                                            Vec3Event::Set(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Set Acc3 in Execute Events").write().expect("Unable to Write Being in Set Acc3 in Execute Events")
                                                .set_acc3(vec3);
                                            },
                                            Vec3Event::Add(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Add Acc3 in Execute Events").write().expect("Unable to Write Being in Add Acc3 in Execute Events")
                                                .add_acc3(vec3);
                                            },
                                            Vec3Event::Mul(vec3) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                world.get_being(id).expect("Unable to Get Being in Mul Acc3 in Execute Events").write().expect("Unable to Write Being in Mul Acc3 in Execute Events")
                                                .mul_acc3(vec3);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::Transform(being_id, entity_id, transform_event) => {
                                        match transform_event {
                                            TransformEvent::Perspective(matrix, inverse) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                                let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                                transforms.read().expect("Unable to Read Transforms in Entity Perspective in Execute Events").set_perspective_matrix(entity, matrix, inverse);
                                            },
                                            TransformEvent::View(matrix, inverse) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                                let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                                transforms.read().expect("Unable to Read Transforms in Entity View in Execute Events").set_view_matrix(entity, matrix, inverse);
                                            },
                                            TransformEvent::Model(matrix, inverse) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                let being = world.get_being(being_id).expect("Unable to Get Being in Entity Model in Execute Events").read().expect("Unable to Read Being in Entity Model in Execute Events");
                                                let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Model in Execute Events");
                                                transforms.read().expect("Unable to Read Transforms in Entity Model in Execute Events").set_view_matrix(entity, matrix, inverse);
                                            },
                                        };
                                        None
                                    },
                                    WorldEvent::TransformBase(being_type, entity_id, transform_event) => {
                                        match transform_event {
                                            TransformEvent::Perspective(perspective, inverse) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Entity Base Perspective in Execute Events");
                                                let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Perspective in Execute Events").read().expect("Unable to Read Base in Entity Base Perspective in Execute Events");
                                                transforms.read().expect("Unable to Read Transforms in Entity Base Perspective in Execute Events").set_perspective_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Perspective in Execute Events"), perspective, inverse);
                                            },
                                            TransformEvent::View(view, inverse) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Entity Base View in Execute Events");
                                                let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base View in Execute Events").read().expect("Unable to Read Base in Entity Base View in Execute Events");
                                                transforms.read().expect("Unable to Read Transforms Entity Base View in Execute Events").set_view_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base View in Execute Events"), view, inverse);
                                            },
                                            TransformEvent::Model(model, inverse) => {
                                                let world = active_world.read().expect("Unable to Read Active World in Entity Base Model in Execute Events");
                                                let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Model in Execute Events").read().expect("Unable to Read Base in Entity Base Model in Execute Events");
                                                transforms.read().expect("Unable to Read Transforms in Entity Base Model in Execute Events").set_model_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Model in Execute Events"), model, inverse);
                                            },
                                        };
                                        None
                                    },
                                    /*WorldEvent::Entity(being_id, entity_id, entity_event) => match entity_event {
                                        EntityEvent::Vertices(vertices) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_vertices(entity, vertices);
                                        },
                                        EntityEvent::Indices(indices) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_indices(entity, indices);
                                        },
                                        EntityEvent::Texture(texture) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_texture(entity, texture);
                                        },
                                        EntityEvent::DrawMethod(draw_method) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_draw_parameters(entity, method_to_parameters(draw_method));
                                        },
                                        EntityEvent::Perspective(matrix, inverse) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            self.transforms.read().expect("Unable to Read Transforms in Entity Perspective in Execute Events").set_perspective_matrix(entity, matrix, inverse);
                                        },
                                        EntityEvent::View(matrix, inverse) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            self.transforms.read().expect("Unable to Read Transforms in Entity View in Execute Events").set_view_matrix(entity, matrix, inverse);
                                        },
                                        EntityEvent::Model(matrix, inverse) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Model in Execute Events").read().expect("Unable to Read Being in Entity Model in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Model in Execute Events");
                                            self.transforms.read().expect("Unable to Read Transforms in Entity Model in Execute Events").set_view_matrix(entity, matrix, inverse);
                                        },
                                        EntityEvent::UseNewID(id_types) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Use New ID in Execute Events").read().expect("Unable to Read Being in Entity Use New ID in Execute Events");
                                            let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Use New ID in Execute Events").write().expect("Unable to Write Entity in Entity Use New ID in Execute Events");
                                            for id_type in id_types {
                                                entity.use_new_id(manager, id_type);
                                            }
                                        },
                                        EntityEvent::UseOldID(your_id, your_entity_id, id_types) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let my_being = world.get_being(being_id).expect("Unable to Get Being(my being) in Entity Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity Old ID in Execute Events");
                                            let mut my_entity = my_being.get_entity(entity_id).expect("Unable to Get Entity in Entity Use Old ID in Execute Events").write().expect("Unable to Write Entity in Entity Use Old ID in Execute Events");
                                            let your_being = world.get_being(your_id).expect("Unable to Get Being(your being) in Entity Use Old ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
                                            let your_entity = your_being.get_entity(your_entity_id).expect("Unable to Get Entity in Base Use Old ID in Execute Events");
                                            for id_type in id_types {
                                                my_entity.use_other_id(your_entity, id_type);
                                            }
                                        },
                                        EntityEvent::UseBaseID(your_type, your_entity_id, id_types) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let my_being = world.get_being(being_id).expect("Unable to Get Being(my being) in Entity Use Base ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
                                            let mut my_entity = my_being.get_entity(entity_id).expect("Unable to Get Entity in Entity Use Base ID in Execute Events").write().expect("Unable to Write Entity in Entity Use Base ID in Execute Events");
                                            let your_base = world.get_base(your_type).expect("Unable to Find Base Being in Use Base ID in Execute Events").read().expect("Unable to Read Base in Entity Use Base ID in Execute Events");
                                            let your_entity = your_base.get_entity(your_entity_id).expect("Unable to Find Base Entity in Use Base ID in Execute Events");
                                            for id_type in id_types {
                                                my_entity.use_other_id(your_entity, id_type);
                                            }
                                        }
                                    },
                                    WorldEvent::EntityBase(being_type, entity_id, entity_base_event) => match entity_base_event {
                                        EntityEvent::Vertices(vertices) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Vertices Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Vertices in Execute Events").read().expect("Unable to Read Base in Entity Base Vertices in Execute Events");
                                            window.set_vertices(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Vertices in Execute Events"), vertices);
                                        },
                                        EntityEvent::Indices(indices) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Indices in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Indices in Execute Events").read().expect("Unable to Read Base in Entity Base Indices in Execute Events");
                                            window.set_indices(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Indices in Execute Events"), indices);
                                        },
                                        EntityEvent::Texture(texture) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Texture in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Texture in Execute Events").read().expect("Unable to Read Base in Entity Base Texture in Execute Events");
                                            window.set_texture(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Texture in Execute Events"), texture);
                                        },
                                        EntityEvent::DrawMethod(draw_method) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Draw Method in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Draw Method in Execute Events").read().expect("Unable to Read Base in Entity Base Draw Method in Execute Events");
                                            window.set_draw_parameters(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Draw Method in Execute Events"), method_to_parameters(draw_method));
                                        },
                                        EntityEvent::Perspective(perspective, inverse) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Perspective in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Perspective in Execute Events").read().expect("Unable to Read Base in Entity Base Perspective in Execute Events");
                                            self.transforms.read().expect("Unable to Read Transforms in Entity Base Perspective in Execute Events").set_perspective_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Perspective in Execute Events"), perspective, inverse);
                                        },
                                        EntityEvent::View(view, inverse) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base View in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base View in Execute Events").read().expect("Unable to Read Base in Entity Base View in Execute Events");
                                            self.transforms.read().expect("Unable to Read Transforms Entity Base View in Execute Events").set_view_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base View in Execute Events"), view, inverse);
                                        },
                                        EntityEvent::Model(model, inverse) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Model in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Model in Execute Events").read().expect("Unable to Read Base in Entity Base Model in Execute Events");
                                            self.transforms.read().expect("Unable to Read Transforms in Entity Base Model in Execute Events").set_model_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Model in Execute Events"), model, inverse);
                                        },
                                        EntityEvent::UseNewID(id_types) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Use New ID in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Use New ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use New ID");
                                            let mut entity = base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Use New ID in Execute Events").write().expect("Unable to Write Entity in Entity Base Use New ID");
                                            for id_type in id_types {
                                                entity.use_new_id(manager, id_type);
                                            }
                                        },
                                        EntityEvent::UseOldID(your_being_id, your_entity_id, id_types) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Entity Base Use Old ID in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Use Old ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use Old ID in Execute Events");
                                            let mut my_entity = base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Use Old ID in Execute Events").write().expect("Unable to Write Entity in Entity Base Use Old ID in Execute Events");
                                            let your_being = world.get_being(your_being_id).expect("Unable to Get Being in Entity Base Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity Base Use Old ID in Execute Events");
                                            let your_entity = your_being.get_entities().get(&your_entity_id).expect("Unable to Get Entity in Entity Base Use Old ID in Execute Events");
                                            for id_type in id_types {
                                                my_entity.use_other_id(your_entity, id_type);
                                            }
                                        },
                                        EntityEvent::UseBaseID(your_being_type, your_entity_id, id_types) => {
                                            let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                            let my_base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Use Base ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use Base ID in Execute Events");
                                            let mut my_entity = my_base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Use Base ID in Execute Events").write().expect("Unable to Write Entity in Entity Base Use Base ID in Execute Events");
                                            let your_base = world.get_base(your_being_type).expect("Unable to Get Base in Entity Base Use Base ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use Base ID in Execute Events");
                                            let your_entity = your_base.get_entity(your_entity_id).expect("Unable to Get Entity in Entity Base Use Base ID in Execute Events");
                                            for id_type in id_types {
                                                my_entity.use_other_id(your_entity, id_type);
                                            }
                                        }
                                    },*/
                                },
                                None => None,
                            };
                            match events_new_option {
                                Some(mut events) => {
                                    executing = false;
                                    re_execute = true;
                                    re_execute_buffer.write().expect("Unable to Write ReExecute Buffer in Execute Events").append(&mut events);
                                },
                                None => (),
                            }
                        });
                    }
                }
            });
        }
        if re_execute {
            self.expand_events(re_execute_buffer);
            self.execute_events(delta_time);
        }
    }

    // pub fn execute_events(&mut self, manager: &mut IDManager, window: &mut Window, events: &mut Vec<WorldEvent<T>>, active_world: Arc<RwLock<World<T>>>) {
    //     let mut ranked_events: HashMap<u32, Vec<WorldEvent<T>>> = HashMap::new();
    //     let mut ranks: Vec<u32> = vec!();
    //     loop {
    //         match events.pop() {
    //             Some(event) => {
    //                 let rank = get_rank(event.clone());
    //                 ranks.push(rank);
    //                 if !ranked_events.contains_key(&rank) {
    //                     ranked_events.insert(rank, vec!());
    //                 }
    //                 ranked_events.get_mut(&rank).expect("Unable to Get Mut rank in Execute Events").push(event);
    //             },
    //             None => break,
    //         }
    //     }
    //     ranks.sort_by(|a, b| b.cmp(a));
    //     {
    //         for rank in ranks {
    //             loop {
    //                 match ranked_events.get_mut(&rank).expect("Unable to Get Mut Rank in Exceute Events").pop() {
    //                     Some(event) => match event {
    //                         WorldEvent::NewBeing(being_type, mut events) => {
    //                             T::make_being(manager, being_type, &mut events, window, self, active_world.clone());
    //                         },
    //                         WorldEvent::NewBase(being_type) => {
    //                             T::make_base(manager, being_type, window, self.transforms.clone(), active_world.clone());
    //                         },
    //                         WorldEvent::EndBeing(id) => {
    //                             let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
    //                             world.del_being(id);
    //                         }
    //                         WorldEvent::Pos2(id, vec2_event) => match vec2_event {
    //                             Vec2Event::Set(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Set Pos2 in Execute Events").write().expect("Unable to Write Being in Set Pos2 in Execute Events")
    //                                 .set_pos2(vec2);
    //                             },
    //                             Vec2Event::Add(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Add Pos2 in Execute Events").write().expect("Unable to Write Being in Add Pos2 in Execute Events")
    //                                 .add_pos2(vec2);
    //                             },
    //                             Vec2Event::Mul(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Mul Pos2 in Execute Events").write().expect("Unable to Write Being in Mul Pos2 in Execute Events")
    //                                 .mul_pos2(vec2);
    //                             },
    //                         },
    //                         WorldEvent::Pos3(id, vec3_event) => match vec3_event {
    //                             Vec3Event::Set(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Set Pos3 in Execute Events").write().expect("Unable to Write Being in Set Pos3 in Execute Events")
    //                                 .set_pos3(vec3);
    //                             },
    //                             Vec3Event::Add(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Add Pos3 in Execute Events").write().expect("Unable to Write Being in Add Pos3 in Execute Events")
    //                                 .add_pos3(vec3);
    //                             },
    //                             Vec3Event::Mul(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Mul Pos3 in Execute Events").write().expect("Unable to Write Being in Mul Pos3 in Execute Events")
    //                                 .mul_pos3(vec3);
    //                             },
    //                         },
    //                         WorldEvent::Vel2(id, vec2_event) => match vec2_event {
    //                             Vec2Event::Set(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Set Vel2 in Execute Events").write().expect("Unable to Write Being in Set Vel2 in Execute Events")
    //                                 .set_vel2(vec2);
    //                             },
    //                             Vec2Event::Add(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Add Vel2 in Execute Events").write().expect("Unable to Write Being in Add Vel2 in Execute Events")
    //                                 .add_vel2(vec2);
    //                             },
    //                             Vec2Event::Mul(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Mul Vel2 in Execute Events").write().expect("Unable to Write Being in Mul Vel2 in Execute Events")
    //                                 .mul_vel2(vec2);
    //                             },
    //                         },
    //                         WorldEvent::Vel3(id, vec3_event) => match vec3_event {
    //                             Vec3Event::Set(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Set Vel3 in Execute Events").write().expect("Unable to Write Being in Set Vel3 in Execute Events")
    //                                 .set_vel3(vec3);
    //                             },
    //                             Vec3Event::Add(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Add Vel3 in Execute Events").write().expect("Unable to Write Being in Add Vel3 in Execute Events")
    //                                 .add_vel3(vec3);
    //                             },
    //                             Vec3Event::Mul(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Mul Vel3 in Execute Events").write().expect("Unable to Write Being in Mul Vel3 in Execute Events")
    //                                 .mul_vel3(vec3);
    //                             },
    //                         },
    //                         WorldEvent::Acc2(id, vec2_event) => match vec2_event {
    //                             Vec2Event::Set(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Set Acc2 in Execute Events").write().expect("Unable to Write Being in Set Acc2 in Execute Events")
    //                                 .set_acc2(vec2);
    //                             },
    //                             Vec2Event::Add(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Add Acc2 in Execute Events").write().expect("Unable to Write Being in Add Acc2 in Execute Events")
    //                                 .add_acc2(vec2);
    //                             },
    //                             Vec2Event::Mul(vec2) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Mul Acc2 in Execute Events").write().expect("Unable to Write Being in Mul Acc2 in Execute Events")
    //                                 .mul_acc2(vec2);
    //                             },
    //                         },
    //                         WorldEvent::Acc3(id, vec3_event) => match vec3_event {
    //                             Vec3Event::Set(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Set Acc3 in Execute Events").write().expect("Unable to Write Being in Set Acc3 in Execute Events")
    //                                 .set_acc3(vec3);
    //                             },
    //                             Vec3Event::Add(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Add Acc3 in Execute Events").write().expect("Unable to Write Being in Add Acc3 in Execute Events")
    //                                 .add_acc3(vec3);
    //                             },
    //                             Vec3Event::Mul(vec3) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 world.get_being(id).expect("Unable to Get Being in Mul Acc3 in Execute Events").write().expect("Unable to Write Being in Mul Acc3 in Execute Events")
    //                                 .mul_acc3(vec3);
    //                             },
    //                         },
    //                         WorldEvent::Entity(being_id, entity_id, entity_event) => match entity_event {
    //                             EntityEvent::Vertices(vertices) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
    //                                 window.set_vertices(entity, vertices);
    //                             },
    //                             EntityEvent::Indices(indices) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
    //                                 window.set_indices(entity, indices);
    //                             },
    //                             EntityEvent::Texture(texture) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
    //                                 window.set_texture(entity, texture);
    //                             },
    //                             EntityEvent::DrawMethod(draw_method) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
    //                                 window.set_draw_parameters(entity, method_to_parameters(draw_method));
    //                             },
    //                             EntityEvent::Perspective(matrix, inverse) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
    //                                 self.transforms.read().expect("Unable to Read Transforms in Entity Perspective in Execute Events").set_perspective_matrix(entity, matrix, inverse);
    //                             },
    //                             EntityEvent::View(matrix, inverse) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
    //                                 self.transforms.read().expect("Unable to Read Transforms in Entity View in Execute Events").set_view_matrix(entity, matrix, inverse);
    //                             },
    //                             EntityEvent::Model(matrix, inverse) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Model in Execute Events").read().expect("Unable to Read Being in Entity Model in Execute Events");
    //                                 let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Model in Execute Events");
    //                                 self.transforms.read().expect("Unable to Read Transforms in Entity Model in Execute Events").set_view_matrix(entity, matrix, inverse);
    //                             },
    //                             EntityEvent::UseNewID(id_types) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let being = world.get_being(being_id).expect("Unable to Get Being in Entity Use New ID in Execute Events").read().expect("Unable to Read Being in Entity Use New ID in Execute Events");
    //                                 let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Use New ID in Execute Events").write().expect("Unable to Write Entity in Entity Use New ID in Execute Events");
    //                                 for id_type in id_types {
    //                                     entity.use_new_id(manager, id_type);
    //                                 }
    //                             },
    //                             EntityEvent::UseOldID(your_id, your_entity_id, id_types) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let my_being = world.get_being(being_id).expect("Unable to Get Being(my being) in Entity Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity Old ID in Execute Events");
    //                                 let mut my_entity = my_being.get_entity(entity_id).expect("Unable to Get Entity in Entity Use Old ID in Execute Events").write().expect("Unable to Write Entity in Entity Use Old ID in Execute Events");
    //                                 let your_being = world.get_being(your_id).expect("Unable to Get Being(your being) in Entity Use Old ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
    //                                 let your_entity = your_being.get_entity(your_entity_id).expect("Unable to Get Entity in Base Use Old ID in Execute Events");
    //                                 for id_type in id_types {
    //                                     my_entity.use_other_id(your_entity, id_type);
    //                                 }
    //                             },
    //                             EntityEvent::UseBaseID(your_type, your_entity_id, id_types) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let my_being = world.get_being(being_id).expect("Unable to Get Being(my being) in Entity Use Base ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
    //                                 let mut my_entity = my_being.get_entity(entity_id).expect("Unable to Get Entity in Entity Use Base ID in Execute Events").write().expect("Unable to Write Entity in Entity Use Base ID in Execute Events");
    //                                 let your_base = world.get_base(your_type).expect("Unable to Find Base Being in Use Base ID in Execute Events").read().expect("Unable to Read Base in Entity Use Base ID in Execute Events");
    //                                 let your_entity = your_base.get_entity(your_entity_id).expect("Unable to Find Base Entity in Use Base ID in Execute Events");
    //                                 for id_type in id_types {
    //                                     my_entity.use_other_id(your_entity, id_type);
    //                                 }
    //                             }
    //                         },
    //                         WorldEvent::EntityBase(being_type, entity_id, entity_base_event) => match entity_base_event {
    //                             EntityEvent::Vertices(vertices) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Vertices Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Vertices in Execute Events").read().expect("Unable to Read Base in Entity Base Vertices in Execute Events");
    //                                 window.set_vertices(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Vertices in Execute Events"), vertices);
    //                             },
    //                             EntityEvent::Indices(indices) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Indices in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Indices in Execute Events").read().expect("Unable to Read Base in Entity Base Indices in Execute Events");
    //                                 window.set_indices(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Indices in Execute Events"), indices);
    //                             },
    //                             EntityEvent::Texture(texture) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Texture in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Texture in Execute Events").read().expect("Unable to Read Base in Entity Base Texture in Execute Events");
    //                                 window.set_texture(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Texture in Execute Events"), texture);
    //                             },
    //                             EntityEvent::DrawMethod(draw_method) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Draw Method in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Draw Method in Execute Events").read().expect("Unable to Read Base in Entity Base Draw Method in Execute Events");
    //                                 window.set_draw_parameters(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Draw Method in Execute Events"), method_to_parameters(draw_method));
    //                             },
    //                             EntityEvent::Perspective(perspective, inverse) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Perspective in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Perspective in Execute Events").read().expect("Unable to Read Base in Entity Base Perspective in Execute Events");
    //                                 self.transforms.read().expect("Unable to Read Transforms in Entity Base Perspective in Execute Events").set_perspective_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Perspective in Execute Events"), perspective, inverse);
    //                             },
    //                             EntityEvent::View(view, inverse) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base View in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base View in Execute Events").read().expect("Unable to Read Base in Entity Base View in Execute Events");
    //                                 self.transforms.read().expect("Unable to Read Transforms Entity Base View in Execute Events").set_view_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base View in Execute Events"), view, inverse);
    //                             },
    //                             EntityEvent::Model(model, inverse) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Model in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Model in Execute Events").read().expect("Unable to Read Base in Entity Base Model in Execute Events");
    //                                 self.transforms.read().expect("Unable to Read Transforms in Entity Base Model in Execute Events").set_model_matrix(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Model in Execute Events"), model, inverse);
    //                             },
    //                             EntityEvent::UseNewID(id_types) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Use New ID in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Use New ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use New ID");
    //                                 let mut entity = base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Use New ID in Execute Events").write().expect("Unable to Write Entity in Entity Base Use New ID");
    //                                 for id_type in id_types {
    //                                     entity.use_new_id(manager, id_type);
    //                                 }
    //                             },
    //                             EntityEvent::UseOldID(your_being_id, your_entity_id, id_types) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Entity Base Use Old ID in Execute Events");
    //                                 let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Use Old ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use Old ID in Execute Events");
    //                                 let mut my_entity = base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Use Old ID in Execute Events").write().expect("Unable to Write Entity in Entity Base Use Old ID in Execute Events");
    //                                 let your_being = world.get_being(your_being_id).expect("Unable to Get Being in Entity Base Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity Base Use Old ID in Execute Events");
    //                                 let your_entity = your_being.get_entities().get(&your_entity_id).expect("Unable to Get Entity in Entity Base Use Old ID in Execute Events");
    //                                 for id_type in id_types {
    //                                     my_entity.use_other_id(your_entity, id_type);
    //                                 }
    //                             },
    //                             EntityEvent::UseBaseID(your_being_type, your_entity_id, id_types) => {
    //                                 let world = active_world.read().expect("Unable to Read Active World in Execute Events");
    //                                 let my_base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Use Base ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use Base ID in Execute Events");
    //                                 let mut my_entity = my_base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Use Base ID in Execute Events").write().expect("Unable to Write Entity in Entity Base Use Base ID in Execute Events");
    //                                 let your_base = world.get_base(your_being_type).expect("Unable to Get Base in Entity Base Use Base ID in Execute Events").read().expect("Unable to Read Base in Entity Base Use Base ID in Execute Events");
    //                                 let your_entity = your_base.get_entity(your_entity_id).expect("Unable to Get Entity in Entity Base Use Base ID in Execute Events");
    //                                 for id_type in id_types {
    //                                     my_entity.use_other_id(your_entity, id_type);
    //                                 }
    //                             }
    //                         },
    //                     },
    //                     None => break,
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    //
    // pub fn fix_unset(events: Vec<WorldEvent<T>>, being: &Box<Being<T>>) -> Vec<WorldEvent<T>>{
    //     let mut fixed = vec!();
    //     for event in events {
    //         fixed.push(match event.clone() {
    //             WorldEvent::NewBeing(_) => event,
    //             WorldEvent::NewBase(_) => event,
    //             WorldEvent::EndBeing(id) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::EndBeing(being.get_id()),
    //                 _ => event,
    //             },
    //             WorldEvent::Pos2(id, vec2_event) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::Pos2(being.get_id(), vec2_event),
    //                 _ => event,
    //             },
    //             WorldEvent::Pos3(id, vec3_event) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::Pos3(being.get_id(), vec3_event),
    //                 _ => event,
    //             },
    //             WorldEvent::Vel2(id, vec2_event) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::Vel2(being.get_id(), vec2_event),
    //                 _ => event,
    //             },
    //             WorldEvent::Vel3(id, vec3_event) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::Vel3(being.get_id(), vec3_event),
    //                 _ => event,
    //             },
    //             WorldEvent::Acc2(id, vec2_event) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::Acc2(being.get_id(), vec2_event),
    //                 _ => event,
    //             },
    //             WorldEvent::Acc3(id, vec3_event) => match id.get_id() {
    //                 UNSET_ID => WorldEvent::Acc3(being.get_id(), vec3_event),
    //                 _ => event,
    //             },
    //             // WorldEvent::Entity(being_id, entity_id, entity_event) => match being_id.get_id() {
    //             //     UNSET_ID => WorldEvent::Entity(being.get_id(), entity_id, entity_event),
    //             //     _ => event,
    //             // },
    //             // WorldEvent::EntityBase(_, _, _) => event,
    //             WorldEvent::Transform()
    //         });
    //     }
    //     fixed
    // }
}
