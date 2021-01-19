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

    use crate::gfx::canvas::CanvasTag;
    pub use winit::{
        event::{ElementState, KeyboardInput, VirtualKeyCode},
        window::WindowId,
    };

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[non_exhaustive]
    /// Represents all possible inputs.
    pub enum Input {
        /// A key
        Key { id: TypeId, keystate: KeyState },
        /// A character
        Char { id: TypeId, character: char },
        /// A cursor position on a canvas.
        CursorPosition(f64, f64),
        /// Mouse movement.
        MouseDelta(f64, f64),
        /// Mouse wheel movement
        MouseWheelDelta(f64, f64),
    }

    /// A Key input with its corresponding state.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum KeyState {
        /// Key was pressed in a frame.
        Pressed(VirtualKeyCode),
        /// Key was pressed for `more than*` a frame.
        Held(VirtualKeyCode),
        /// Key was released.
        Released(VirtualKeyCode),
        /// Key was any of the three.
        Any(VirtualKeyCode), // Should this be removed?
    }

    impl Input {
        /// Returns a `KeyState` if it's a `Key` that belongs to a specified canvas.
        pub fn key_win<C: CanvasTag>(&self) -> Option<KeyState> {
            if let Input::Key { id, keystate } = *self {
                if TypeId::of::<C>() == id {
                    return Some(keystate);
                }
            }
            None
        }

        /// Returns a KeyState if it's a `Key`.
        pub fn key(&self) -> Option<KeyState> {
            if let Input::Key { id: _, keystate } = *self {
                return Some(keystate);
            }
            None
        }
    }
}

/// Events and some utility methods.
pub mod events {
    use super::ecs::Resources;
    use std::any::type_name;

    use self::Event;
    pub use shrev::*;

    /// Inserts an event channel to `Resources`.
    pub fn new_channel<E: Event>(resources: &mut Resources) {
        resources.insert(EventChannel::<E>::with_capacity(64));
        log::trace!("event channel `{}` created", type_name::<E>());
    }

    /// Retrieves a `ReaderId<T>` from an `EventChannel` from `Resources`.
    /// ## Panics
    /// Make sure that the event channel exists!
    pub fn subscribe<E: Event>(resources: &mut Resources) -> ReaderId<E> {
        let id = resources
            .get_mut::<EventChannel<E>>()
            .expect(&format!(
                "`EventChannel<{}>` is not present in `Resources`",
                type_name::<E>()
            ))
            .register_reader();

        log::trace!("subscribed to `{}` event channel", type_name::<E>());
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
