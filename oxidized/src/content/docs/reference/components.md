---
title: Components
description: Components 
---

`engine::components`

Components in oxidized are just `structs` that implement a tick behaviour and the general component trait

## Component Trait

The main definition for a component comes from the trait:

```rust
pub trait ComponentTrait: Send + Sync + Downcast + ComponentSaveLoad
where
    Self: 'static,
{
    fn name(&self) -> &str;
    fn state(&mut self) -> &mut ComponentState;
}
```

You only need to worry about implementing the `name()` and the `state()` functions

As an example on how to implement the `state()` method simply:

```rust
struct TestComponent {
    ...
    pub state: ComponentState,
    ...
}

impl TestComponent {
    ...
}

impl ComponentTrait for TestComponent {
    fn name(&self) -> &str {
        "TestComponent"
    }

    fn state(&mut self) -> &mut engine::component::ComponentState {
        &mut self.state
    }
}
```

The second piece is a bit harder to implement, the tick behaviour. Each component can choose from 3 tick behaviours:
1. Default
2. Input 
3. Render

### Default

The default tick behaviour is when you don't need any IO for your component and it just provides a background process todo with only your `GameObject`.

To implement it use this trait:
```rust
pub trait TickBehavior: Send + Sync {
    fn tick(&mut self, obj: &mut GameObject, dt: Duration);
}
```

### Input

The input tick behaviour when you want to use some input data to control what happens in the component.

The input data passed in follows this schema:

```rust
pub struct InputData {
    pub keys_pressed: Vec<winit::event::VirtualKeyCode>,
    pub mouse_buttons_pressed: Vec<winit::event::MouseButton>,
    pub mouse_position: (f64, f64),
}
```

To implement the behaviour use:

```rust
pub trait InputTickBehavior: Send + Sync {
    fn tick_with_input(&mut self, input: &InputData, obj: &mut GameObject, dt: Duration);
}
```

### Render 

The render tick behaviour should **only** be used to provide a render output to the graphics backend.

The `RenderOutput` follows:

```rust
pub struct RenderOutput {
    pub obj: Option<Box<dyn Object>>,
}
```

:::note 
See [graphics](graphics) for details on the `Object` trait
:::

To implement the render tick use:

```rust
pub trait RenderTickBehavior: Send + Sync {
    fn render_tick(&mut self, obj: &mut GameObject, dt: Duration, cam: camera::Camera) -> RenderOutput;
}
```

### Constructor 

The constructor for a component must be done in a **very** specific way to avoid errors.

For a **normal** component:

```rust
impl Component {
    pub fn new(...) -> Arc<Mutex<ComponentWrapper>> {
        let component = Arc::new(Mutex::new(Self {
            ...
        }));

        ...

        let tick_variant = Arc::new(Mutex::new(TickVariant::Input(component.clone())));
        Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
    }
}

impl InputTickBehavior for Component { ... };
```

The only thing that changes between different tick types is the `TickVariant`:

```rust
pub enum TickVariant {
    Input(Arc<Mutex<dyn InputTickBehavior>>),
    Render(Arc<Mutex<dyn RenderTickBehavior>>),
    Default(Arc<Mutex<dyn TickBehavior>>),
}
```

## Save & Load

Oxidized can save and load engine state for distribution of your game. It can export to a struct or to raw json. It does this through macros.

### Normal Components

:::tip
Normal components include Default, Input and Render tick behaviours.
:::

To implement save and load for the normal components use the macro:

```rust
impl_save_load!()
```

- The first argument of the macro is the component struct name 
- The second is the name for the save data struct (**make sure to not name 2 save structs the same thing**)
- The third agrument is for the tick type
    - If it is an input component use "input" (*without quotes*)
    - If it is a render component use "render" (*without quotes*)
    - If it is a default component use "default" (*without quotes*)
- The fourth argument is the list of fields that should be saved as part of the component's state
    - This is written in the format `{ field_name: field_type, ... }`
    - These fields are copied directly into the save struct and will be serialized/deserialized accordingly
- The fifth argument is the list of linked fields, which reference other components or objects
    - This is written in the format `{ link_field_name: link_field_type, ... }`
    - These fields are stored as `Link<T>` objects in the save struct, allowing them to reference other saved entities via UUIDs
    - When saving, the macro ensures that linked components are properly registered and assigned UUIDs
    - When loading, the macro attempts to retrieve the linked components using their saved UUIDs

#### Example Usage

```rust
impl_save_load!(PlayerComponent, PlayerComponentSaveData, default, 
    {
        health: u32,
        stamina: f32,
        name: String
    },
    {
        inventory: InventoryComponent
    }
);
```

##### What Happens Here?

- The macro expands to define a save struct (PlayerComponentSaveData) that stores:
    - `health: u32`
    - `stamina: f32`
    - `name: String`
    - `inventory` as a linked field (`Link<InventoryComponent>`)
- Implements the ComponentSaveLoad trait for PlayerComponent, defining:
    - `to_save_data()`, which serializes the component
    - `from_save_data()`, which reconstructs the component from its saved state
    - Registers `PlayerComponent` for saving and loading.

### Static Components

The static component macro:

```rust
impl_static_save_load!()
```

Works in the same way as the above macro `impl_save_load!()`, **except with one major difference**:
- The tick behaviour argument is no longer needed so all other arguments move up one position in the order


For example say the `PlayerComponent` was static instead of attached to a `GameObject`:

```rust
impl_static_save_load!(PlayerComponentStatic, PlayerComponentStaticSaveData, 
    {
        health: u32,
        stamina: f32,
        name: String
    },
    {
        inventory: InventoryComponent
    }
);
```

## Pre-Loaded Components

Oxidized comes with 2 pre-loaded components:
- `CharacterController2D`
- `Transform`

### CharacterController2D

The `CharacterController2D` is a simple controller using WASD and Left/Right arrow for rotation, to move a `GameObjects` `Transform` component around the screen, within specified bounds.

To construct this component use:
```rust 
gameobject::add_component(id, CharacterController2D::new(Some(Bounds2D::new(2.7, 2.0))));
```

Where `2.7` and `2.0` are the x and y boundries on the screen.

### Transform

The `Transform` component only serves one purpose and that is to keep track of the current position of the `GameObject` in world space. It stores `(x, y, z)` position but also `(roll, pitch, yaw)` for rotation.

:::note 
GameObjects that were created using `make_base_game_object()` already have a Transform component applied
:::

To edit values in the `Transform` component use:

```rust
let obj = GameObject::find_by_id(id).clone();
let exp = player_obj.expect("no gameobject found");
let mut lock = exp.lock().unwrap();
lock.get_component_closure::<Transform>(|transform| {});
```

Inside the closure you have access to the following fields on `transform`:

```rust
pub struct Transform {
    pub state: ComponentState,
    pub pos: [f32; 3],
    pub rot: [f32; 3],
}
```

:::caution 
Avoid editing the `state` as it is used internally in oxidized
:::
