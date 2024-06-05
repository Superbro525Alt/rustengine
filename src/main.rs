mod engine;
mod frontend;

use std::time::{Duration, Instant};
#[allow(unused)]
use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::engine::{collider::OctagonCollider, save::{EngineSaveData, self}};
#[allow(unused)]
use crate::engine::component::ComponentTrait;
use engine::{gameobject::{self, make_base_game_object, GameObject}, static_component::StaticComponent, component::{CharacterController2D, ComponentState, ComponentWrapper, TickVariant, InputTickBehavior, RenderTickBehavior, RenderOutput, Transform}, components::RenderComponent, graphics_backend::{primitives::{Primitives, self}, object::Object}, bounds::Bounds2D, raycast, collider::{CubeCollider, Point}, time::OxidizedInstant};

use serde::{Serialize, Deserialize};

#[allow(unused)]
use rand::Rng;
use winit::event_loop::EventLoopBuilder;
use eframe;
use std::env;
use log;

fn main() {
    env_logger::init();
    pollster::block_on(run());
}

#[derive(Debug, Serialize, Deserialize)]
struct Score {
    pub over: bool,
    pub score: u32
}

impl Score {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {score: 0, over: false}))
    }
}

impl StaticComponent for Score {
    fn tick(&mut self, engine: &mut engine::state::Engine) {
        if !self.over {
            engine.add_ui_element(engine::ui::UIElement::Text(engine::ui::text::Text { content: String::from("Score: ".to_owned() + &self.score.to_string().to_owned()), position: cgmath::Point2 { x: -335.0, y: -275.0 }, color: [0.0, 1.0, 0.0, 1.0], origin: engine::ui::text::TextOrigin::Center }));
        }
        else {
            engine.add_ui_element(engine::ui::UIElement::Text(engine::ui::text::Text { content: String::from("Game Over. Final Score: ".to_owned() + &self.score.to_string().to_owned()), position: cgmath::Point2 { x: 0.0, y: 0.0 }, color: [0.0, 1.0, 0.0, 1.0], origin: engine::ui::text::TextOrigin::Center }));
            engine.pause();
        }
    }

    fn name(&mut self) -> String {
        "Score".to_string()
    }
}

struct Spawner {
    pub enemies: Vec<i32>,
    pub last_spawn: Option<OxidizedInstant>,
    pub cooldown: Duration,
    pub player: i32,
    pub moveamt: f32,
    pub scorer: Arc<Mutex<Score>>
}

impl Spawner {
    pub fn new(player: i32, scorer: Arc<Mutex<Score>>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            enemies: Vec::new(),
            last_spawn: None,
            cooldown: Duration::from_secs_f32(0.5),
            player,
            moveamt: 1.0,
            scorer
        }))
    }

    pub fn spawn(&mut self, e: &mut engine::state::Engine, mut bounds: Bounds2D) {
        // println!("tick");
        let mut rng = rand::thread_rng();
        // println!("{} {}", bounds.x(), bounds.y());
        let mut pos = Point {
            x: rng.gen_range(bounds.x()..bounds.x() * 2.0),
            y: rng.gen_range(bounds.y()..bounds.y() * 2.0),
            z: 0.0
        };

        pos.x -= bounds.x();
        pos.y -= bounds.y();
        
        let enemy = e.add_object(make_base_game_object("enemy ".to_owned() + &self.enemies.len().to_string().to_owned()));

        let op = GameObject::find_by_id(enemy).clone();

        let exp = op.expect("cannot find");

        let mut lock = exp.lock().unwrap();


        lock.add_component(RenderComponent::new(Primitives::Octagon(0.1, [1.0, 0.0, 0.0])));

        lock.add_collider(Arc::new(Mutex::new(OctagonCollider::new(0.1))));

        lock.get_component_closure::<Transform>(|trans| {
            trans.pos[0] = pos.x;
            trans.pos[1] = pos.y;
        });

        drop(lock);

        self.enemies.push(enemy);
    }
}

