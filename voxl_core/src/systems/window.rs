use crate::ecs::{
    events::{EventChannel, ReaderId},
    system,
};

use voxl_graph::{
    wgpu::SwapChainDescriptor,
    winit::{
        dpi::PhysicalSize,
        event::{DeviceEvent, Event, KeyboardInput, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::desktop::EventLoopExtDesktop,
        window::Window,
    },
};

use voxl_time::DeltaTime;

#[system]
pub fn screen_size(
    #[resource] channel: &EventChannel<PhysicalSize<u32>>,
    #[state] mut reader: &mut ReaderId<PhysicalSize<u32>>,
    #[resource] sc_desc: &mut SwapChainDescriptor,
) {
    for new_size in channel.read(&mut reader) {
        sc_desc.width = new_size.width;
        sc_desc.height = new_size.height;
    }
}

#[system]
pub fn auto_fov(
    #[resource] channel: &EventChannel<PhysicalSize<u32>>,
    #[state] mut reader: &mut ReaderId<PhysicalSize<u32>>,
) {
    for new_size in channel.read(&mut reader) {
        let aspect = new_size.width as f32 / new_size.height as f32;
        println!("aspect is {}", aspect);
    }
}

#[system]
pub fn window(
    #[state] delta_time: &mut DeltaTime,
    #[state] eloop: &mut EventLoop<()>,
    #[state] window: &mut Window,
    #[resource] keyboard_channel: &mut EventChannel<KeyboardInput>,
    #[resource] screen_size_channel: &mut EventChannel<PhysicalSize<u32>>,
) {
    eloop.run_return(|event, _window_target, control_flow| {
        match event {
            //Event::RedrawRequested(_) => {} // gfx.update(); //gfx.render();
            //Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // UPDATED!
                match event {
                    WindowEvent::CloseRequested => {}

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
                }
            }

            Event::DeviceEvent {
                device_id: _,
                event: DeviceEvent::MouseMotion { delta: _ },
            } => {}

            _ => {}
        }

        *control_flow = ControlFlow::Exit;
    });

    delta_time.flush();
}
