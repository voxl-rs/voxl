use crate::ecs::{
    events::{EventChannel, ReaderId},
    *,
};

use voxl_graph::{
    cgmath::{InnerSpace, Point3, Vector3},
    uniforms::Camera,
    winit::event::{ElementState, KeyboardInput, VirtualKeyCode},
};

use voxl_time::DeltaTime;

const SPEED: f64 = 5.;

#[system(for_each)]
pub fn input(
    #[state] delta_time: &mut DeltaTime,
    #[state] mut reader_id: &mut ReaderId<KeyboardInput>,
    #[state] movement_binding: &mut MovementBindings,
    #[resource] keyboard_channel: &EventChannel<KeyboardInput>,
    _: &Camera,
    translation: &mut Point3<f32>,
) {
    for event in keyboard_channel.read(&mut reader_id) {
        match event {
            &KeyboardInput {
                scancode: _,
                state: ElementState::Released,
                virtual_keycode: Some(key),
                ..
            } => movement_binding.released(key),

            &KeyboardInput {
                scancode: _,
                state: ElementState::Pressed,
                virtual_keycode: Some(key),
                ..
            } => movement_binding.pressed(key),

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

#[test]
fn movement_binding_test_01() {
    let m = MovementBindings::default();
    assert_eq!(m.front.1, KeyState::Released);
}

#[test]
fn movement_binding_test_02() {
    let kstate = KeyState::Pressed;
    let k2state = KeyState::Held;
    let k3state = KeyState::Released;
    assert!(kstate.active() == k2state.active() != k3state.active());
}

#[system]
fn movement() {}
