/// All you need to get started
use crate::core::ecs::{systems::Builder, *};
use rayon::{ThreadPool, ThreadPoolBuilder};

pub const DEFAULT_THREAD_NUM: usize = 8;

pub struct AppBuilder {
    pub world: World,
    pub resources: Resources,
    pub builder: Builder,
    num_threads: usize,
}

impl AppBuilder {
    /// Application with an empty `World`, and one `ResumeApp` in `Resources`
    fn empty() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut builder = Schedule::builder();

        resources.insert(ResumeApp::default());
        log::debug!("`ResumeApp` pushed to `Resources`");

        #[cfg(feature = "gui")]
        super::gfx::window_event_routine(&mut world, &mut resources, &mut builder);

        Self {
            world,
            resources,
            builder,
            num_threads: DEFAULT_THREAD_NUM,
        }
    }

    /// Change the number of threads used, default is `8`
    pub fn num_threads(mut self, num: usize) -> Self {
        self.num_threads = num;
        self
    }

    pub fn routine<T: Routine>(mut self) -> Self {
        T::setup(&mut self.world, &mut self.resources, &mut self.builder);
        self
    }

    pub fn routine_fn<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut World, &mut Resources, &mut Builder),
    {
        f(&mut self.world, &mut self.resources, &mut self.builder);
        self
    }

    pub fn build(self) -> App {
        self.into()
    }
}

impl From<AppBuilder> for App {
    fn from(
        AppBuilder {
            world,
            resources,
            builder,
            num_threads,
        }: AppBuilder,
    ) -> Self {
        Self {
            w: world,
            r: resources,
            s: builder.into(),
            pool: ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap(),
        }
    }
}

pub struct App {
    w: World,
    r: Resources,
    s: Schedule,
    pool: ThreadPool,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::empty()
    }

    pub fn run(&mut self) {
        log::info!("start");

        while **get_expect::<ResumeApp>(&self.r) {
            self.s
                .execute_in_thread_pool(&mut self.w, &mut self.r, &self.pool);
        }

        log::info!("stop");
    }
}

pub trait Routine {
    fn setup(w: &mut World, r: &mut Resources, b: &mut Builder);
}

/// A resource that decides whether
/// the application should continue
/// iterating before coming to a halt
#[derive(Debug, shrinkwraprs::Shrinkwrap)]
pub struct ResumeApp(pub bool);

impl ResumeApp {
    pub fn end(&mut self) {
        self.0 = false;
    }
}

impl Default for ResumeApp {
    fn default() -> Self {
        Self(true)
    }
}
