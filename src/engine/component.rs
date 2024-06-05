use crate::engine::gameobject::GameObject;
use crate::engine::graphics_backend::object::Object;
use crate::engine::graphics_backend::vertex::Vertex;
use crate::impl_save_load;
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use serde_json::Value;
use std::any::{self, Any, TypeId};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Serialize, Deserialize};

use super::bounds;
use super::bounds::Bounds2D;
use super::camera;
use crate::engine::save::{ComponentSaveLoad};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentState {
    _state: Value,
}

impl ComponentState {
    pub fn new() -> Self {
        Self {
            _state: Value::Null,
        }
    }
    pub fn get(&self) -> Value {
        self._state.clone()
    }
    pub fn set(&mut self, value: Value) {
        self._state = value;
    }
}

impl Default for ComponentState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ComponentTrait: Send + Sync + Downcast + ComponentSaveLoad
where
    Self: 'static,
{
    fn name(&self) -> &str;
    fn state(&mut self) -> &mut ComponentState;
    fn to_save_data(&self) -> Value {
        ComponentSaveLoad::to_save_data(self)
    }
    // fn tick_type(&mut self) -> &mut TickVariant;
}

impl_downcast!(ComponentTrait);

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Input,
    Render,
    Default,
}

pub struct InputData {
    pub keys_pressed: Vec<winit::event::VirtualKeyCode>,
    pub mouse_buttons_pressed: Vec<winit::event::MouseButton>,
    pub mouse_position: (f64, f64),
}

// #[derive(Clone)]
pub struct RenderOutput {
    pub obj: Option<Box<dyn Object>>,
}

impl RenderOutput {
    pub fn raw_desc(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        if self.obj.is_some() {
            return self.obj.as_mut().expect("none").desc_raw();
        }

        (vec![], vec![])
    }
}

pub trait TickBehavior: Send + Sync {
    fn tick(&mut self, obj: &mut GameObject, dt: Duration);
}

pub trait InputTickBehavior: Send + Sync {
    fn tick_with_input(&mut self, input: &InputData, obj: &mut GameObject, dt: Duration);
}

pub trait RenderTickBehavior: Send + Sync {
    fn render_tick(&mut self, obj: &mut GameObject, dt: Duration, cam: camera::Camera) -> RenderOutput;
}

// impl_downcast!(InputTickBehavior);
// impl_downcast!(RenderTickBehavior);
// impl_downcast!(TickBehavior);

// trait Underlying: Sized {
//     fn underlying<T>(&mut self) -> Arc<Mutex<&T>> {
//         Arc::new(Mutex::new(&self.as_any().downcast_ref::<T>().unwrap()))
//     }
// }

pub enum TickVariant {
    Input(Arc<Mutex<dyn InputTickBehavior>>),
    Render(Arc<Mutex<dyn RenderTickBehavior>>),
    Default(Arc<Mutex<dyn TickBehavior>>),
}

impl TickVariant {
    pub fn component_type(&self) -> ComponentType {
        match self {
            TickVariant::Input(_) => ComponentType::Input,
            TickVariant::Render(_) => ComponentType::Render,
            TickVariant::Default(_) => ComponentType::Default,
        }
    }

    // pub fn _underlying(&self) -> Arc<Mutex<&dyn ComponentTrait>> {
    //     match self {
    //         TickVariant::Input(ref behavior) => behavior.underlying::<dyn ComponentTrait>(),
    //         TickVariant::Render(ref behavior) => Arc::clone(behavior),
    //         TickVariant::Default(ref behavior) => Arc::clone(behavior),
    //     }
    // }

