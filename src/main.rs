mod engine;

fn main() {
    let mut e = engine::state::Engine::new();

    let g1 = e.add_object(engine::gameobject::make_base_game_object(String::from(
        "parent",
    )));
    let g2 = e.add_object(engine::gameobject::make_base_game_object(String::from(
        "child",
    )));

    engine::gameobject::reparent(g1, g2);

    engine::gameobject::add_component(
        g1,
        engine::components::make_safe(engine::components::Input::new()),
    );

    engine::gameobject::add_component(
        g1,
        engine::components::make_safe(engine::components::Input::new()),
    );

    e.tick();
}
