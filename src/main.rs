mod engine;
use std::sync::{Arc, Mutex};

#[allow(unused)]
use crate::engine::component::ComponentTrait;
use winit::event_loop::EventLoopBuilder;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let (mut e, eventloop) =
        engine::state::Engine::new(true, EventLoopBuilder::<()>::with_user_event().build()).await;

    let g1 = e.add_object(engine::gameobject::make_base_game_object(String::from(
        "thing1",
    )));

    engine::gameobject::add_component(
        g1,
        engine::components::RenderComponent::new(String::from("i")),
    );

    engine::gameobject::add_component(
        g1,
        engine::components::RenderComponent::new(String::from("i")),
    );

    engine::state::Engine::run(Arc::new(Mutex::new(e)), eventloop);
}
