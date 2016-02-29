use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use glium::glutin::Event as WindowEvent;
use scoped_threadpool::{Pool};
use time::{precise_time_s};
use utils::{UNSET_ID, ID, IDManager, IDType};
use world::{World, WorldEvent, EntityEvent, EntityBaseEvent, Vec2Event, Vec3Event, get_rank};
use graphics::{Window, method_to_parameters};
use being::{BeingType, Being};

pub struct Game<T: BeingType<T>> {
    worlds: HashMap<ID, Arc<RwLock<World<T>>>>,
    active_world_id: ID,
    thread_pool: Pool,
    window: Window,
    resolution: (u32, u32),
    aspect_ratio: f32,
}

impl<T: BeingType<T>> Game<T> {
    pub fn new(manager: &mut IDManager, thread_count: u32, active_world: World<T>, window: Window) -> Game<T> {
        let id = ID::new(manager, IDType::World);
        let mut map = HashMap::new();
        map.insert(id, Arc::new(RwLock::new(active_world)));
        let resolution = window.get_start_resolution();
        Game {
            worlds: map,
            active_world_id: id,
            thread_pool: Pool::new(thread_count),
            aspect_ratio: resolution.0 as f32 / resolution.1 as f32,
            resolution: resolution,
            window: window,
        }
    }

