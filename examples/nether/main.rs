use voxl::{
    app::{AppBuilder, Bundle, ResumeApp},
    chunk::Accessor,
    core::{
        ecs::{systems::Builder, *},
        events::{register_reader_from_resource, EventChannel, ReaderId},
        Input,
    },
    gfx::camera::{Camera, Projection, ProjectionExt},
    math::cg::{Deg, InnerSpace, Point3, Rad, Vector3},
    time::DeltaTime,
};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkDimensions;

impl Accessor for ChunkDimensions {
    const SIDE_LEN: usize = 8;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Blocks {
    Air,
}

impl Default for Blocks {
    fn default() -> Self {
        Self::Air
    }
}

fn main() {
    env_logger::init();
    let builder = AppBuilder::default();
    let mut app = builder.bundle::<MyChonk>().unwrap().build(8);

    app.run();
}

#[derive(Debug)]
pub struct MyChonk;
impl Bundle for MyChonk {
    fn arrange(
        world: &mut World,
        mut resources: &mut Resources,
        schedule: &mut Builder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        world.push((
            Camera::new(Deg(-90f32), Deg(-20f32)),
            Point3::<f32>::new(0., 0., 10.),
        ));

        schedule.add_system(play_system(
            DeltaTime::default(),
            register_reader_from_resource::<Input>(&mut resources),
            MovementBindings::new(
                VirtualKeyCode::Space,
                VirtualKeyCode::LShift,
                VirtualKeyCode::A,
                VirtualKeyCode::D,
                VirtualKeyCode::W,
                VirtualKeyCode::S,
                VirtualKeyCode::Up,
                VirtualKeyCode::Down,
            ),
        ));

        Ok(())
    }
}

const SPEED: f64 = 5.;
const MOUSE_SPEED: f64 = 0.2;

#[system(for_each)]
fn play(
    #[state] delta_time: &mut DeltaTime,
    #[state] mut reader_id: &mut ReaderId<Input>,
    #[state] movement_binding: &mut MovementBindings,
    #[resource] ev_channel: &mut EventChannel<Input>,
    #[resource] projection: &mut Projection,
    #[resource] resume: &mut ResumeApp,
    cam: &mut Camera,
    translation: &mut Point3<f32>,
) {
    for event in ev_channel.read(&mut reader_id) {
        match event {
            Input::Key(KeyboardInput {
                scancode: _,
                state: _,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            }) => {
                resume.end();
                log::warn!("shutting down");
            }

            Input::Key(KeyboardInput {
                scancode: _,
                state,
                virtual_keycode: Some(key),
                ..
            }) => {
                if *state == ElementState::Released {
                    movement_binding.released(*key)
                } else {
                    movement_binding.pressed(*key)
                }
            }

            Input::MouseDelta(x, y) => cam.orient(
                Rad((*x * MOUSE_SPEED * delta_time.elapsed()) as f32),
                Rad((-*y * MOUSE_SPEED * delta_time.elapsed()) as f32),
            ),

            _ => {}
        }
    }

    if let Some(dir) = movement_binding.dir() {
        *translation += dir * (delta_time.elapsed() * SPEED) as f32;
    }

    projection.zoom(Rad(movement_binding.zooming() * delta_time.elapsed() as f32));

    delta_time.flush();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyState {
    Pressed,
    Held,
    Released,
}

impl KeyState {
    pub fn active(&self) -> bool {
        match self {
            Self::Pressed | Self::Held => true,
            Self::Released => false,
        }
    }

    #[inline]
    pub fn held(&self) -> bool {
        *self == Self::Held
    }

    #[inline]
    pub fn pressed(&self) -> bool {
        *self == Self::Pressed
    }

    #[inline]
    pub fn released(&self) -> bool {
        *self == Self::Released
    }

    #[inline]
    pub fn hold(&mut self) {
        *self = Self::Held;
    }

    #[inline]
    pub fn press(&mut self) {
        *self = Self::Pressed;
    }

    #[inline]
    pub fn release(&mut self) {
        *self = Self::Released;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MovementBindings {
    up: (VirtualKeyCode, KeyState),
    down: (VirtualKeyCode, KeyState),
    left: (VirtualKeyCode, KeyState),
    right: (VirtualKeyCode, KeyState),
    front: (VirtualKeyCode, KeyState),
    back: (VirtualKeyCode, KeyState),
    zoom_in: (VirtualKeyCode, KeyState),
    zoom_out: (VirtualKeyCode, KeyState),
}

impl MovementBindings {
    pub fn new(
        up: VirtualKeyCode,
        down: VirtualKeyCode,
        left: VirtualKeyCode,
        right: VirtualKeyCode,
        front: VirtualKeyCode,
        back: VirtualKeyCode,
        zoom_in: VirtualKeyCode,
        zoom_out: VirtualKeyCode,
    ) -> Self {
        Self {
            up: (up, KeyState::Released),
            down: (down, KeyState::Released),
            left: (left, KeyState::Released),
            right: (right, KeyState::Released),
            front: (front, KeyState::Released),
            back: (back, KeyState::Released),
            zoom_in: (zoom_in, KeyState::Released),
            zoom_out: (zoom_out, KeyState::Released),
        }
    }

    pub fn zooming(&self) -> f32 {
        let mut res = 0.;
        if self.zoom_in.1.active() {
            res -= 1.;
        } else if self.zoom_out.1.active() {
            res += 1.;
        }
        res
    }

    /// Returns normalized vector
    pub fn dir(&self) -> Option<Vector3<f32>> {
        let mut direction: Vector3<f32> = Vector3::new(0., 0., 0.);

        // Right hand coordinate system
        if self.front.1.active() {
            direction -= Vector3::unit_z();
        }

        if self.back.1.active() {
            direction += Vector3::unit_z();
        }

        if self.up.1.active() {
            direction += Vector3::unit_y();
        }

        if self.down.1.active() {
            direction -= Vector3::unit_y();
        }

        if self.right.1.active() {
            direction += Vector3::unit_x();
        }

        if self.left.1.active() {
            direction -= Vector3::unit_x();
        }

        if direction.magnitude() > 0. {
            return Some(direction.normalize());
        }

        None
    }

    pub fn released(&mut self, key: VirtualKeyCode) {
        if key == self.up.0 {
            self.up.1.release();
        }
        if key == self.down.0 {
            self.down.1.release();
        }
        if key == self.left.0 {
            self.left.1.release();
        }
        if key == self.right.0 {
            self.right.1.release();
        }
        if key == self.front.0 {
            self.front.1.release();
        }
        if key == self.back.0 {
            self.back.1.release();
        }
        if key == self.zoom_in.0 {
            self.zoom_in.1.release();
        }
        if key == self.zoom_out.0 {
            self.zoom_out.1.release();
        }
    }

    pub fn pressed(&mut self, key: VirtualKeyCode) {
        const PRESSED: KeyState = KeyState::Pressed;

        if key == self.up.0 {
            self.up.1 = PRESSED;
        }
        if key == self.down.0 {
            self.down.1 = PRESSED;
        }
        if key == self.left.0 {
            self.left.1 = PRESSED;
        }
        if key == self.right.0 {
            self.right.1 = PRESSED;
        }
        if key == self.front.0 {
            self.front.1 = PRESSED;
        }
        if key == self.back.0 {
            self.back.1 = PRESSED;
        }
        if key == self.zoom_in.0 {
            self.zoom_in.1 = PRESSED;
        }
        if key == self.zoom_out.0 {
            self.zoom_out.1 = PRESSED;
        }
    }
}