impl StaticComponent for Spawner {
    fn tick(&mut self, e: &mut engine::state::Engine) {
        if let Some(last_spawn) = self.last_spawn {
            println!("{:?}", OxidizedInstant::now());
            if OxidizedInstant::now() >= last_spawn + self.cooldown {
                self.spawn(e, Bounds2D::new(2.7, 2.0));
                self.last_spawn = Some(OxidizedInstant::now());
            }
        } else {
            self.spawn(e, Bounds2D::new(2.7, 2.0));
            self.last_spawn = Some(OxidizedInstant::now());
        }

        let player_obj = GameObject::find_by_id(self.player).clone();
        let player_exp = player_obj.expect("no");
        let mut player_lock = player_exp.lock().unwrap();
        let mut player_pos = [0.0, 0.0, 0.0];

        player_lock.get_component_closure::<Transform>(|trans| {
            player_pos = trans.pos;
        });

        drop(player_lock);

        for enemy in self.enemies.iter() {
            let obj = GameObject::find_by_id(*enemy).clone();
            if obj.is_none() {
                continue;
            }

            let exp = obj.expect("ok");
            let mut lock = exp.lock().unwrap();

            lock.get_component_closure::<Transform>(|trans| {
                let pos = trans.pos;
                let dt = e.dt.unwrap_or(Duration::from_secs(0));

                // // Calculate direction towards player
                let direction = [
                    player_pos[0] - pos[0],
                    player_pos[1] - pos[1],
                    player_pos[2] - pos[2],
                ];

                // Normalize direction
                let distance = (direction[0].powi(2) + direction[1].powi(2) + direction[2].powi(2)).sqrt();

                if distance < 0.1 {
                    self.scorer.lock().unwrap().over = true;
                    return;
                }

                if distance > 0.0 {
                    let normalized_direction = [
                        direction[0] / distance,
                        direction[1] / distance,
                        direction[2] / distance,
                    ];

                    // Move enemy towards player
                    trans.pos[0] += normalized_direction[0] * self.moveamt * dt.as_secs_f32();
                    trans.pos[1] += normalized_direction[1] * self.moveamt * dt.as_secs_f32();
                    trans.pos[2] += normalized_direction[2] * self.moveamt * dt.as_secs_f32();
                }
            });
        }
    }

    fn name(&mut self) -> String {
        "Spawner".to_string()
    }
}

struct ShootComponent {
    pub state: ComponentState,
    pub cooldown: Duration,
    pub last_pressed: Option<OxidizedInstant>,
    pub score: Arc<Mutex<Score>>
}

impl ShootComponent {
    pub fn new(score: Arc<Mutex<Score>>) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            state: ComponentState::new(),
            cooldown: Duration::from_secs_f32(0.5),
            last_pressed: None,
            score
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Input(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }

}

impl ComponentTrait for ShootComponent {
    fn name(&self) -> &str {
        "ShootComponent"
    }

    fn state(&mut self) -> &mut engine::component::ComponentState {
        &mut self.state
    }
}

