#![feature(doc_cfg)]
#![feature(option_expect_none)]

pub mod app;
pub mod core;
pub mod gfx;
pub mod time;

pub mod math {
    pub mod ng {
        pub use nalgebra::*;
    }
    pub mod cg {
        pub use cgmath::*;
    }
    pub mod no {
        pub use noice::*;
    }
}
