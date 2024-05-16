mod engine;
use std::sync::{Arc, Mutex};

#[allow(unused)]
use crate::engine::component::ComponentTrait;
use crate::engine::component::InputTickBehavior;

// use crate::engine::state::CustomEvent;
use colored::CustomColor;
use std::thread;
use std::time::Duration;
use winit::event_loop::{EventLoop, EventLoopBuilder};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let (mut e, mut eventloop) = engine::state::Engine::new(
        true,
        EventLoopBuilder::<()>::with_user_event().build(),
    )
    .await;

    let g1 = e.add_object(engine::gameobject::make_base_game_object(String::from(
        "thing1",
    )));
    // //
    // engine::gameobject::add_component(g1, engine::component::LambdaComponent::new(String::from("ok"), move || {println!("ok")}));

    engine::gameobject::add_component(
        g1,
        engine::components::RenderComponent::new(String::from("i")),
    );
    // let mut c = engine::components::InputComponent::new(String::from("k"));

    // let g2 = e.add_object(engine::gameobject::make_base_game_object(String::from(
    //     "child",
    // )));
    // //
    // engine::gameobject::reparent(g1, g2);
    //
    // engine::gameobject::add_component(
    //     g1,
    //     engine::components::make_safe(engine::components::Input::new()),
    // );
    // //
    // engine::gameobject::add_component(
    //     g2,
    //     engine::components::make_safe(engine::components::Input::new()),
    // );
    //
    // engine::gameobject::get_component::<engine::components::Input, _>(g2, |comp| {
    //     println!("{}", comp.name());
    // });

    // loop {
    // e.tick();
    // }
    // e.tick();
    e.run(eventloop);
    // thread::spawn(|| {e.renderer.run()});

    // loop {
    //     e.tick()
    // }

    // e.renderer.run();

    // thread::sleep(Duration::from_secs(10));

    // e.renderer.run();
    // println!("done");
}
