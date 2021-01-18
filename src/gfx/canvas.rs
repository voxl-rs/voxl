use crate::{
    core::{
        ecs::{
            systems::{Builder, Runnable},
            *,
        },
        events::{new_channel, EventChannel},
        input_event::*,
    },
    gfx::{PaintBrush, Resolution},
    time::TpsCounter,
};

use shrev::*;
use shrinkwraprs::*;
use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
};

use wgpu::*;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

/// Describes a Canvas.
#[derive(Debug)]
pub struct CanvasMeta {
    pub clear_color: Color,
    surface: Surface,
    sc_desc: SwapChainDescriptor,
}

impl CanvasMeta {
    pub fn surface(&self) -> &'_ Surface {
        &self.surface
    }

    pub fn sc_desc(&self) -> &'_ SwapChainDescriptor {
        &self.sc_desc
    }

    pub fn resize(&mut self, res: &Resolution) {
        let (x, y) = *res.dimensions();
        self.sc_desc.width = x;
        self.sc_desc.height = y;
    }
}

/// Represents a drawable window.
#[derive(Debug)]
pub struct Canvas<T: WindowMarker> {
    pub resolution: Resolution,
    pub meta: CanvasMeta,
    window_handle: Window,
    window_marker: PhantomData<T>,
}

/// Represents a Canvas update.
#[derive(Debug, Clone, Copy)]
pub struct CanvasUpdate {
    id: TypeId,
    update: UpdateKind,
}

/// A kind of update for a Canvas
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum UpdateKind {
    Resize(Resolution),
    Quit,
}

/// A resource of the screen's perceived
/// Frames per second.
#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct FpsCounter(pub TpsCounter);

impl<T: WindowMarker> Canvas<T> {
    /// An event listener for resizing canvas dimensions.
    pub fn update_system(id: ReaderId<CanvasUpdate>) -> impl Runnable {
        let mut reader_id = id;
        let type_id = TypeId::of::<T>();
        let sys_name = format!("Canvas{}System", type_name::<T>());

        SystemBuilder::new(sys_name)
            .write_resource::<Canvas<T>>()
            .read_resource::<EventChannel<CanvasUpdate>>()
            .build(move |_, _, (canvas, updates), _| {
                updates
                    .read(&mut reader_id)
                    .filter_map(|u| {
                        if u.id == type_id {
                            return Some(u.update);
                        }
                        None
                    })
                    .for_each(|u| match u {
                        UpdateKind::Resize(resolution) => {
                            canvas.meta.resize(&resolution);
                            canvas.resolution = resolution;
                        }
                        UpdateKind::Quit => {}
                    })
            })
    }
}

/// A compile-time unique identifier for a canvas.
pub trait WindowMarker: Debug + 'static {}

#[derive(Debug)]
/// A resource for spawning [`Canvas`](Canvas)s.
pub struct WindowEventLoop {
    eloop: EventLoop<()>,
    map: HashMap<WindowId, TypeId>,
    instance: Instance, // Creating canvas/paint_brushes
}

impl WindowEventLoop {
    /// Produces a `PaintBrush`, only need one
    /// is needed for an entire application.
    fn paint_brush(&self) -> PaintBrush {
        PaintBrush::new(&self.instance)
    }

    /// Creates a canvas to draw on.
    /// ## Panics
    /// You can only use a window marker
    /// type once ever, otherwise it will panic.
    pub fn new_canvas<T, F>(&mut self, f: F) -> Canvas<T>
    where
        T: WindowMarker,
        F: Fn(WindowBuilder) -> WindowBuilder,
    {
        let type_id = TypeId::of::<T>();

        self.map.values().find(|&&v| v == type_id).expect_none(
            "you cannot create a new canvas \
                with an already used window marker!",
        );

        let window_builder = WindowBuilder::default().with_title("voxl window");
        let window_handle = f(window_builder)
            .build(&self.eloop)
            .expect("unable to create winit window.");

        let resolution = Resolution::from(window_handle.inner_size());

        let meta = {
            let surface = unsafe { self.instance.create_surface(&window_handle) };
            let sc_desc = Self::swap_chain(&window_handle.inner_size());
            let clear_color = Color {
                r: 0.39215686274,
                g: 0.58431372549,
                b: 0.9294117647,
                a: 1.,
            }; // Cornflower blue

            CanvasMeta {
                clear_color,
                surface,
                sc_desc,
            }
        };

        self.map.insert(window_handle.id(), type_id);

        Canvas {
            resolution,
            meta,
            window_handle,
            window_marker: PhantomData::default(),
        }
    }

