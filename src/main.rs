mod engine;

#[allow(unused)]
use crate::engine::component::ComponentTrait;

fn main() {
    run();
}

async fn run() {
    let mut e = engine::state::Engine::new();

    // let g1 = e.add_object(engine::gameobject::make_base_game_object(String::from(
    //     "parent",
    // )));
    // let g2 = e.add_object(engine::gameobject::make_base_game_object(String::from(
    //     "child",
    // )));
    //
    // engine::gameobject::reparent(g1, g2);
    //
    // engine::gameobject::add_component(
    //     g1,
    //     engine::components::make_safe(engine::components::Input::new()),
    // );
    //
    // engine::gameobject::add_component(
    //     g2,
    //     engine::components::make_safe(engine::components::Input::new()),
    // );

    // engine::gameobject::get_component::<engine::components::Input, _>(1, |comp| {
    //     println!("{}", comp.name());
    // });

    // loop {
    // e.tick();
    // }
    e.await.renderer.run();
}
