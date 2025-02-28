---
title: Engine
description: The base class 
---

```rust
use engine::state::Engine; 
```

## Constructor

To construct the `Engine` class:

```rust
let (mut e, eventloop) = Engine::new(true, EventLoopBuilder::<()>::with_user_event()
    .build()).await;
```

The engine requires a user constructed eventloop to function correctly (the second argument). But it can also take another argument, `graphics` (the first arg). If this is `true` then it will construct a [`Renderer`](../renderer) and spawn a rendering thread. **ONLY** enable this if you intend to use the inbuilt graphics library because it will consume resources while running.

## Interaction with [`GameObjects`](../gameobject)

*See [`GameObject`](gameobject) for details on creating and using them*

To add a gameobject to the game engine so it can be used in your game you must use:

```rust
engine.add_object(obj)
```

with `obj` being your constructed GameObject (see [`GameObject::make_base_game_object`](../gameobject#make_base_game_object))

This will do 2 things:
1. Add your game object to the internal physics engine
2. Adds your game object to the internal engine state (for managing the components)

## Static Components

A static component is built from the same concept as a normal [`Component`](../component) except is is applied to the engine as if the engine was its own game object.

For example, if you wanted to make a shared state for all of your gameobjects, like a score counter you should use a static component so it can sync through all other sections of your game.

To make a static component you **must** implement the following trait for your struct:

```rust
pub trait StaticComponent: Send + Sync + Downcast + StaticComponentSaveLoad + Debug
where Self: 'static {
    fn tick(&mut self, engine: &mut Engine);
    fn name(&mut self) -> String;
}
```

Where:
- `tick()` is called every tick of the engine and a mutable reference of the `Engine` passed in 
- `name()` should return the name of the static component (**needs to be unique**)

You *can* add your new component with:

```rust
struct Static;
impl StaticComponent for Static { ... };

unsafe { 
    engine.add_static(Arc::new(Mutex::new(Static::new())));
}
```

:::caution 
This method is marked as unsafe because it doesn't automatically create a `Link`. Depending on how you use it, it may not export properly.
:::

To do it safely use:

```rust 
struct Static;
impl StaticComponent for Static { ... };

let s = Arc::new(Mutex::new(Static::new()));
let s_link: Link<Static> = engine.add_static_link(s.clone()); // This returns a Link<T>
```

You can then use the `Link<T>` later and this will automatically sync data between components. The link **will** be automatically saved and exported as opposed to the `engine.add_static(e)`.

For saving static components to the JSON see [components](components)

## UI 

*See [`UIElement`](../uielement) for more information*

To display something on the screen, this method **must** be called every tick, preferably *by* some static component. 

Use:

```rust
impl Engine {
    ...
    pub fn add_ui_element(&mut self, element: UIElement);
    ...
}
```

## Pause & Unpause

Pausing your engine will:
- Stop the rendering
    - Stop the ui elements from updating
    - Stop the drawn sprites from updating
- Stop the ticks of components

:::note
Keep in mind that any updates to game objects will not take effect until the engine is unpaused.
:::

### To Pause

```rust
engine.pause();
```

### To Unpause

```rust
engine.unpause();
```

## Quit

To exit the application and clean up the rendering thread use:

```rust
engine.quit();
```

## Export & Import 

To export your game to a json which can be loaded again and run:

```rust
engine.export_raw();
```

This returns a `String` of a json

You can also export with

```rust
engine.export();
```

which returns an instance of EngineSaveData which represents all the data required to reconstruct the engine state.

You can import raw json (which may be better for distributing your game) with:

```rust
let data: String = ""; // some json data
let (mut engine, eventloop) = engine.import_from_json(data);
```

To import the save data struct (`EngineSaveData`)

```rust 
let mut data: EngineSaveData = EngineSaveData { ... }; // DATA MUST BE MUTABLE

let (mut engine, eventloop) = engine.import(data);
```

## Running the engine

To run the engine use:

```rust
let (mut e, eventloop) =
    engine::state::Engine::new(true, EventLoopBuilder::<()>::with_user_event().build()).await;
let e = Arc::new(Mutex::new(e));
engine::state::Engine::run(e, eventloop);
```

:::note 
Make sure to pass in the initially created `eventloop` and to construct an `Arc<Mutex<Engine>>` to pass in to the run method
:::