    pub fn run(&mut self, manager: &mut IDManager, starting_events: &mut Vec<WorldEvent<T>>) {
        self.starting_events(manager, starting_events);

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
                for event in self.window.poll_events() {
                    match event {
                        WindowEvent::Resized(width, height) => {
                            self.resolution = (width, height);
                            self.aspect_ratio = (width as f32 / height as f32);
                        },
                        WindowEvent::Moved(x, y) => {

                        },
                        WindowEvent::Closed => {

                        },
                        WindowEvent::DroppedFile(path_buffer) => {

                        },
                        WindowEvent::ReceivedCharacter(character) => {

                        },
                        WindowEvent::Focused(focused) => {

                        },
                        WindowEvent::KeyboardInput(element_state, key_code, virtual_key_code) => {

                        },
                        WindowEvent::MouseMoved((x, y)) => {

                        },
                        WindowEvent::MouseWheel(mouse_scroll_data) => {

                        },
                        WindowEvent::MouseInput(element_state, mouse_button) => {

                        },
                        WindowEvent::Awakened => {

                        },
                        WindowEvent::Refresh => {

                        },
                        WindowEvent::Suspended(suspended) => {

                        },
                        WindowEvent::Touch(touch) => {

                        },
                    }
                }
                self.tick(manager, tps_s as f32);
                delta_time -= tps_s;
                ticks += 1;
            }
            self.render();
            frames += 1;
            if now > i + 1.0 {
                i += 1.0;
                println!("{} {}", frames.to_string(), ticks.to_string());
                frames = 0;
                ticks = 0;
            }
        }
    }

    fn render(&mut self) {
        let mut frame = self.window.frame();
        for entry in self.worlds[&self.active_world_id].read().expect("Unable to Read World when rendering").get_beings() {
            let being = entry.1;
            frame.draw_entity(being.read().expect("Unable to Read Being when rendering").get_entity());
        }
        frame.end();
    }

    fn tick(&mut self, manager: &mut IDManager, delta_time: f32) {
        let events_arc: Arc<RwLock<Vec<WorldEvent<T>>>> = Arc::new(RwLock::new(vec!()));
        let delta_time_arc = Arc::new(delta_time);
        let active_world = self.worlds.remove(&self.active_world_id).expect("Unable to find Active world in Tick");
        self.thread_pool.scoped(|scope| {
            for entry in active_world.read().expect("Unable to Read active word in Tick").get_beings() {
                let being = entry.1.clone();
                let dt = delta_time_arc.clone();
                let world = active_world.clone();
                let events = events_arc.clone();
                scope.execute(move || {
                    let being_read = being.read().expect("Unable to Read Being in Tick");
                    let tick_events = being_read.tick(&world.read().expect("Unable to Read World in Tick"), &dt);
                    let mut events = events.write().expect("Unable to Write Events in Tick");
                    for event in tick_events {
                        events.push(event);
                    }
                });
            }
        });
        let events = match Arc::try_unwrap(events_arc) {
            Ok(rwlock) => rwlock,
            Err(_) => panic!("Unable to dereference events"),
        };
        let mut events = events.into_inner().expect("Unable to Dereference Events in Tick");
        self.execute_events(manager, &mut events, active_world.clone());
        self.worlds.insert(self.active_world_id, active_world);
    }

    pub fn starting_events(&mut self, manager: &mut IDManager, events: &mut Vec<WorldEvent<T>>) {
        let world = self.worlds.remove(&self.active_world_id).expect("Unable to Find Active World in Starting Events");
        self.execute_events(manager, events, world.clone());
        self.worlds.insert(self.active_world_id, world);
    }

    pub fn execute_events(&mut self, manager: &mut IDManager, events: &mut Vec<WorldEvent<T>>, active_world: Arc<RwLock<World<T>>>) {
        let mut ranked_events: HashMap<u32, Vec<WorldEvent<T>>> = HashMap::new();
        let mut ranks: Vec<u32> = vec!();
        loop {
            match events.pop() {
                Some(event) => {
                    let rank = get_rank(event.clone());
                    ranks.push(rank);
                    if !ranked_events.contains_key(&rank) {
                        ranked_events.insert(rank, vec!());
                    }
                    ranked_events.get_mut(&rank).expect("Unable to Get Mut rank in Execute Events").push(event);
                },
                None => break,
            }
        }
        ranks.sort_by(|a, b| b.cmp(a));
        {
            for rank in ranks {
                loop {
                    match ranked_events.get_mut(&rank).expect("Unable to Get Mut Rank in Exceute Events").pop() {
                        Some(event) => match event {
                            WorldEvent::NewBeing(being_type, mut events) => {
                                T::make_being(manager, being_type, &mut events, self, active_world.clone());
                            },
                            WorldEvent::NewBeingBase(being_type, mut events) => {
                                T::make_being(manager, being_type, &mut events, self, active_world.clone());
                            },
                            WorldEvent::EndBeing(id) => {
                                let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                world.del_being(id);
                            }
                            WorldEvent::Pos2(id, vec2_event) => match vec2_event {
                                Vec2Event::Set(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Set Pos2 in Execute Events").write().expect("Unable to Write Being in Set Pos2 in Execute Events")
                                    .set_pos2(vec2);
                                },
                                Vec2Event::Add(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Add Pos2 in Execute Events").write().expect("Unable to Write Being in Add Pos2 in Execute Events")
                                    .add_pos2(vec2);
                                },
                                Vec2Event::Mul(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Mul Pos2 in Execute Events").write().expect("Unable to Write Being in Mul Pos2 in Execute Events")
                                    .mul_pos2(vec2);
                                },
                            },
                            WorldEvent::Pos3(id, vec3_event) => match vec3_event {
                                Vec3Event::Set(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Set Pos3 in Execute Events").write().expect("Unable to Write Being in Set Pos3 in Execute Events")
                                    .set_pos3(vec3);
                                },
                                Vec3Event::Add(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Add Pos3 in Execute Events").write().expect("Unable to Write Being in Add Pos3 in Execute Events")
                                    .add_pos3(vec3);
                                },
                                Vec3Event::Mul(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Mul Pos3 in Execute Events").write().expect("Unable to Write Being in Mul Pos3 in Execute Events")
                                    .mul_pos3(vec3);
                                },
                            },
                            WorldEvent::Vel2(id, vec2_event) => match vec2_event {
                                Vec2Event::Set(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Set Vel2 in Execute Events").write().expect("Unable to Write Being in Set Vel2 in Execute Events")
                                    .set_vel2(vec2);
                                },
                                Vec2Event::Add(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Add Vel2 in Execute Events").write().expect("Unable to Write Being in Add Vel2 in Execute Events")
                                    .add_vel2(vec2);
                                },
                                Vec2Event::Mul(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Mul Vel2 in Execute Events").write().expect("Unable to Write Being in Mul Vel2 in Execute Events")
                                    .mul_vel2(vec2);
                                },
                            },
                            WorldEvent::Vel3(id, vec3_event) => match vec3_event {
                                Vec3Event::Set(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Set Vel3 in Execute Events").write().expect("Unable to Write Being in Set Vel3 in Execute Events")
                                    .set_vel3(vec3);
                                },
                                Vec3Event::Add(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Add Vel3 in Execute Events").write().expect("Unable to Write Being in Add Vel3 in Execute Events")
                                    .add_vel3(vec3);
                                },
                                Vec3Event::Mul(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Mul Vel3 in Execute Events").write().expect("Unable to Write Being in Mul Vel3 in Execute Events")
                                    .mul_vel3(vec3);
                                },
                            },
                            WorldEvent::Acc2(id, vec2_event) => match vec2_event {
                                Vec2Event::Set(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Set Acc2 in Execute Events").write().expect("Unable to Write Being in Set Acc2 in Execute Events")
                                    .set_acc2(vec2);
                                },
                                Vec2Event::Add(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Add Acc2 in Execute Events").write().expect("Unable to Write Being in Add Acc2 in Execute Events")
                                    .add_acc2(vec2);
                                },
                                Vec2Event::Mul(vec2) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Mul Acc2 in Execute Events").write().expect("Unable to Write Being in Mul Acc2 in Execute Events")
                                    .mul_acc2(vec2);
                                },
                            },
                            WorldEvent::Acc3(id, vec3_event) => match vec3_event {
                                Vec3Event::Set(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Set Acc3 in Execute Events").write().expect("Unable to Write Being in Set Acc3 in Execute Events")
                                    .set_acc3(vec3);
                                },
                                Vec3Event::Add(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Add Acc3 in Execute Events").write().expect("Unable to Write Being in Add Acc3 in Execute Events")
                                    .add_acc3(vec3);
                                },
                                Vec3Event::Mul(vec3) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).expect("Unable to Get Being in Mul Acc3 in Execute Events").write().expect("Unable to Write Being in Mul Acc3 in Execute Events")
                                    .mul_acc3(vec3);
                                },
                            },
                            WorldEvent::Entity(id, entity_event) => match entity_event {
                                EntityEvent::Vertices(vertices) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_vertices(world.get_being(id).expect("Unable to Get Being in Entity Set Vertices in Execute Events").write().expect("Unable to Write Being Set Vertices in Execute Events").get_entity_mut(), vertices);
                                },
                                EntityEvent::Indices(indices) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_indices(world.get_being(id).expect("Unable to Get Being in Entity Set Indices in Execute Events").write().expect("Unable to Write Being in Set Indices in Execute Events").get_entity_mut(), indices);
                                },
                                EntityEvent::Texture(texture) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_texture(world.get_being(id).expect("Unable to Get Being in Entity Set Texture in Execute Events").write().expect("Unable to Write Being in Set Texture in Execute Events").get_entity_mut(), texture);
                                },
                                EntityEvent::DrawMethod(draw_method) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_draw_parameters(world.get_being(id).expect("Unable to Get Being in Entity Set Draw Method in Execute Events").write().expect("Unable to Write Being in Set DrawMethod in Execute Events").get_entity_mut(), method_to_parameters(draw_method));
                                },
                                EntityEvent::Perspective(perspective) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_perspective_matrix(world.get_being(id).expect("Unable to Get Being in Entity Set Perspective in Execute Events").write().expect("Unable to Write Being in Set Perspective in Execute Events").get_entity_mut(), perspective);
                                },
                                EntityEvent::View(view) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_view_matrix(world.get_being(id).expect("Unable to Get Being in Entity Set View in Execute Events").write().expect("Unable to Write Being in Set View in Execute Events").get_entity_mut(), view);
                                },
                                EntityEvent::Model(model) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_model_matrix(world.get_being(id).expect("Unable to Get Being in Entity Set Model in Execute Events").write().expect("Unable to Write Being in Set Model in Execute Events").get_entity_mut(), model);
                                },
                                EntityEvent::UseNewID(id_types) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    let mut being = world.get_being(id).expect("Unable to Get Being in Entity Use New ID in Execute Events").write().expect("Unable to Write Being in New ID in Execute Events");
                                    let mut entity = being.get_entity_mut();
                                    for id_type in id_types {
                                        entity.use_new_id(manager, id_type);
                                    }
                                },
                                EntityEvent::UseOldID(your_id, id_types) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    let mut my_being = world.get_being(id).expect("Unable to Get Being(my being) in Entity Use Old ID in Execute Events").write().expect("Unable to Write Being in Old ID in Execute Events");
                                    let mut my_entity = my_being.get_entity_mut();
                                    let your_being = world.get_being(your_id).expect("Unable to Get Being(your being) in Entity Use Old ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
                                    let your_entity = your_being.get_entity();
                                    for id_type in id_types {
                                        my_entity.use_other_id(your_entity, id_type);
                                    }
                                },
                                EntityEvent::UseBaseID(your_type, id_types) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    let mut my_being = world.get_being(id).expect("Unable to Get Being(my being) in Entity Use Base ID in Execute Events").write().expect("Unable to Write Being in Old ID in Execute Events");
                                    let mut my_entity = my_being.get_entity_mut();
                                    let your_being = world.get_base_being(your_type).expect("Unable to Find Base Being in Use Base ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
                                    let your_entity = your_being.get_entity();
                                    for id_type in id_types {
                                        my_entity.use_other_id(your_entity, id_type);
                                    }
                                }
                            },
                            WorldEvent::EntityBase(being_type, entity_base_event) => match entity_base_event {
                                EntityBaseEvent::Vertices(vertices) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_vertices(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set Vertices in Execute Events").write().expect("Unable to Write Being Set Vertices in Execute Events").get_entity_mut(), vertices);
                                },
                                EntityBaseEvent::Indices(indices) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_indices(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set Indices in Execute Events").write().expect("Unable to Write Being in Set Indices in Execute Events").get_entity_mut(), indices);
                                },
                                EntityBaseEvent::Texture(texture) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_texture(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set Texture in Execute Events").write().expect("Unable to Write Being in Set Texture in Execute Events").get_entity_mut(), texture);
                                },
                                EntityBaseEvent::DrawMethod(draw_method) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_draw_parameters(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set Draw Method in Execute Events").write().expect("Unable to Write Being in Set DrawMethod in Execute Events").get_entity_mut(), method_to_parameters(draw_method));
                                },
                                EntityBaseEvent::Perspective(perspective) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_perspective_matrix(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set Perspective in Execute Events").write().expect("Unable to Write Being in Set Perspective in Execute Events").get_entity_mut(), perspective);
                                },
                                EntityBaseEvent::View(view) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_view_matrix(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set View in Execute Events").write().expect("Unable to Write Being in Set View in Execute Events").get_entity_mut(), view);
                                },
                                EntityBaseEvent::Model(model) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_model_matrix(world.get_base_being(being_type).expect("Unable to Get Being in Entity Set Model in Execute Events").write().expect("Unable to Write Being in Set Model in Execute Events").get_entity_mut(), model);
                                },
                                EntityBaseEvent::UseNewID(id_types) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    let mut being = world.get_base_being(being_type).expect("Unable to Get Being in Entity Use New ID in Execute Events").write().expect("Unable to Write Being in New ID in Execute Events");
                                    let mut entity = being.get_entity_mut();
                                    for id_type in id_types {
                                        entity.use_new_id(manager, id_type);
                                    }
                                },
                                EntityBaseEvent::UseOldID(your_id, id_types) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    let mut my_being = world.get_base_being(being_type).expect("Unable to Get Being(my being) in Entity Use Old ID in Execute Events").write().expect("Unable to Write Being in Old ID in Execute Events");
                                    let mut my_entity = my_being.get_entity_mut();
                                    let your_being = world.get_being(your_id).expect("Unable to Get Being(your being) in Entity Use Old ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
                                    let your_entity = your_being.get_entity();
                                    for id_type in id_types {
                                        my_entity.use_other_id(your_entity, id_type);
                                    }
                                },
                                EntityBaseEvent::UseBaseID(your_being_type, id_types) => {
                                    let world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    let mut my_being = world.get_base_being(being_type).expect("Unable to Get Being(my being) in Entity Use Base ID in Execute Events").write().expect("Unable to Write Being in Old ID in Execute Events");
                                    let mut my_entity = my_being.get_entity_mut();
                                    let your_being = world.get_base_being(your_being_type).expect("Unable to Find Base Being in Use Base ID in Execute Events").read().expect("Unable to Read Being in Old ID in Execute Events");
                                    let your_entity = your_being.get_entity();
                                    for id_type in id_types {
                                        my_entity.use_other_id(your_entity, id_type);
                                    }
                                }
                            },
                        },
                        None => break,
                    }
                }
            }
        }
    }


    pub fn fix_unset(events: Vec<WorldEvent<T>>, being: &Box<Being<T>>) -> Vec<WorldEvent<T>>{
        let mut fixed = vec!();
        for event in events {
            fixed.push(match event.clone() {
                WorldEvent::NewBeing(_, _) => event,
                WorldEvent::NewBeingBase(_, _) => event,
                WorldEvent::EndBeing(id) => match id.get_id() {
                    UNSET_ID => WorldEvent::EndBeing(being.get_id()),
                    _ => event,
                },
                WorldEvent::Pos2(id, vec2_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Pos2(being.get_id(), vec2_event),
                    _ => event,
                },
                WorldEvent::Pos3(id, vec3_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Pos3(being.get_id(), vec3_event),
                    _ => event,
                },
                WorldEvent::Vel2(id, vec2_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Vel2(being.get_id(), vec2_event),
                    _ => event,
                },
                WorldEvent::Vel3(id, vec3_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Vel3(being.get_id(), vec3_event),
                    _ => event,
                },
                WorldEvent::Acc2(id, vec2_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Acc2(being.get_id(), vec2_event),
                    _ => event,
                },
                WorldEvent::Acc3(id, vec3_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Acc3(being.get_id(), vec3_event),
                    _ => event,
                },
                WorldEvent::Entity(id, entity_event) => match id.get_id() {
                    UNSET_ID => WorldEvent::Entity(being.get_id(), entity_event),
                    _ => event,
                },
                WorldEvent::EntityBase(being_type, entity_base_event) => event,
            });
        }
        fixed
    }
}
