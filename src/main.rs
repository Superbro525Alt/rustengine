mod engine;
use std::sync::{Arc, Mutex};

#[allow(unused)]
use crate::engine::component::ComponentTrait;
use crate::engine::component::InputTickBehavior;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut e = engine::state::Engine::new(true).await;

    let g1 = e.lock().unwrap().add_object(engine::gameobject::make_base_game_object(String::from(
        "thing1",
    ), e.clone()));
    // //
    // engine::gameobject::add_component(g1, engine::component::LambdaComponent::new(String::from("ok"), move || {println!("ok")}));

    engine::gameobject::add_component(g1, engine::components::InputComponent::new(String::from("i")));
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
    // e.renderer.run();
    e.lock().unwrap().tick();
}
