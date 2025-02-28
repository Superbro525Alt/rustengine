---
title: Game Object
description: The game object 
---

```rust
use engine::gameobject
```

A GameObject in oxidized represents a structure that can have infinite components aattached which can do actions. The gameobject itself cannot interact with anything.

`GameObjects` work in a hierarchy meaning that all gameobjects have:
- A parent
- Children

They also contain:
- Colliders
- Components

## Constructor

To manually create a `GameObject` you can use:

```rust
let g: GameObject = GameObject::new("name", vec![], GameObjectState::new(true, vec![], vec![]));
```

The `GameObjectState` follows the following schema:

```rust
#[derive(Clone)]
pub struct GameObjectState {
    pub active: bool,
    pub parent_id: Option<i32>,
    pub child_ids: Vec<i32>,
}

impl GameObjectState {
    pub fn new(active: bool, parent_id: Option<i32>, child_ids: Vec<i32>) -> Self;
    ...
}
```

And the constructor of `GameObject`:

```rust
impl GameObject {
    pub unsafe fn new(
        name: String,
        components: Vec<Arc<Mutex<component::ComponentWrapper>>>,
        state: GameObjectState,
    ) -> Arc<Mutex<Self>>;
}
```

:::caution 
This is an unsafe method as it provides fine control over the specifics of the object
:::

The method detailed below is preferable for the construction of a *safe* `GameObject`:

```rust
pub fn make_base_game_object(name: String) -> Arc<Mutex<GameObject>>;
```

:::tip 
This constructs a `GameObject` places it in an `Arc<Mutex<GameObject>>` for thread safety. It also attaches a base transform component with an origin of `(0, 0, 0)`
:::

## Getting by ID

To get a `GameObject` by its assigned ID use:

```rust
impl GameObject {
    ...
    pub fn find_by_id(id: i32) -> Option<Arc<Mutex<Self>>>;
    ...
}
```

This will return an `Option`, if the id exists, the option will contain the reference to the GameObject, otherwise it will raise an error when you unwrap it.

## Component Related Methods

In an ECS engine (like oxidized), `GameObject`s work off components. Below are the details for all required operations todo with these components (see more about components [here](components))

:::caution 
There is no currently implemented method to remove a component. 
:::

### Adding Components

To add a component to a `GameObject`, you must first get a pointer to the object (using an `Arc<Mutex<GameObject>>`). This can be done in a variety of ways like find_by_id (see above). 

Once you obtain the pointer (assume `g` is an `Arc<Mutex<GameObject>>`):

```rust
// This example adds a RenderComponent that renders an octagon
let mut lock = g.lock().unwrap();

lock.add_component(RenderComponent::new(Primitives::Octagon(0.1, [1.0, 0.0, 0.0])));
```

Tou can also use the utility method:

```rust
gameobject::add_component(id, RenderComponent::new(...));
```

### Getting Components

If you want to edit the details of a component or call methods at certain times you can do this mutably with:

```rust
lock.get_component_closure::<T>(|component| {
    component.some_method();
    component.some_value = 0;
});
```

Where `T` can be the type of any component on the `GameObject`. The method will return an `Option<()>`, if this option is `None` then it failed to find the component and the closure did not get called.

### Has Component

To check if a `GameObject` has a particular component use:

```rust
let has = lock.has_component::<T>();
```

The return value is a `bool` representing if the component of type `T` was found on the `GameObject`


## Destroy

To destroy your `GameObject` means a few things:
- The components will stop ticking (updating)
- The `GameObject` will stop rendering
- All previous references to the `GameObject` will become irrelevant
- Any attempt to get it by id or to get a component from it will result in either a runtime error or return `None`

:::note 
There is no way to undestroy a `GameObject`
:::

To destroy it use:

```rust
lock.destroy();
```

## Reparent

To reparent a `GameObject` to a new parent object use the utility method in `engine::gameobject`:

```rust
pub fn reparent(parent_id: i32, child_id: i32);
```

## Collisions

:::note 
For more information on colliders and the creation of them see [colliders](colliders)
:::

`GameObjects` have utility for collisions, by attaching [collider](colliders) objects to the `GameObject` it will automatically update the colliders origin to the `GameObjects` current transform. It also provides methods to check all of a `GameObjects` colliders for a collision with another collider or a point. This can be used for physics or hitboxes.

:::tip
Colliders on `GameObjects` is how [`Raycasts`](raycast) detect
:::

### Adding a Collider

To add a collider use the following utility method:

```rust
pub fn add_collider(obj_id: i32, coll: Arc<Mutex<Box<dyn Collider>>>);
```

This method can once again be found in `engine::gameobject::add_collider`

### Collisions

To detect a collision with another collider use `engine::gameobject::colliding_with`

```rust
pub fn colliding_with(
    obj_id: i32,
    other: Arc<Mutex<Box<dyn Collider>>>,
    other_pos: collider::Point,
) -> bool;
```

Keep in mind that colliders don't keep an origin internally so you have to specify the origin of the other `Collider`

To detect a collision with another point use `engine::gameobject::colliding_point`

```rust
pub fn colliding_point(obj_id: i32, other: collider::Point) -> bool;
```

:::tip 
The point type simply represents a 3d vector (x, y, z) and allows for mathamatical operations on it. To create one see [point](colliders#point)
:::
