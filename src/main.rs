mod engine;

#[allow(unused)]
use crate::engine::component::ComponentTrait;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let mut e = engine::state::Engine::new(true).await;

    let g1 = e.add_object(engine::gameobject::make_base_game_object(String::from(
        "thing1",
    )));

    engine::gameobject::add_component(g1, engine::components::make_safe(engine::component::LambdaComponent::new(String::from("lambda"), move || {println!("tick")})));
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
    e.tick();
}
