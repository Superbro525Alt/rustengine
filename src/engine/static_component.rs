use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

pub trait StaticComponent: Send + Sync + Downcast {
    fn new() -> Self
    where
        Self: Sized;
    fn postinit(&mut self) {}
    fn tick(&mut self);
    fn clean(&mut self) {}
}

impl_downcast!(StaticComponent);

struct Physics {}

impl StaticComponent for Physics {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn postinit(&mut self) {}

    fn tick(&mut self) {}

    fn clean(&mut self) {}
}