impl InputTickBehavior for ShootComponent {
    fn tick_with_input(&mut self, input: &engine::component::InputData, obj: &mut GameObject, dt: Duration) {
        if let Some(last_pressed) = &self.last_pressed {
            if OxidizedInstant::now() >= *last_pressed + self.cooldown {
                self.last_pressed = None
            } else { 
                return;
            }
        }

        let mut _pos: Option<[f32; 3]> = None;
        let mut _rot: Option<[f32; 3]> = None;

        obj.get_component_closure::<Transform>(|trans| {
            _pos = Some(trans.pos.clone());
            _rot = Some(trans.rot.clone());
        });

        if _pos.is_none() || _rot.is_none() { return };

        let mut pos = _pos.expect("idk how");  
        let mut rot = _rot.expect("idk how");

        let mut point = Point {
            x: pos[0],
            y: pos[1],
            z: pos[2]
        };

        for key in input.keys_pressed.clone() {
            match key {
                winit::event::VirtualKeyCode::Space => {
                    self.last_pressed = Some(OxidizedInstant::now());
                    let cast = raycast::Raycast::send(point.clone(), rot[2] + 90.0, 1000.0, vec![obj.id()]);
                    let mut result = cast.unwrap();

                    if result.underlying.len() > 0 { 
                        // println!("hits: {}", result.underlying.len()); 
                        obj.get_component_closure::<BulletRenderer>(|renderer| {
                            renderer.set_thickness_timeout(0.1, Duration::from_secs(0));
                            renderer.tick();
                            renderer.set_thickness_timeout(0.01, Duration::from_secs_f32(0.2));
                        });

                        for enemy in result.underlying.iter_mut() {
                            enemy.lock().unwrap().destroy();
                            self.score.lock().unwrap().score += 1;
                        }
                    };
                },
                _ => {}
            }
        }
    }
}

struct BulletRenderer {
    pub state: ComponentState,
    pub thickness: f32,
    pub to_set_thickness: f32,
    pub timeout_end: Option<OxidizedInstant>,
}

impl BulletRenderer {
    pub fn new() -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            state: ComponentState::new(),
            thickness: 0.01,
            to_set_thickness: 0.01,
            timeout_end: None,
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Render(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }

    pub fn set_thickness_timeout(&mut self, thickness: f32, timeout: Duration) {
        self.to_set_thickness = thickness;
        self.timeout_end = Some(OxidizedInstant::now() + timeout);
    }

    fn tick(&mut self) {
        if let Some(timeout_end) = &self.timeout_end {
            if OxidizedInstant::now() >= *timeout_end {
                self.thickness = self.to_set_thickness;
                self.timeout_end = None;
            }
        }
    }
}

impl ComponentTrait for BulletRenderer {
    fn name(&self) -> &str {
        "BulletRenderer"
    }

    fn state(&mut self) -> &mut engine::component::ComponentState {
        &mut self.state
    }
}

impl RenderTickBehavior for BulletRenderer {
    fn render_tick(&mut self, obj: &mut GameObject, dt: Duration, cam: engine::camera::Camera) -> engine::component::RenderOutput {
        self.tick();
        let mut _pos: Option<[f32; 3]> = None;
        let mut _rot: Option<[f32; 3]> = None;

        obj.get_component_closure::<Transform>(|trans| {
            _pos = Some(trans.pos.clone());
            _rot = Some(trans.rot.clone());
        });

        if _pos.is_none() || _rot.is_none() { return RenderOutput { obj: None } };

        let mut pos = _pos.expect("idk how");  
        let mut rot = _rot.expect("idk how");

        let mut point = Point {
            x: pos[0],
            y: pos[1],
            z: pos[2]
        };

        let obj: Option<Box<dyn Object>> = Some(Box::new(raycast::Raycast::show(point.clone(), rot[2] + 90.0, 100.0, self.thickness)));

        RenderOutput { obj }
    }
}

