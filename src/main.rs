mod engine;
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[allow(unused)]
use crate::engine::component::ComponentTrait;
use engine::{component::Transform, gameobject::GameObject, graphics_backend::color::Colors};
use winit::event_loop::EventLoopBuilder;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let (mut e, eventloop) =
        engine::state::Engine::new(true, EventLoopBuilder::<()>::with_user_event().build()).await;

    // let g1 = e.add_object(engine::gameobject::make_base_game_object(String::from(
    //     "thing1",
    // )));
    //
    // engine::gameobject::add_component(
    //     g1,
    //     engine::components::RenderComponent::new(String::from("i"), Colors::BLUE),
    // );

    // engine::gameobject::add_component(g1, engine::component::CharacterController2D::new());

    // engine::gameobject::add_collider(
    //     g1,
    //     Arc::new(Mutex::new(engine::collider::CubeCollider::new(0.1))),
    // );

    // engine::gameobject::add_component(g1, engine::component::Rigidbody::new(true, true));

    // engine::gameobject::add_collider
    //
    // let g2 = e.add_object(engine::gameobject::make_base_game_object(String::from(
    //     "thing2",
    // )));
    //
    // engine::gameobject::add_component(
    //     g2,
    //     engine::components::RenderComponent::new(String::from("i"), Colors::GREEN),
    // );
    //
    // engine::gameobject::add_collider(
    //     g2,
    //     Arc::new(Mutex::new(engine::collider::CubeCollider::new(0.1))),
    // );

    // engine::gameobject::add_component(g2, engine::component::Rigidbody::new(true, true));

    // GameObject::find_by_id(g2)
    //     .expect("nahh")
    //     .lock()
    //     .unwrap()
    //     .get_component_closure::<Transform>(|transform| {
    //         transform.inner[1] = 1.0;
    //     });

    engine::state::Engine::run(Arc::new(Mutex::new(e)), eventloop);
}
