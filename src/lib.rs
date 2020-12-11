//! [Voxl][gh] is a free and open source 3D game engine written in [Rust][rs]
//! for your build-your-worlds making use of voxels, tensors, and octrees
//! to easily compose and optimize your game. It makes use of [Legion][lg]'s
//! Data Oriented pattern to properly separate concerns and maximize
//! extensibility.
//!
//! Feel free to browse the [book][bk] for a proper introduction to the engine.
//!
//! [rs]: https://www.rust-lang.org/
//! [gh]: https://github.com/voxl-rs/voxl
//! [lg]: https://github.com/amethyst/legion
//! [bk]: https://book.voxl.rs/master/
//! ```rust
//! use voxl::{
//!     vox::{new_world, Accessor},
//!     math::ng::Point3,
//! };
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! struct MyAccessor;
//! impl Accessor for MyAccessor {
//!     const SIDE_LEN: usize = 8;
//! }
//!
//! enum Blocks {
//!     Air,
//!     Solid,
//! }
//!
//! fn main() {
//! }
//! ```

#![feature(min_const_generics)]
#![doc(html_logo_url = "../../../assets/hyper_cube_sand2.png")]
/*
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![warn(clippy::all)]
#![allow(clippy::new_without_default)]
*/

/// Contains other essential features that your game will need
pub mod core;

/// To get your application loop and window up
pub mod app;

/// Handles your gui, rendering, etc; To draw something on your screen
pub mod graph;

/// Data Types for storing and processing chunked data
pub mod chunk;

/// Timing
pub mod time;

/// Inevitable math made accesible here
pub mod math {
    pub use cgmath as cg;
    pub use nalgebra as ng;
    pub use noise as no;
}
