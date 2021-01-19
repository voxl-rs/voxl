use voxl::{
    app::*,
    core::{
        ecs::{systems::Builder, *},
        events::{subscribe, EventChannel, ReaderId},
        input_event::*,
    },
};

fn setup_quit(_: &mut World, r: &mut Resources, b: &mut Builder) {
    let mut reader_id: ReaderId<Input> = subscribe(r);

    b.add_system(
        SystemBuilder::new("ExitSystem")
            .read_resource::<EventChannel<Input>>()
            .write_resource::<ResumeApp>()
            .build(move |_, _, (ev_channel, app), _with_win| {
                use KeyState::*;
                use VirtualKeyCode::*;

                ev_channel
                    .read(&mut reader_id)
                    .filter_map(|i| i.key())
                    .for_each(|k| match k {
                        Pressed(Escape) => app.end(),
                        _ => {}
                    });
            }),
    );
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut app: App = App::builder().routine_fn(setup_quit).into();

    app.run();
}
