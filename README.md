<div align=center>
    <h1>Voxl 🧊</h1>
    <a href="https://github.com/voxl-rs/voxl/actions"><img height=20 alt="Workflow Status" src="https://img.shields.io/github/workflow/status/voxl-rs/voxl/Rust?style=for-the-badge"></a>
    <br><br>
    A 3D game engine specialized for voxel-type worlds.
    </div>
    <h2>Data driven</h2>
    <p>Voxl uses an Entity-Component-System architecture from legion, working along with rust's safety and concurrency features for preventing invalidating states.</p>
  <h2>Fast Render</h2>
  <p>The render engine is built on top of wgpu-rs ensuring optimal drawing capabilities, theoretically more performant than opengl.</p>
  
### Examples

To compile any of the examples run:

```
$ cargo run --example name_of_example
```
For example:
```
$ cargo run --example nether
```
