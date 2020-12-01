//use legion::*;

use crate::core::ecs::*;

use shrev::{EventChannel, ReaderId};

use wgpu::SwapChainDescriptor;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::desktop::EventLoopExtDesktop,
    window::WindowId,
};

#[system]
pub fn screen_size(
    #[state] mut reader: &mut ReaderId<PhysicalSize<u32>>,
    #[resource] channel: &EventChannel<PhysicalSize<u32>>,
    #[resource] sc_desc: &mut SwapChainDescriptor,
) {
    for new_size in channel.read(&mut reader) {
        sc_desc.width = new_size.width;
        sc_desc.height = new_size.height;
    }
}

#[system]
pub fn event_loop(
    #[state] eloop: &mut EventLoop<()>,
    #[state] win_id: &WindowId,
    #[resource] keyboard_channel: &mut EventChannel<KeyboardInput>,
    #[resource] mouse_channel: &mut EventChannel<(f64, f64)>,
    #[resource] screen_size_channel: &mut EventChannel<PhysicalSize<u32>>,
    #[resource] resume: &mut bool,
) {
    eloop.run_return(|event, _window_target, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == *win_id => match event {
                WindowEvent::CloseRequested => *resume = false,

                WindowEvent::KeyboardInput { input, .. } => {
                    keyboard_channel.single_write(*input);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    screen_size_channel.single_write(**new_inner_size);
                }

                WindowEvent::Resized(physical_size) => {
                    screen_size_channel.single_write(*physical_size);
                }

                _ => {}
            },

            Event::DeviceEvent {
                device_id: _,
                event: DeviceEvent::MouseMotion { delta },
            } => mouse_channel.single_write(delta),

            _ => {}
        }

        *control_flow = ControlFlow::Exit;
    });
}
