/// Module containing Legion and other utilities
pub mod ecs {
    /// Retrieves an immutable borrow
    /// of a `Resource` from `Resources`.
    /// ## Panics
    /// * If a resource does not exist.
    /// * When a system already mutably borrowed it.
    /// ### Precaution
    /// * Only use during setup of routines.
    /// * Do Not use while systems are running!
    pub fn get_expect<'r, R: Resource>(r: &'r Resources) -> Fetch<'r, R> {
        r.get::<R>().expect(&format!(
            "`{}` does not exist in `Resources`",
            std::any::type_name::<R>()
        ))
    }

    /// Inserts a `Resource` if it doesn't exist already in `Resources`
    pub fn insert_if_none<R: Resource>(r: &mut Resources, insert: R) {
        if !r.contains::<R>() {
            r.insert(insert);
        }
    }

    pub use legion::*;

    use systems::{Fetch, Resource};

    use shrinkwraprs::*;

    #[derive(Debug, Shrinkwrap)]
    #[shrinkwrap(mutable)]
    pub struct ResourceIfChanged<T: Resource> {
        #[shrinkwrap(main_field)]
        pub data: T,
        changed: bool,
    }

    impl<T: Resource> ResourceIfChanged<T> {
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

#[cfg(feature = "gui")]
pub mod input_event {
    use std::any::TypeId;

    use crate::gfx::WindowMarker;
    pub use winit::{
        event::{ElementState, KeyboardInput, VirtualKeyCode},
        window::WindowId,
    };

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[non_exhaustive]
    /// Represents all possible inputs.
    pub enum Input {
        Key { id: TypeId, keystate: KeyState },
        Char { id: TypeId, character: char },
        CursorPosition(f64, f64),
        MouseDelta(f64, f64),
        MouseWheelDelta(f64, f64),
    }

    /// A Key input with its corresponding state.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum KeyState {
        /// Key was pressed in a frame
        Pressed(VirtualKeyCode),
        /// Key was pressed for `more than*` a frame
        Held(VirtualKeyCode),
        /// Key was released
        Released(VirtualKeyCode),
        /// Key was any of the three
        Any(VirtualKeyCode),
    }

    impl Input {
        /// Retrive a key input from a particular window.
        pub fn key_win<W: WindowMarker>(&self) -> Option<KeyState> {
            if let Input::Key { id, keystate } = *self {
                if TypeId::of::<W>() == id {
                    return Some(keystate);
                }
            }
            None
        }

        /// Retrive a keystate.
        pub fn key(&self) -> Option<KeyState> {
            if let Input::Key { id: _, keystate } = *self {
                return Some(keystate);
            }
            None
        }
    }
}

pub mod events {
    use self::Event;
    use super::ecs::Resources;
    use std::any::type_name;

    pub use shrev::*;

    /// Inserts an event channel to `Resources`.
    pub fn new_channel<T: Event>(resources: &mut Resources) {
        resources.insert(EventChannel::<T>::with_capacity(32));
        log::debug!("event channel `{}` instanced", type_name::<T>());
    }

    /// Registers a reader from an EventChannel already present in `Resources`.
    pub fn subscribe<T: Event>(resources: &mut Resources) -> ReaderId<T> {
        let id = resources
            .get_mut::<EventChannel<T>>()
            .expect(&format!(
                "`EventChannel<{}>` is not present in `Resources`",
                type_name::<T>()
            ))
            .register_reader();

        log::debug!("subscribed to `{}` event channel", type_name::<T>());
        id
    }
}

#[cfg(feature = "extent")]
/// This is fun ngl
pub mod extensive {
    use super::ecs::Schedule;
    /// A State dictates which systems are run at some point in time;
    /// most of the time, you don't want all of your systems to run, a state
    /// allows you to to mutate your game's behavior
    ///
    /// If you want T to store some data,
    /// then you must push an instance to `Resources`
    /// and retrieve it from there
    pub trait State {}

    /// Manipulate the world and resources using a schedule
    /// to initiate towards another state, it's recommended to
    /// also perform cleanups here
    pub trait TransitionTo<T: State>: State {
        /// Returns a schedule that will only be executed once
        fn transition() -> Schedule;
    }

    use std::marker::PhantomData;

    use shrinkwraprs::*;
    #[derive(Shrinkwrap)]
    #[shrinkwrap(mutable)]
    pub struct CurrentState<T: State> {
        #[shrinkwrap(main_field)]
        pub sched: Schedule,
        state: PhantomData<T>,
    }
}
