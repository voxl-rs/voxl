use legion::{systems::Runnable, *};
use shrev::{EventChannel, ReaderId};

use cgmath::{Deg, InnerSpace, Point3, Vector3};

use crate::graph::camera::Camera;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use crate::time::DeltaTime;

const SPEED: f64 = 5.;
const MOUSE_SPEED: f64 = 3.33;

/// Contains axes
#[derive(Debug, Clone, Copy)]
pub struct Control {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
    front: i32,
    back: i32,
}

/// Builds the Input System
pub fn build_input_system(
    delta_time: DeltaTime,
    m_reader_id: ReaderId<(f64, f64)>,
    reader_id: ReaderId<KeyboardInput>,
    movement_binding: MovementBindings,
) -> impl Runnable {
    input_system(delta_time, m_reader_id, reader_id, movement_binding)
}

#[system(for_each)]
fn input(
    #[state] delta_time: &mut DeltaTime,
    #[state] mut m_reader_id: &mut ReaderId<(f64, f64)>,
    #[state] mut k_reader_id: &mut ReaderId<KeyboardInput>,
    #[state] movement_binding: &mut MovementBindings,
    #[resource] keyboard_channel: &EventChannel<KeyboardInput>,
    #[resource] mouse_channel: &EventChannel<(f64, f64)>,
    cam: &mut Camera,
    translation: &mut Point3<f32>,
) {
    for event in keyboard_channel.read(&mut k_reader_id) {
        match event {
            KeyboardInput {
                scancode: _,
                state: ElementState::Released,
                virtual_keycode: Some(key),
                ..
            } => movement_binding.released(*key),

            KeyboardInput {
                scancode: _,
                state: ElementState::Pressed,
                virtual_keycode: Some(key),
                ..
            } => movement_binding.pressed(*key),

            _ => {}
        }
    }

    let mut movement = movement_binding.dir();

    if movement.magnitude() > 0. {
        movement *= (delta_time.val() * SPEED) as f32;
        *translation += movement;
        // print!("Movement vector: {:?} ", movement);
        // println!("Position: {:?}", *translation);
    }

    for event in mouse_channel.read(&mut m_reader_id) {
        let (x, y) = (
            (event.0 * MOUSE_SPEED * delta_time.val()) as f32,
            (-event.1 * MOUSE_SPEED * delta_time.val()) as f32,
        );

        cam.orient(Deg(x), Deg(y));
    }

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
}

impl Default for MovementBindings {
    fn default() -> Self {
        Self {
            up: (VirtualKeyCode::LShift, KeyState::Released),
            down: (VirtualKeyCode::LControl, KeyState::Released),
            left: (VirtualKeyCode::Left, KeyState::Released),
            right: (VirtualKeyCode::Right, KeyState::Released),
            front: (VirtualKeyCode::Up, KeyState::Released),
            back: (VirtualKeyCode::Down, KeyState::Released),
        }
    }
}

impl MovementBindings {
    pub fn new(
        up: VirtualKeyCode,
        down: VirtualKeyCode,
        left: VirtualKeyCode,
        right: VirtualKeyCode,
        front: VirtualKeyCode,
        back: VirtualKeyCode,
    ) -> Self {
        Self {
            up: (up, KeyState::Released),
            down: (down, KeyState::Released),
            left: (left, KeyState::Released),
            right: (right, KeyState::Released),
            front: (front, KeyState::Released),
            back: (back, KeyState::Released),
        }
    }

    /// Returns normalized vector
    pub fn dir(&self) -> Vector3<f32> {
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
            return direction.normalize();
        } else {
            direction
        }
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
    }
}
