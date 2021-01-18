// #![feature(min_const_generics)]
#![feature(doc_cfg)]
#![feature(option_expect_none)]
#![feature(or_patterns)]

pub mod app;
// pub mod chunk;
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
