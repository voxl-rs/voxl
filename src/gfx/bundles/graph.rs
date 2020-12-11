use crate::{
    app::Bundle,
    core::{
        ecs::{systems::Builder, *},
        events::insert_event_channel,
        Input,
    },
    graph::{
        gfx::{swap_chain, Render},
        gpu::BackendBit,
        systems::{event_loop::event_loop_system, render::render_system},
        win::{event_loop::EventLoop, window::Window},
        DisplayFPS, DrawFrame,
    },
    time::FpsCounter,
};

#[derive(Debug)]
pub struct Graph;
impl Bundle for Graph {
    fn arrange(
        _: &mut World,
        mut resources: &mut Resources,
        schedule: &mut Builder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).expect("unable to create winit window.");

        insert_event_channel::<Input>(&mut resources);
        log::debug!("resource loaded -> EventChannel<Input>");

        schedule.add_thread_local(event_loop_system(
            event_loop,
            window.id(),
            FpsCounter::default(),
        ));

        let sc_desc = swap_chain(&window.inner_size());

        resources.insert(sc_desc.clone());
        log::debug!("resource loaded -> SwapChainDescriptor");

        resources.insert(DisplayFPS::default());

        let render = Render::new(BackendBit::VULKAN, &window);
        let render_bunch = render.bunch(&sc_desc);

        resources.insert(window);
        log::debug!("resource loaded -> Window");

        resources.insert(DrawFrame(false));
        log::debug!("resource loaded -> DrawFrame");

        schedule.add_system(render_system(render, render_bunch));

        Ok(())
    }
}
