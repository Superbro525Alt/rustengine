use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use serde_json::Value;
use std::any::{self, Any, TypeId};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
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

impl_downcast!(ComponentTrait);

pub trait ComponentTrait: Send + Sync + Downcast
where
    Self: 'static,
{
    fn name(&self) -> &str;
    fn state(&mut self) -> &mut ComponentState;
    // fn tick_type(&mut self) -> &mut TickVariant;
}

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Input,
    Render,
    Default,
}

pub struct InputData;
pub struct RenderOutput;

pub trait TickBehavior: Send + Sync {
    fn tick(&mut self);
}

pub trait InputTickBehavior: Send + Sync {
    fn tick_with_input(&mut self, input: &InputData);
}

pub trait RenderTickBehavior: Send + Sync {
    fn render_tick(&mut self) -> RenderOutput;  
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

     pub fn tick(&mut self, input: Option<&InputData>) -> Option<RenderOutput> {
        match self {
            TickVariant::Input(behavior) => {
                if let Some(input) = input {
                    behavior.lock().unwrap().tick_with_input(input);
                }
                None
            },
            TickVariant::Render(behavior) => {
                Some(behavior.lock().unwrap().render_tick())
            },
            TickVariant::Default(behavior) => {
                behavior.lock().unwrap().tick();
                None
            },
        }
    }
}


pub struct ComponentWrapper {
    pub component: Arc<Mutex<dyn ComponentTrait>>,
    pub ticker: Arc<Mutex<TickVariant>>,
}

impl ComponentWrapper {
    pub fn new(component: Arc<Mutex<dyn ComponentTrait>>, ticker: Arc<Mutex<TickVariant>>) -> Self {
        Self { component, ticker }
    }

    pub fn tick(&self, input: Option<&InputData>) -> Option<RenderOutput> {
        let mut ticker = self.ticker.lock().unwrap();
        ticker.tick(input)
    }
}

pub fn create_component_wrapper(
    component: Arc<Mutex<dyn ComponentTrait>>,
    tick_variant: Arc<Mutex<TickVariant>>
) -> Arc<Mutex<ComponentWrapper>> {
    Arc::new(Mutex::new(ComponentWrapper::new(component, tick_variant)))
}

pub struct LambdaComponent<F>
where
    F: FnMut(),
{
    name: String,
    state: ComponentState,
    tick_behavior: F,
}

impl<F> LambdaComponent<F>
where
    F: FnMut() + Send + Sync + 'static,
{
    pub fn new(name: String, tick_behavior: F) -> Arc<Mutex<ComponentWrapper>> {
        let lambda_component = Arc::new(Mutex::new(Self { name, tick_behavior, state: ComponentState::new() }));
        let tick_variant = Arc::new(Mutex::new(TickVariant::Default(lambda_component.clone())));

        Arc::new(Mutex::new(ComponentWrapper {
            component: lambda_component as Arc<Mutex<dyn ComponentTrait>>,
            ticker: tick_variant,
        }))
    }
}

impl<F> ComponentTrait for LambdaComponent<F>
where
    F: FnMut() + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn state(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl<F> TickBehavior for LambdaComponent<F>
where
    F: FnMut() + Send + Sync + 'static,
{
    fn tick(&mut self) {
        (self.tick_behavior)();
    }
}