    pub fn tick(
        &mut self,
        input: Option<&InputData>,
        obj: &mut GameObject,
        dt: Duration,
        cam: camera::Camera
    ) -> Option<RenderOutput> {
        match self {
            TickVariant::Input(behavior) => {
                if let Some(input) = input {
                    behavior.try_lock().unwrap().tick_with_input(input, obj, dt);
                }
                None
            }
            TickVariant::Render(behavior) => Some(behavior.lock().unwrap().render_tick(obj, dt, cam)),
            TickVariant::Default(behavior) => {
                behavior.lock().unwrap().tick(obj, dt);
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct ComponentWrapper {
    pub component: Arc<Mutex<dyn ComponentTrait>>,
    pub ticker: Arc<Mutex<TickVariant>>,
}

impl ComponentWrapper {
    pub fn new(component: Arc<Mutex<dyn ComponentTrait>>, ticker: Arc<Mutex<TickVariant>>) -> Self {
        Self { component, ticker }
    }

    pub fn tick(
        &self,
        input: Option<&InputData>,
        obj: &mut GameObject,
        dt: Duration,
        cam: camera::Camera
    ) -> Option<RenderOutput> {
        let mut ticker = self.ticker.lock().unwrap();
        ticker.tick(input, obj, dt, cam)
    }
}

pub fn create_component_wrapper(
    component: Arc<Mutex<dyn ComponentTrait>>,
    tick_variant: Arc<Mutex<TickVariant>>,
) -> Arc<Mutex<ComponentWrapper>> {
    Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
}

// pub struct LambdaComponent<F>
// where
//     F: FnMut(),
// {
//     pub name: String,
//     pub state: ComponentState,
//     pub tick_behavior: F,
// }
//
// impl<F> LambdaComponent<F>
// where
//     F: FnMut() + Send + Sync + 'static,
// {
//     pub fn new(name: String, tick_behavior: F) -> Arc<Mutex<ComponentWrapper>> {
//         let lambda_component = Arc::new(Mutex::new(Self {
//             name,
//             tick_behavior,
//             state: ComponentState::new(),
//         }));
//         let tick_variant = Arc::new(Mutex::new(TickVariant::Default(lambda_component.clone())));
//
//         Arc::new(Mutex::new(ComponentWrapper {
//             component: lambda_component as Arc<Mutex<dyn ComponentTrait>>,
//             ticker: tick_variant,
//         }))
//     }
// }
//
// impl<F> ComponentTrait for LambdaComponent<F>
// where
//     F: FnMut() + Send + Sync + 'static,
// {
//     fn name(&self) -> &str {
//         &self.name
//     }
//
//     fn state(&mut self) -> &mut ComponentState {
//         &mut self.state
//     }
// }
//
// impl<F> TickBehavior for LambdaComponent<F>
// where
//     F: FnMut() + Send + Sync + 'static,
// {
//     fn tick(&mut self, obj: &mut GameObject, dt: Duration) {
//         (self.tick_behavior)();
//     }
// }

pub struct Transform {
    pub state: ComponentState,
    pub pos: [f32; 3],
    pub rot: [f32; 3],
}

impl ComponentTrait for Transform {
    fn name(&self) -> &str {
        "Transform"
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl TickBehavior for Transform {
    fn tick(&mut self, obj: &mut GameObject, dt: Duration) {}
}

impl Transform {
    pub fn new() -> Arc<Mutex<ComponentWrapper>> {
        let transform = Arc::new(Mutex::new(Self {
            state: ComponentState::new(),
            pos: [0.0, 0.0, 0.0],
            rot: [0.0, 0.0, 0.0],
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Default(transform.clone())));

        Arc::new(Mutex::new(ComponentWrapper {
            component: transform as Arc<Mutex<dyn ComponentTrait>>,
            ticker: tick_variant,
        }))
    }
}


pub struct CharacterController2D {
    pub moveamt: f32,
    pub state: ComponentState,
    pub rotamt: f32,
    pub bounds: Option<bounds::Bounds2D>
}

impl InputTickBehavior for CharacterController2D {
    fn tick_with_input(&mut self, input: &InputData, obj: &mut GameObject, dt: Duration) {
        let mut bound = self.bounds.clone();

        obj.get_component_closure::<Transform>(|transform| {
            let mut new = transform.pos;
            let mut new_rot = transform.rot;
            // let dt_conv = (dt.as_millis() as f32) / 100.0;
            let dt_conv = 1.0;
            for key in input.keys_pressed.iter() {
                match key {
                    winit::event::VirtualKeyCode::W => new[1] += self.moveamt / dt_conv,
                    winit::event::VirtualKeyCode::S => new[1] -= self.moveamt / dt_conv,
                    winit::event::VirtualKeyCode::A => new[0] -= self.moveamt / dt_conv,
                    winit::event::VirtualKeyCode::D => new[0] += self.moveamt / dt_conv,

                    winit::event::VirtualKeyCode::Left => new_rot[2] += self.rotamt / dt_conv,
                    winit::event::VirtualKeyCode::Right => new_rot[2] -= self.rotamt / dt_conv,
                    _ => {}
                }
            }

            if (bound.is_some()) {
                let clipped = bound.as_mut().expect("no bounds").clip(new[0], new[1]);
                new[0] = clipped[0];
                new[1] = clipped[1];
            }

            // println!("deg: {}", new_rot[2]);

            // let mut angle = new_rot[2];
            // angle = angle % 360.0; // Wrap angle within 0-360
            // if angle < 0.0 {
            //     angle += 360.0; // Ensure angle is positive
            // }
            //
            // new_rot[2] = angle;

            transform.pos = new;
            transform.rot = new_rot;
        });
    }
}

impl ComponentTrait for CharacterController2D {
    fn name(&self) -> &str {
        "CharacterController2D"
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl CharacterController2D {
    pub fn new(bounds: Option<Bounds2D>) -> Arc<Mutex<ComponentWrapper>> {
        let controller = Arc::new(Mutex::new(Self {
            moveamt: 0.01,
            rotamt: 2.0 /*0.01*/,
            state: ComponentState::new(),
            bounds
        }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Input(controller.clone())));

        Arc::new(Mutex::new(ComponentWrapper {
            component: controller as Arc<Mutex<dyn ComponentTrait>>,
            ticker: tick_variant,
        }))
    }
}

pub struct Rigidbody {
    pub state: ComponentState,
    pub friction: f32,
    pub gravity: bool,
    pub collisions: bool,
}

// impl ComponentTrait for Rigidbody {
//     fn name(&self) -> &str {
//         "Rigidbody"
//     }
//
//     fn state(&mut self) -> &mut ComponentState {
//         &mut self.state
//     }
// }
//
// impl TickBehavior for Rigidbody {
//     fn tick(&mut self, obj: &mut GameObject, dt: Duration) {}
// }
//
// impl Rigidbody {
//     pub fn new(gravity: bool, collisions: bool) -> Arc<Mutex<ComponentWrapper>> {
//         let s = Arc::new(Mutex::new(Self {
//             state: ComponentState::new(),
//             friction: 0.1,
//             gravity,
//             collisions,
//         }));
//         let tick_variant = Arc::new(Mutex::new(TickVariant::Default(s.clone())));
//
//         Arc::new(Mutex::new(ComponentWrapper {
//             component: s as Arc<Mutex<dyn ComponentTrait>>,
//             ticker: tick_variant,
//         }))
//     }
// }
