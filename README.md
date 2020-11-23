# voxl-rs engine
A 3D game engine specialized for voxel-type worlds.


## Data-driven
voxl-rs uses an Entity-Component-System architecture from legion, working along with rust's safety and concurrency features for preventing invalidating states.

## Fast Render
The render engine is built on top of wgpu-rs ensuring optimal drawing
capabilities, theoretically more performant than opengl.
