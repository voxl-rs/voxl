/// Systems that do not strictly couple with other top-level modules
pub mod systems;

/// Utils for threading
pub mod thread;

/// 3D Transformations
pub mod transform;

/// Module containing Legion and `system` macro
pub mod ecs {
    pub use legion::system;
    pub use legion::*;
}

pub use shrev as events;
