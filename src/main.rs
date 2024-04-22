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
        g2,
        engine::components::make_safe(engine::components::Input::new()),
    );

    engine::gameobject::to_object(g1, |obj| {
        let op = obj.get_component::<engine::components::Input>();
        if op.is_some() {
            let arc = op.unwrap();
            println!("{}", arc.lock().unwrap().name()); 
        }
    })

    // e.tick();
}
