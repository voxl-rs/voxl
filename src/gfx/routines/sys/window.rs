use legion::*;
use shrev::{EventChannel, ReaderId};
use winit::window::Window;

#[system]
pub fn window_system(
    #[state] _win: &mut Window,
    #[state] mut reader: &mut ReaderId<()>,
    #[resource] channel: &EventChannel<()>,
) {
    for _ in channel.read(&mut reader) {}
}
