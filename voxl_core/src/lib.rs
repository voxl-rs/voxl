pub mod systems;
pub mod ecs {
    pub use legion::system;
    pub use legion::*;
    pub use shrev as events;
}

pub use serde::{Deserialize, Serialize};

pub mod threading {
    pub fn create_pool(num_threads: usize) -> rayon::ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Control {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
    front: i32,
    back: i32,
}

pub fn load() {}
