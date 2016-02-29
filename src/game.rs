use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use scoped_threadpool::{Pool};
use time::{precise_time_s};
use utils::{ID, IDManager, IDType};
use world::{World, WorldEvent, EntityEvent, Vec2Event, Vec3Event, get_rank};
use graphics::{Window, method_to_parameters};
use being::{BeingType};

pub struct Game<T: BeingType<T>> {
    worlds: HashMap<ID, Arc<RwLock<World<T>>>>,
    active_world_id: ID,
    thread_pool: Pool,
    window: Window,
}

impl<T: BeingType<T>> Game<T> {
    pub fn new(manager: &mut IDManager, thread_count: u32, active_world: World<T>, window: Window) -> Game<T> {
        let id = ID::new(manager, IDType::World);
        let mut map = HashMap::new();
        map.insert(id, Arc::new(RwLock::new(active_world)));
        Game {
            worlds: map,
            active_world_id: id,
            thread_pool: Pool::new(thread_count),
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
        let events = match Arc::try_unwrap(events_arc){
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
                            WorldEvent::Pos2(vec2_event) => match vec2_event {
                                Vec2Event::Set(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Set Pos2 in Execute Events")
                                    .set_pos2(vec2);
                                },
                                Vec2Event::Add(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Add Pos2 in Execute Events")
                                    .add_pos2(vec2);
                                },
                                Vec2Event::Mul(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Mul Pos2 in Execute Events")
                                    .mul_pos2(vec2);
                                },
                            },
                            WorldEvent::Pos3(vec3_event) => match vec3_event {
                                Vec3Event::Set(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Set Pos3 in Execute Events")
                                    .set_pos3(vec3);
                                },
                                Vec3Event::Add(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Add Pos3 in Execute Events")
                                    .add_pos3(vec3);
                                },
                                Vec3Event::Mul(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Mul Pos3 in Execute Events")
                                    .mul_pos3(vec3);
                                },
                            },
                            WorldEvent::Vel2(vec2_event) => match vec2_event {
                                Vec2Event::Set(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Set Vel2 in Execute Events")
                                    .set_vel2(vec2);
                                },
                                Vec2Event::Add(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Add Vel2 in Execute Events")
                                    .add_vel2(vec2);
                                },
                                Vec2Event::Mul(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Mul Vel2 in Execute Events")
                                    .mul_vel2(vec2);
                                },
                            },
                            WorldEvent::Vel3(vec3_event) => match vec3_event {
                                Vec3Event::Set(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Set Vel3 in Execute Events")
                                    .set_vel3(vec3);
                                },
                                Vec3Event::Add(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Add Vel3 in Execute Events")
                                    .add_vel3(vec3);
                                },
                                Vec3Event::Mul(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Mul Vel3 in Execute Events")
                                    .mul_vel3(vec3);
                                },
                            },
                            WorldEvent::Acc2(vec2_event) => match vec2_event {
                                Vec2Event::Set(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Set Acc2 in Execute Events")
                                    .set_acc2(vec2);
                                },
                                Vec2Event::Add(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Add Acc2 in Execute Events")
                                    .add_acc2(vec2);
                                },
                                Vec2Event::Mul(id, vec2) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Mul Acc2 in Execute Events")
                                    .mul_acc2(vec2);
                                },
                            },
                            WorldEvent::Acc3(vec3_event) => match vec3_event {
                                Vec3Event::Set(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Set Acc3 in Execute Events")
                                    .set_acc3(vec3);
                                },
                                Vec3Event::Add(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Add Acc3 in Execute Events")
                                    .add_acc3(vec3);
                                },
                                Vec3Event::Mul(id, vec3) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    world.get_being(id).write().expect("Unable to Write Being in Mul Acc3 in Execute Events")
                                    .mul_acc3(vec3);
                                },
                            },
                            WorldEvent::Entity(entity_event) => match entity_event {
                                EntityEvent::Vertices(id, vertices) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_vertices(world.get_being(id).write().expect("Unable to Write Being Set Vertices in Execute Events").get_entity_mut(), vertices);
                                },
                                EntityEvent::Indices(id, indices) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_indices(world.get_being(id).write().expect("Unable to Write Being in Set Indices in Execute Events").get_entity_mut(), indices);
                                },
                                EntityEvent::Texture(id, texture) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_texture(world.get_being(id).write().expect("Unable to Write Being in Set Texture in Execute Events").get_entity_mut(), texture);
                                },
                                EntityEvent::DrawMethod(id, draw_method) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_draw_parameters(world.get_being(id).write().expect("Unable to Write Being in Set DrawMethod in Execute Events").get_entity_mut(), method_to_parameters(draw_method));
                                },
                                EntityEvent::Perspective(id, perspective) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_perspective_matrix(world.get_being(id).write().expect("Unable to Write Being in Set Perspective in Execute Events").get_entity_mut(), perspective);
                                },
                                EntityEvent::View(id, view) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_view_matrix(world.get_being(id).write().expect("Unable to Write Being in Set View in Execute Events").get_entity_mut(), view);
                                },
                                EntityEvent::Model(id, model) => {
                                    let mut world = active_world.write().expect("Unable to Write Active World in Execute Events");
                                    self.window.set_model_matrix(world.get_being(id).write().expect("Unable to Write Being in Set Model in Execute Events").get_entity_mut(), model);
                                },
                            },
                        },
                        None => break,
                    }
                }
            }
        }
    }
}
