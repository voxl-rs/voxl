/// Module containing Legion and `system` macro
pub mod ecs {
    pub use legion::system;
    pub use legion::*;

    use shrinkwraprs::*;

    #[derive(Debug, Shrinkwrap)]
    #[shrinkwrap(mutable)]
    pub struct ResourceIfChanged<T: legion::systems::Resource> {
        #[shrinkwrap(main_field)]
        pub data: T,
        changed: bool,
    }

    impl<T: legion::systems::Resource> ResourceIfChanged<T> {
        pub fn new(data: T) -> Self {
            Self {
                data,
                changed: true,
            }
        }

        pub fn do_if_changed<F: Fn(&mut T)>(&mut self, f: F) {
            if self.changed {
                f(&mut *self);
                self.changed = false;
            }
        }

        pub fn update<F: Fn(&mut T)>(&mut self, f: F) {
            f(&mut *self);
            self.changed = true;
        }

        pub fn read(&self) -> (bool, &T) {
            (self.changed, &self.data)
        }
    }
}

use crate::graph::win::event::KeyboardInput;
#[derive(Debug)]
#[non_exhaustive]
pub enum Input {
    Key(KeyboardInput),
    MouseDelta(f64, f64),
    Emulated,
}

pub mod events {
    use self::Event;
    use super::ecs::Resources;

    pub use shrev::*;

    /// Inserts an event channel for a type
    #[inline(always)]
    pub fn insert_event_channel<T: Event>(resources: &mut Resources) {
        resources.insert(EventChannel::<T>::with_capacity(32));
    }

    /// Retrieves a reader id from an EventChannel already present in Resources
    #[inline(always)]
    pub fn register_reader_from_resource<T: Event>(resources: &mut Resources) -> ReaderId<T> {
        resources
            .get_mut::<EventChannel<T>>()
            .expect("EventChannel<T> does not exist")
            .register_reader()
    }
}
