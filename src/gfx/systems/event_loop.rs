use crate::{
    app::ResumeApp,
    core::{ecs::*, events::EventChannel, Input},
    graph::{
        gpu::SwapChainDescriptor,
        win::{
            event::{DeviceEvent, Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            platform::desktop::EventLoopExtDesktop,
            window::WindowId,
        },
        DisplayFPS, DrawFrame,
    },
    time::FpsCounter,
};
use winit::dpi::PhysicalSize;

#[system]
pub fn event_loop(
    #[state] eloop: &mut EventLoop<()>,
    #[state] win_id: &WindowId,
    #[state] counter: &mut FpsCounter,
    #[resource] display_fps: &mut DisplayFPS,
    #[resource] input_channel: &mut EventChannel<Input>,
    #[resource] mut sc_desc: &mut SwapChainDescriptor,
    #[resource] resume: &mut ResumeApp,
    #[resource] draw_frame: &mut DrawFrame,
) {
    eloop.run_return(|event, _window_target, control_flow| {
        match event {
            Event::RedrawEventsCleared => {
                counter.update();
                **display_fps = counter.tps();
                counter.flush();
                **draw_frame = true;
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == *win_id => match event {
                WindowEvent::CloseRequested => resume.end(),

                WindowEvent::KeyboardInput { input, .. } => {
                    input_channel.single_write(Input::Key(*input));
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize_sc_desc(&mut sc_desc, **new_inner_size);
                }

                WindowEvent::Resized(new_size) => {
                    resize_sc_desc(&mut sc_desc, *new_size);
                }

                _ => {}
            },

            Event::DeviceEvent {
                device_id: _,
                event: DeviceEvent::MouseMotion { delta },
            } => input_channel.single_write(Input::MouseDelta(delta.0, delta.1)),

            _ => {}
        }

        *control_flow = ControlFlow::Exit;
    });
}

#[inline(always)]
fn resize_sc_desc(sc_desc: &mut SwapChainDescriptor, size: PhysicalSize<u32>) {
    sc_desc.width = size.width;
    sc_desc.height = size.height;
}
