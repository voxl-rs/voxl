use voxl::{
    core::{
        ecs::{Resources, Schedule, World},
        events::{Event as EventShrev, EventChannel, ReaderId},
        systems::input::{build_input_system, MovementBindings},
        thread::create_pool,
    },
    graph::{
        camera::{Camera, Projection},
        gfx::{swap_chain, Render},
        gpu::BackendBit,
        systems::{
            event_loop::{event_loop_system, screen_size_system},
            render::{camera_system, render_system},
        },
        uniforms::Uniforms,
        win::{
            dpi::PhysicalSize,
            event::{KeyboardInput, VirtualKeyCode},
            event_loop::EventLoop,
            window::Window,
        },
    },
    math::cg::{Deg, Point3},
    time::DeltaTime,
};

fn main() {
    env_logger::init();

    let mut world = World::default();
    let mut resources = Resources::default();
    // Termination
    resources.insert(true);

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_cursor_visible(false);
    let _ = window.set_cursor_grab(true);

    let sc_desc = swap_chain(&window.inner_size());
    let render = Render::new(BackendBit::PRIMARY, &window);
    let render_bunch = render.bunch(&sc_desc);

    let camera = Camera::new(Deg(-90.0), Deg(-20.0));
    let projection = Projection::new(sc_desc.width, sc_desc.height, Deg(45.), 0.1, 100.);
    let position: Point3<f32> = Point3::new(0., 0., 10.);
    world.push((camera, position));
    resources.insert(projection);
    // Uniforms and SwapChainDescriptor
    resources.insert(Uniforms::default());
    resources.insert(sc_desc);

    let screen_size_reader = event_channel_init::<PhysicalSize<u32>>(&mut resources);
    let keyboard_reader = event_channel_init::<KeyboardInput>(&mut resources);
    let mouse_reader = event_channel_init::<(f64, f64)>(&mut resources);

    let mut schedule = Schedule::builder()
        //.add_thread_local(window_system(DeltaTime::default(), event_loop, window))
        .add_thread_local(event_loop_system(event_loop, window.id()))
        .add_system(screen_size_system(screen_size_reader))
        .add_system(camera_system(DeltaTime::default()))
        .add_system(render_system(DeltaTime::default(), render, render_bunch))
        .add_system(build_input_system(
            DeltaTime::default(),
            mouse_reader,
            keyboard_reader,
            MovementBindings::new(
                VirtualKeyCode::Space,
                VirtualKeyCode::LShift,
                VirtualKeyCode::Left,
                VirtualKeyCode::Right,
                VirtualKeyCode::Up,
                VirtualKeyCode::Down,
            ),
        ))
        .build();

    let pool = create_pool(8);

    while *resources
        .get::<bool>()
        .expect("please insert a `bool` resource.")
    {
        schedule.execute_in_thread_pool(&mut world, &mut resources, &pool);
    }
}

pub fn event_channel_init<T: EventShrev>(resources: &mut Resources) -> ReaderId<T> {
    let mut channel = EventChannel::<T>::with_capacity(32);
    let reader = channel.register_reader();
    resources.insert(channel);
    reader
}