async fn run() {
    save::init();
    // let (mut e, eventloop) =
    //     engine::state::Engine::new(true, EventLoopBuilder::<()>::with_user_event().build()).await;

    impl_save_load!(ShootComponent, ShootComponentSaveData, input, 
        state: ComponentState, 
        cooldown: Duration, 
        last_pressed: Option<OxidizedInstant>, 
        score: Arc<Mutex<Score>>
    );

    impl_save_load!(BulletRenderer, BulletRendererSaveData, render, 
        state: ComponentState, 
        thickness: f32, 
        to_set_thickness: f32, 
        timeout_end: Option<OxidizedInstant>
    );

    impl_static_save_load!(Score, ScoreSaveData, over: bool, score: u32);

    impl_static_save_load!(Spawner, SpawnerSaveData, enemies: Vec<i32>, last_spawn: Option<OxidizedInstant>, cooldown: Duration, player: i32, moveamt: f32, scorer: Arc<Mutex<Score>>);

    //
    // let ship = e.add_object(gameobject::make_base_game_object(String::from("ship")));
    // //
    // let scorer = Score::new();
    //
    // gameobject::add_component(ship, CharacterController2D::new(Some(Bounds2D::new(2.7, 2.0))));
    // gameobject::add_component(ship, RenderComponent::new(Primitives::Triangle(0.1, [0.0, 1.0, 0.0])));
    // gameobject::add_collider(ship, Arc::new(Mutex::new(CubeCollider::new(0.1))));
    // gameobject::add_component(ship, BulletRenderer::new());
    // gameobject::add_component(ship, ShootComponent::new(scorer.clone()));
    //
    // let enemy = e.add_object(make_base_game_object("enemy ".to_owned()));
    // let op = GameObject::find_by_id(enemy).clone();
    // let exp = op.expect("cannot find");
    // let mut lock = exp.lock().unwrap();
    //
    // lock.add_component(RenderComponent::new("enemy render".to_string(), Primitives::Octagon(0.1, [1.0, 0.0, 0.0])));
    //
    // lock.add_collider(Arc::new(Mutex::new(OctagonCollider::new(0.1))));
    //
    // drop(lock);

    // e.add_static(scorer.clone());
    // e.add_static(Spawner::new(ship, scorer));

    let (mut e, eventloop) =
        // engine::state::Engine::import(EngineSaveData {
        //     objects: Vec::new(),
        //     static_components: Vec::new(),
        //     graphics: true
        // }).await;
        engine::state::Engine::import_from_json(String::from("{\"objects\":[{\"components\":[{\"id\":\"Transform\",\"data\":{\"pos\":[0.0,0.0,0.0],\"rot\":[0.0,0.0,0.0],\"state\":{\"_state\":null}}},{\"id\":\"CharacterController2D\",\"data\":{\"bounds\":{\"limits\":{\"x\":{\"x\":2.700000047683716},\"y\":{\"y\":2.0}}},\"moveamt\":0.009999999776482582,\"rotamt\":2.0,\"state\":{\"_state\":null}}},{\"id\":\"RenderComponent\",\"data\":{\"name\":\"RenderComponent\",\"obj\":{\"Triangle\":[0.10000000149011612,[0.0,1.0,0.0]]},\"state\":{\"_state\":null}}},{\"id\":\"BulletRenderer\",\"data\":{\"state\":{\"_state\":null},\"thickness\":0.009999999776482582,\"timeout_end\":null,\"to_set_thickness\":0.009999999776482582}},{\"id\":\"ShootComponent\",\"data\":{\"cooldown\":{\"nanos\":500000000,\"secs\":0},\"last_pressed\":null,\"score\":{\"over\":false,\"score\":0},\"state\":{\"_state\":null}}}],\"colliders\":[{\"collider\":{\"CubeCollider\":{\"side_length\":0.1}}}],\"parent\":null,\"children\":[],\"id\":0,\"name\":\"ship\",\"active\":true}],\"static_components\":[{\"id\":\"Score\",\"data\":{\"over\":false,\"score\":0}},{\"id\":\"Spawner\",\"data\":{\"cooldown\":{\"nanos\":500000000,\"secs\":0},\"enemies\":[],\"last_spawn\":null,\"moveamt\":1.0,\"player\":0,\"scorer\":{\"over\":false,\"score\":0}}}],\"graphics\":true}")).await;        

    // e.add_object(make_base_game_object("ok".to_owned()));

    let e = Arc::new(Mutex::new(e));

    println!("{:?}", e.lock().unwrap().export_raw());

    engine::state::Engine::run(e, eventloop);
}
