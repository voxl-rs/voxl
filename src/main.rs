use voxl::{
    app::*,
    core::{
        ecs::{systems::Builder, *},
        events::{subscribe, EventChannel},
        input_event::*,
    },
    gfx::{Canvas, CanvasUpdate, FpsCounter, WindowEventLoop, WindowMarker},
    time::TpsCounter,
};

#[derive(Debug)]
pub struct MainWindow;
impl WindowMarker for MainWindow {}

fn setup_window(_: &mut World, r: &mut Resources, b: &mut Builder) {
    let canvas = {
        r.get_mut::<WindowEventLoop>()
            .expect("weloop is missing")
            .new_canvas::<MainWindow, _>(|w| w.with_title("Game"))
    };

    let id = subscribe::<CanvasUpdate>(r);

    r.insert(canvas);
    b.add_system(Canvas::<MainWindow>::update_system(id));
}

fn setup_quit(_: &mut World, r: &mut Resources, b: &mut Builder) {
    let mut reader_id = subscribe::<Input>(r);

    b.add_system(
        SystemBuilder::new("ExitSystem")
            .read_resource::<EventChannel<Input>>()
            .write_resource::<ResumeApp>()
            .build(move |_, _, (ev_channel, resume), _with_win| {
                use KeyState::*;
                use VirtualKeyCode::*;

                for input in ev_channel.read(&mut reader_id) {
                    if let Some(key) = input.key_win::<MainWindow>() {
                        match key {
                            Pressed(Escape) | Pressed(Space) => resume.end(),
                            _ => {}
                        }
                    }
                }
            }),
    );
}

fn setup_screen_game_tps(_: &mut World, r: &mut Resources, b: &mut Builder) {
    b.add_system(
        SystemBuilder::new("ScreenFPSSystem")
            .read_resource::<FpsCounter>()
            .build(|_, _, counter, _| {
                log::info!("screen_fps: {}", counter.tps());
            }),
    );

    r.insert(TpsCounter::default());

    b.add_system(
        SystemBuilder::new("GameTpsSystem")
            .write_resource::<TpsCounter>()
            .build(|_, _, counter, _| {
                counter.update();
                counter.flush();
                log::info!("game_tps: {}", counter.tps());
            }),
    );
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut app: App = App::builder()
        .routine_fn(setup_window)
        .routine_fn(setup_screen_game_tps)
        .routine_fn(setup_quit)
        .into();

    app.run();
}
