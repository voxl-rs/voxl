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
//!
//! /*
//! #[derive(GameState)]
//! pub struct StartUpState {
//!     data: u128,
//! }
//!
//! fn main() {
//!     let mut vox = Vox<StartUpState, u32, 512> {
//!         name: "Minecraft Ripoff",
//!         strategy: ChunkLoadingStrategy::Cube { side: 8 },
//!     };
//!
//!     vox.run();
//! }
//! */
//! ```

#![feature(min_const_generics)]
#![doc(html_logo_url = "../../../assets/hyper_cube_sand2.png")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![warn(clippy::all)]
#![allow(clippy::new_without_default)]

/// Contains other essential features that your game will need
pub mod core;
/// Handles your gui, rendering, etc; To draw something on your screen
pub mod graph;

/// Inevitable math made accesible here
pub mod math {
    pub use cgmath as cg;
    pub use nalgebra as ng;
    pub use noise as no;
}

/// Timing
pub mod time;
/// Data Types for storing and processing chunked data
pub mod vox;
