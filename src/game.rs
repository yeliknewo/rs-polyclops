use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use glium::glutin::Event as WindowEvent;
use glium::glutin::ElementState as GliumElementState;
use glium::glutin::MouseButton as GliumMouseButton;
use glium::glutin::VirtualKeyCode as GliumKeyCode;
use scoped_threadpool::{Pool};
use time::{precise_time_s};

use utils::{ID, IDManager, IDType};
use world::{World, WorldEvent, TickEvent, TickAfterEvent, EntityGraphicsEvent, EntityIDEvent, TransformEvent, Vec2Event, Vec3Event, get_rank_tick, get_rank_tick_after};
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
    ranked_tick_events: Arc<RwLock<HashMap<u32, Arc<RwLock<Vec<TickEvent<T>>>>>>>,
    ranked_tick_after_events: Arc<RwLock<HashMap<u32, Arc<RwLock<Vec<TickAfterEvent<T>>>>>>>,
    tick_ranks: Arc<RwLock<Vec<u32>>>,
    tick_after_ranks: Arc<RwLock<Vec<u32>>>,
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
            ranked_tick_events: Arc::new(RwLock::new(HashMap::new())),
            ranked_tick_after_events: Arc::new(RwLock::new(HashMap::new())),
            tick_ranks: Arc::new(RwLock::new(vec!())),
            tick_after_ranks: Arc::new(RwLock::new(vec!())),
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

    pub fn run(&mut self, starting_events: Vec<WorldEvent<T>>, window: &mut Window) {
        self.starting_events(window, 0.0, Arc::new(RwLock::new(starting_events)));

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
                    self.expand_tick_events(events);
                    self.execute_tick_events(tps_f32);
                    //self.clear_tick_executions();
                    let events = self.tick_after();
                    self.expand_tick_after_events(events);
                    self.execute_tick_after_events(window);
                    //self.clear_tick_after_executions();
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
        for entry in self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Render").read().expect("Unable to Read World when rendering").get_beings() {
            let being = entry.1;
            for entity in being.read().expect("Unable to Read Being when rendering").get_entities() {
                frame.draw_entity(entity.1, &self.transforms);
            }
        }
        frame.end();
    }

    fn starting_events(&mut self, window: &mut Window, delta_time: f32, events: Arc<RwLock<Vec<WorldEvent<T>>>>) {
        let events_split = self.split_events(events);
        self.expand_tick_events(events_split.0);
        self.execute_tick_events(delta_time);
        //self.clear_tick_executions();
        self.expand_tick_after_events(events_split.1);
        self.execute_tick_after_events(window);
        //self.clear_tick_after_executions();
    }

    fn split_events(&self, events: Arc<RwLock<Vec<WorldEvent<T>>>>) -> (Arc<RwLock<Vec<TickEvent<T>>>>, Arc<RwLock<Vec<TickAfterEvent<T>>>>) {
        let tick: Arc<RwLock<Vec<TickEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        let after: Arc<RwLock<Vec<TickAfterEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        {
            let mut events = events.write().expect("Unable to Write Events in Split Events");
            let mut tick = tick.write().expect("Unable to Write Tick in Split Events");
            let mut after = after.write().expect("Unable to Write After in Split Events");
            loop {
                match events.pop() {
                    Some(event) => match event {
                        WorldEvent::Tick(event) => tick.push(event),
                        WorldEvent::TickAfter(event) => after.push(event),
                    },
                    None => break,
                }
            }
        }
        (tick, after)
    }

    fn tick_after(&mut self) -> Arc<RwLock<Vec<TickAfterEvent<T>>>> {
        let events_arc: Arc<RwLock<Vec<TickAfterEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        let active_world = self.worlds.remove(&self.active_world_id).expect("Unable to Find Active World in Tick After");
        {
            let transforms = &self.transforms;
            self.thread_pool.scoped(|scope| {
                for entry in active_world.read().expect("Unable to Read Active World in Tick After").get_beings() {
                    let being = entry.1.clone();
                    let world = active_world.clone();
                    let events = events_arc.clone();
                    let transforms = transforms.clone();
                    scope.execute(move || {
                        let being_read = being.read().expect("Unable to Read Being in Tick After");
                        let tick_after_events = being_read.tick_after(&world.read().expect("Unable to Read World in Tick After"), &transforms.read().expect("Unable to Read Transforms in Tick After"));
                        let mut events = events.write().expect("Unable to Write Events in Tick After");
                        for event in tick_after_events {
                            events.push(event);
                        }
                    });
                }
            });
        }
        self.worlds.insert(self.active_world_id, active_world);
        events_arc
    }

    fn tick(&mut self, delta_time: f32) -> Arc<RwLock<Vec<TickEvent<T>>>> {
        let events_arc: Arc<RwLock<Vec<TickEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        let delta_time_arc = Arc::new(delta_time);
        let active_world = self.worlds.remove(&self.active_world_id).expect("Unable to find Active World in Tick");
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
                        let tick_events = being_read.tick(&world.read().expect("Unable to Read World in Tick"), &transforms.read().expect("Unable to Read Transforms in Tick"), &dt);
                        let mut events = events.write().expect("Unable to Write Events in Tick");
                        for event in tick_events {
                            events.push(event);
                        }
                    });
                }
            });
        }
        self.worlds.insert(self.active_world_id, active_world);
        events_arc
    }

    // fn clear_tick_after_executions(&mut self) {
    //     for entry in self.ranked_tick_after_events.read().expect("Unable to Read Ranked Tick After Events in Clear Tick After Executions").iter() {
    //         entry.1.write().expect("Unable to Write Tick After Event Rank in Clear Tick After Executions").clear();
    //     }
    //     self.tick_after_ranks.write().expect("Unable to Write Tick After Ranks in Clear Tick After Executions").clear();
    // }
    //
    // fn clear_tick_executions(&mut self) {
    //     for entry in self.ranked_tick_events.read().expect("Unable to Read Ranked Tick Events in Clear Tick Executions").iter() {
    //         entry.1.write().expect("Unable to Write Tick Event Rank in Clear Tick Executions").clear();
    //     }
    //     self.tick_ranks.write().expect("Unable to Write Tick Ranks in Clear Tick Executions").clear();
    // }

    fn expand_tick_after_events(&mut self, events: Arc<RwLock<Vec<TickAfterEvent<T>>>>) {
        let mut events = events.write().expect("Unable to Write Events in Expand Tick After Events");
        loop {
            match events.pop() {
                Some(event) => {
                    let rank = get_rank_tick_after(event.clone());
                    self.tick_after_ranks.write().expect("Unable to Write Tick After Ranks in Expand Tick After Events").push(rank);
                    if !self.ranked_tick_after_events.read().expect("Unable to Read Ranked Tick After Events in Expand Tick After Events").contains_key(&rank) {
                        self.ranked_tick_after_events.write().expect("Unable to Write Ranked Tick After Events in Expand Tick After Events").insert(rank, Arc::new(RwLock::new(vec!())));
                    }
                    let ranked = self.ranked_tick_after_events.read().expect("Unable to Read Ranked Tick After Events in Expand Tick After Events");
                    ranked.get(&rank).expect("Unable to Get Rank in Expand Tick After Events").write().expect("Unable to Write Ranked Tick After Events in Expand Tick After Events").push(event);
                },
                None => break,
            }
        }
    }

    fn expand_tick_events(&mut self, events: Arc<RwLock<Vec<TickEvent<T>>>>) {
        let mut events = events.write().expect("Unable to Write Events in Expand Tick Events");
        loop {
            match events.pop() {
                Some(event) => {
                    let rank = get_rank_tick(event.clone());
                    self.tick_ranks.write().expect("Unable to Write Tick Ranks in Expand Tick Events").push(rank);
                    if !self.ranked_tick_events.read().expect("Unable to Read Ranked Tick Events in Expand Tick Events").contains_key(&rank) {
                        self.ranked_tick_events.write().expect("Unable to Write Ranked Tick Events in Expand Tick Events").insert(rank, Arc::new(RwLock::new(vec!())));
                    }
                    let ranked = self.ranked_tick_events.read().expect("Unable to Read Ranked Tick Events in Expand Tick Events");
                    ranked.get(&rank).expect("Unable to Get Rank in Expand Tick Events").write().expect("Unable to Write Ranked Tick Events in Expand Tick Events").push(event);
                },
                None => break,
            }
        }
    }

    fn execute_tick_after_events(&mut self, window: &mut Window) {
        {
            let mut ranks_write = self.tick_after_ranks.write().expect("unable to Write Tick After Ranks in Execute Tick After Events");
            ranks_write.sort_by(|a, b| a.cmp(b));
            ranks_write.dedup();
        }
        loop {
            let rank = self.tick_after_ranks.write().expect("Unable to Write Tick After Ranks in Execute Tick After Events").pop();
            match rank {
                Some(rank) => {
                    let ranked_events = self.ranked_tick_after_events.read().expect("Unable to Read Ranked Tick After Events in Execute Tick After Events");
                    let mut ranked_events_vec = ranked_events.get(&rank).expect("Unable to Get Rank in Execute Tick After Events").write().expect("Unable to Write Ranked Events in Execute Tick After Events");
                    loop {
                        match ranked_events_vec.pop() {
                            Some(event) => {
                                match event {
                                    TickAfterEvent::EndBeing(id) => {
                                        let mut world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").write().expect("Unable to Write Active World in Execute Events");
                                        world.del_being(id);
                                    },
                                    TickAfterEvent::Entity(being_id, entity_id, entity_event) => match entity_event {
                                        EntityGraphicsEvent::Vertices(vertices) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_vertices(entity, vertices);
                                        },
                                        EntityGraphicsEvent::Indices(indices) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_indices(entity, indices);
                                        },
                                        EntityGraphicsEvent::Texture(texture) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_texture(entity, texture);
                                        },
                                        EntityGraphicsEvent::DrawMethod(draw_method) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Execute Events");
                                            let being = world.get_being(being_id).expect("Unable to Get Being in Entity Vertices in Execute Events").read().expect("Unable to Read Being in Entity Vertices in Execute Events");
                                            let entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity Vertices in Execute Events");
                                            window.set_draw_parameters(entity, method_to_parameters(draw_method));
                                        },
                                    },
                                    TickAfterEvent::EntityBase(being_type, entity_id, entity_base_event) => match entity_base_event {
                                        EntityGraphicsEvent::Vertices(vertices) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Entity Base Vertices Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Vertices in Execute Events").read().expect("Unable to Read Base in Entity Base Vertices in Execute Events");
                                            window.set_vertices(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Vertices in Execute Events"), vertices);
                                        },
                                        EntityGraphicsEvent::Indices(indices) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Entity Base Indices in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Indices in Execute Events").read().expect("Unable to Read Base in Entity Base Indices in Execute Events");
                                            window.set_indices(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Indices in Execute Events"), indices);
                                        },
                                        EntityGraphicsEvent::Texture(texture) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Entity Base Texture in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Texture in Execute Events").read().expect("Unable to Read Base in Entity Base Texture in Execute Events");
                                            window.set_texture(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Texture in Execute Events"), texture);
                                        },
                                        EntityGraphicsEvent::DrawMethod(draw_method) => {
                                            let world = self.worlds.get(&self.active_world_id).expect("Unable to Get Active World in Execute Events").read().expect("Unable to Read Active World in Entity Base Draw Method in Execute Events");
                                            let base = world.get_base(being_type).expect("Unable to Get Base in Entity Base Draw Method in Execute Events").read().expect("Unable to Read Base in Entity Base Draw Method in Execute Events");
                                            window.set_draw_parameters(base.get_entity(entity_id).expect("Unable to Get Entity in Entity Base Draw Method in Execute Events"), method_to_parameters(draw_method));
                                        },
                                    },
                                };
                            },
                            None => break,
                        }
                    }
                },
                None => break,
            }
        }
    }

    fn execute_tick_events(&mut self, delta_time: f32) {
        {
            let mut ranks_write = self.tick_ranks.write().expect("Unable to Write Ranks for Sorting in Execute Events");
            ranks_write.sort_by(|a, b| a.cmp(b));
            ranks_write.dedup();
        }
        let re_execute = AtomicBool::new(false);
        {
            let re_execute = &re_execute;
            let re_execute_buffer: Arc<RwLock<Vec<WorldEvent<T>>>> = Arc::new(RwLock::new(vec!()));
            {
                let ranks = self.tick_ranks.clone();
                let world = self.worlds.remove(&self.active_world_id).expect("Unable to Remove Active World in Execute Events");
                let manager = &self.manager;
                let ranked_events = &self.ranked_tick_events;
                let executing = AtomicBool::new(true);
                let rank_is_good = AtomicBool::new(true);
                let transforms = &self.transforms;
                self.thread_pool.scoped(|scope| {
                    let executing = &executing;
                    while executing.load(Ordering::Relaxed) {
                        let rank = ranks.write().expect("Unable to Write Ranks in Execute Events").pop();
                        {
                            let rank_is_good = &rank_is_good;
                            match rank {
                                Some(rank) => {
                                    while rank_is_good.load(Ordering::Relaxed) {
                                        let events_arc = ranked_events.clone();
                                        let active_world = world.clone();
                                        let manager = manager.clone();
                                        let transforms = transforms.clone();
                                        let re_execute_buffer = re_execute_buffer.clone();
                                        scope.execute(move || {
                                            let events = events_arc.read().expect("Unable to Read Events in Execute Events");
                                            let events_vec = events.get(&rank).expect("Unable to Get Event in Execute Events");
                                            let event_option  = events_vec.write().expect("Unable to Write Events Vec in Execute Events").pop();
                                            let events_new_option = match event_option {
                                                Some(event) => match event {
                                                    TickEvent::NewBeing(being_type, being_args) => {
                                                        Some(T::make_being(manager, being_type, active_world, being_args))
                                                    },
                                                    TickEvent::NewBase(being_type) => {
                                                        Some(T::make_base(manager, being_type, active_world))
                                                    },
                                                    TickEvent::Sca2(id, vec2_event) => {
                                                        match vec2_event {
                                                            Vec2Event::Set(vec2) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Set Sca2 in Execute Events").write().expect("Unable to Write Being in Set Pos2 in Execute Events")
                                                                .set_sca2(vec2);
                                                            },
                                                            Vec2Event::Add(vec2) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Add Sca2 in Execute Events").write().expect("Unable to Write Being in Add Pos2 in Execute Events")
                                                                .add_sca2(vec2);
                                                            },
                                                            Vec2Event::Mul(vec2) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Mul Sca2 in Execute Events").write().expect("Unable to Write Being in Mul Pos2 in Execute Events")
                                                                .mul_sca2(vec2);
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::Sca3(id, vec3_event) => {
                                                        match vec3_event {
                                                            Vec3Event::Set(vec3) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Set Pos3 in Execute Events").write().expect("Unable to Write Being in Set Pos3 in Execute Events")
                                                                .set_sca3(vec3);
                                                            },
                                                            Vec3Event::Add(vec3) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Add Pos3 in Execute Events").write().expect("Unable to Write Being in Add Pos3 in Execute Events")
                                                                .add_sca3(vec3);
                                                            },
                                                            Vec3Event::Mul(vec3) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Mul Pos3 in Execute Events").write().expect("Unable to Write Being in Mul Pos3 in Execute Events")
                                                                .mul_sca3(vec3);
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::Rot2(id, vec2_event) => {
                                                        match vec2_event {
                                                            Vec2Event::Set(vec2) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Set Pos2 in Execute Events").write().expect("Unable to Write Being in Set Pos2 in Execute Events")
                                                                .set_rot2(vec2);
                                                            },
                                                            Vec2Event::Add(vec2) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Add Pos2 in Execute Events").write().expect("Unable to Write Being in Add Pos2 in Execute Events")
                                                                .add_rot2(vec2);
                                                            },
                                                            Vec2Event::Mul(vec2) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Mul Pos2 in Execute Events").write().expect("Unable to Write Being in Mul Pos2 in Execute Events")
                                                                .mul_rot2(vec2);
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::Rot3(id, vec3_event) => {
                                                        match vec3_event {
                                                            Vec3Event::Set(vec3) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Set Pos3 in Execute Events").write().expect("Unable to Write Being in Set Pos3 in Execute Events")
                                                                .set_rot3(vec3);
                                                            },
                                                            Vec3Event::Add(vec3) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Add Pos3 in Execute Events").write().expect("Unable to Write Being in Add Pos3 in Execute Events")
                                                                .add_rot3(vec3);
                                                            },
                                                            Vec3Event::Mul(vec3) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Execute Events");
                                                                world.get_being(id).expect("Unable to Get Being in Mul Pos3 in Execute Events").write().expect("Unable to Write Being in Mul Pos3 in Execute Events")
                                                                .mul_rot3(vec3);
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::Pos2(id, vec2_event) => {
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
                                                    TickEvent::Pos3(id, vec3_event) => {
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
                                                    TickEvent::Vel2(id, vec2_event) => {
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
                                                    TickEvent::Vel3(id, vec3_event) => {
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
                                                    TickEvent::Acc2(id, vec2_event) => {
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
                                                    TickEvent::Acc3(id, vec3_event) => {
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
                                                    TickEvent::EntityID(being_id, entity_id, entity_id_event) => {
                                                        match entity_id_event {
                                                            EntityIDEvent::UseNewID(ids_to_change) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Entity ID Use New ID in Execute Events");
                                                                let being = world.get_being(being_id).expect("Unable to Get Being in Entity ID Use New ID in Execute Events").read().expect("Unable to Read Being in Entity ID Use New ID in Execute Events");
                                                                let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity ID Use New ID in Execute Events").write().expect("Unable to Write Entity in Entity ID Use New ID in Execute Events");
                                                                for id in ids_to_change {
                                                                    entity.use_new_id(manager.clone(), id);
                                                                }
                                                            },
                                                            EntityIDEvent::UseOldID(your_being_id, your_entity_id, ids_to_change) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Entity ID Use Old ID in Execute Events");
                                                                let being = world.get_being(being_id).expect("Unable to Get Being in Entity ID Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity ID Use Old ID in Execute Events");
                                                                let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity ID Use Old ID in Execute Events").write().expect("Unable to Write Entity in Entity ID Use Old ID in Execute Events");
                                                                let your_being = world.get_being(your_being_id).expect("Unable to Get Being in Entity ID Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity ID Use Old ID in Execute Events");
                                                                let your_entity = your_being.get_entity(your_entity_id).expect("Unable to Get Entity in Entity ID use Old ID in Execute Events");
                                                                for id in ids_to_change {
                                                                    entity.use_old_id(your_entity, id);
                                                                }
                                                            },
                                                            EntityIDEvent::UseBaseID(your_being_type, your_entity_id, ids_to_change) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Entity ID Use Base ID in Execute Events");
                                                                let being = world.get_being(being_id).expect("Unable to Get Being in Entity ID Use Base ID in Execute Events").read().expect("Unable to Read Being in Entity ID Use Base ID in Execute Events");
                                                                let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity ID Use Base ID in Execute Events").write().expect("Unable to Write Entity in Entity ID Use Base ID in Execute Events");
                                                                let your_being = world.get_base(your_being_type).expect("Unable to Get Base in Entity ID Use Base ID in Execute Events").read().expect("Unable to Read Being in Entity ID Use Base ID in Execute Events");
                                                                let your_entity = your_being.get_entity(your_entity_id).expect("Unable to Get Entity in Entity ID use Base ID in Execute Events");
                                                                for id in ids_to_change {
                                                                    entity.use_old_id(your_entity, id);
                                                                }
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::EntityIDBase(being_type, entity_id, entity_id_event) => {
                                                        match entity_id_event {
                                                            EntityIDEvent::UseNewID(ids_to_change) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Entity ID Base Use New ID in Execute Events");
                                                                let being = world.get_base(being_type).expect("Unable to Get Being in Entity ID Base Use New ID in Execute Events").read().expect("Unable to Read Being in Entity ID Base Use New ID in Execute Events");
                                                                let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity ID Base Use New ID in Execute Events").write().expect("Unable to Write Entity in Entity ID Base Use New ID in Execute Events");
                                                                for id in ids_to_change {
                                                                    entity.use_new_id(manager.clone(), id);
                                                                }
                                                            },
                                                            EntityIDEvent::UseOldID(your_being_id, your_entity_id, ids_to_change) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Entity ID Base Use Old ID in Execute Events");
                                                                let being = world.get_base(being_type).expect("Unable to Get Being in Entity ID Base Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity ID Base Use Old ID in Execute Events");
                                                                let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity ID Base Use Old ID in Execute Events").write().expect("Unable to Write Entity in Entity ID Base Use Old ID in Execute Events");
                                                                let your_being = world.get_being(your_being_id).expect("Unable to Get Being in Entity ID Base Use Old ID in Execute Events").read().expect("Unable to Read Being in Entity ID Base Use Old ID in Execute Events");
                                                                let your_entity = your_being.get_entity(your_entity_id).expect("Unable to Get Entity in Entity ID Base use Old ID in Execute Events");
                                                                for id in ids_to_change {
                                                                    entity.use_old_id(your_entity, id);
                                                                }
                                                            },
                                                            EntityIDEvent::UseBaseID(your_being_type, your_entity_id, ids_to_change) => {
                                                                let world = active_world.read().expect("Unable to Read Active World in Entity ID Base Use Base ID in Execute Events");
                                                                let being = world.get_base(being_type).expect("Unable to Get Being in Entity ID Base Use Base ID in Execute Events").read().expect("Unable to Read Being in Entity ID Base Use Base ID in Execute Events");
                                                                let mut entity = being.get_entity(entity_id).expect("Unable to Get Entity in Entity ID Base Use Base ID in Execute Events").write().expect("Unable to Write Entity in Entity ID Base Use Base ID in Execute Events");
                                                                let your_being = world.get_base(your_being_type).expect("Unable to Get Base in Entity ID Base Use Base ID in Execute Events").read().expect("Unable to Read Being in Entity ID Base Use Base ID in Execute Events");
                                                                let your_entity = your_being.get_entity(your_entity_id).expect("Unable to Get Entity in Entity ID Base use Base ID in Execute Events");
                                                                for id in ids_to_change {
                                                                    entity.use_old_id(your_entity, id);
                                                                }
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::Transform(being_id, entity_id, transform_event) => {
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
                                                                transforms.read().expect("Unable to Read Transforms in Entity Model in Execute Events").set_model_matrix(entity, matrix, inverse);
                                                            },
                                                        };
                                                        None
                                                    },
                                                    TickEvent::TransformBase(being_type, entity_id, transform_event) => {
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
                                                },
                                                None => {
                                                    rank_is_good.store(false, Ordering::Relaxed);
                                                    None
                                                },
                                            };
                                            match events_new_option {
                                                Some(mut events) => {
                                                    executing.store(false, Ordering::Relaxed);
                                                    re_execute.store(true, Ordering::Relaxed);
                                                    re_execute_buffer.write().expect("Unable to Write ReExecute Buffer in Execute Events").append(&mut events);
                                                },
                                                None => (),
                                            }
                                        });
                                    }
                                },
                                None => {
                                    executing.store(false, Ordering::Relaxed);
                                }
                            }
                        }
                        scope.join_all();
                    }
                });
                self.worlds.insert(self.active_world_id, world);
            }
            if re_execute.load(Ordering::Relaxed) {
                let events_split = self.split_events(re_execute_buffer);
                self.expand_tick_events(events_split.0);
                self.expand_tick_after_events(events_split.1);
                self.execute_tick_events(delta_time);
            }
        }
    }
}