    /// Swapchain for windows
    fn swap_chain(window_size: &PhysicalSize<u32>) -> SwapChainDescriptor {
        SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Mailbox,
        }
    }
}

impl Default for WindowEventLoop {
    fn default() -> Self {
        Self {
            eloop: EventLoop::new(),
            map: HashMap::default(),
            instance: Instance::new(BackendBit::PRIMARY),
        }
    }
}

/// Adds all the features of window_event_system, resizing_system, and drawing_system.
#[doc(cfg(feature = "gui"))]
pub fn window_event_routine(_: &mut World, r: &mut Resources, b: &mut Builder) {
    b.add_thread_local(window_event_system(r));
}

/// Returns a `System` that manages window associated data,
/// such as:
/// * Queue `Input`s on an event channel.
/// * Update `Resolution` when a window resizes.
/// * Update an `FPSCounter` resource.
fn window_event_system(r: &mut Resources) -> impl Runnable {
    let weloop = WindowEventLoop::default();

    insert_if_none(r, weloop.paint_brush());
    insert_if_none(r, weloop);
    insert_if_none(r, FpsCounter(TpsCounter::default()));

    new_channel::<Input>(r);
    new_channel::<CanvasUpdate>(r);

    SystemBuilder::new("EventLoopSystem")
        .write_resource::<FpsCounter>()
        .write_resource::<WindowEventLoop>()
        .write_resource::<EventChannel<Input>>()
        .write_resource::<EventChannel<CanvasUpdate>>()
        .build(
            move |_, _, (fps_counter, weloop, input_channel, canvas_channel), _| {
                let WindowEventLoop {
                    eloop,
                    map,
                    instance: _,
                } = &mut **weloop;

                eloop.run_return(|event, _, control_flow| {
                    match event {
                        Event::RedrawEventsCleared => {
                            fps_counter.update();
                            fps_counter.flush();
                            return;
                        }

                        Event::WindowEvent {
                            ref event,
                            window_id,
                        } => {
                            if let Some(&id) = map.get(&window_id) {
                                match event {
                                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                                        let update = UpdateKind::Quit;
                                        canvas_channel.single_write(CanvasUpdate { id, update });
                                    }

                                    WindowEvent::KeyboardInput {
                                        input:
                                            KeyboardInput {
                                                scancode: _,
                                                state,
                                                virtual_keycode: Some(key),
                                                ..
                                            },
                                        ..
                                    } => {
                                        use ElementState as ES;
                                        use KeyState::*;

                                        let keystate = match state {
                                            ES::Pressed => Pressed(*key),
                                            ES::Released => Released(*key),
                                        };

                                        input_channel.iter_write(vec![
                                            Input::Key { id, keystate },
                                            Input::Key {
                                                id,
                                                keystate: KeyState::Any(*key),
                                            },
                                        ]);
                                    }

                                    WindowEvent::ReceivedCharacter(c) => {
                                        input_channel
                                            .single_write(Input::Char { id, character: *c });
                                    }

                                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                        let resolution = Resolution::from(**new_inner_size);
                                        let update = UpdateKind::Resize(resolution);
                                        canvas_channel.single_write(CanvasUpdate { id, update });
                                    }

                                    WindowEvent::Resized(new_size) => {
                                        let resolution = Resolution::from(*new_size);
                                        let update = UpdateKind::Resize(resolution);
                                        canvas_channel.single_write(CanvasUpdate { id, update });
                                    }

                                    _ => {}
                                }
                            }
                        }

                        Event::DeviceEvent {
                            device_id: _,
                            event,
                        } => match event {
                            DeviceEvent::MouseMotion { delta } => {
                                input_channel.single_write(Input::MouseDelta(delta.0, delta.1))
                            }

                            DeviceEvent::MouseWheel {
                                delta: MouseScrollDelta::LineDelta(x, y),
                            } => input_channel
                                .single_write(Input::MouseWheelDelta(x as f64, y as f64)),

                            _ => {}
                        },
                        _ => {}
                    }

                    *control_flow = ControlFlow::Exit;
                });
            },
        )
}
