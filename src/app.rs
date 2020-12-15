/// All you need to get started
use crate::{
    core::ecs::{systems::Builder, *},
    gfx::routines::{Cam, Graph},
};

pub struct AppBuilder {
    pub world: World,
    pub resources: Resources,
    pub schedule: Builder,
}

impl AppBuilder {
    /// An empty builder, this is not recommended as it does not
    /// contain built in graphics, windowing, and Input event channel
    pub fn empty() -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
            schedule: Builder::default(),
        }
    }

    pub fn with_routine<T: Routine>(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        T::setup(&mut self.world, &mut self.resources, &mut self.schedule)?;
        Ok(self)
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        let mut builder = Self::empty();

        builder.resources.insert(ResumeApp::default());
        log::debug!("resource loaded -> ResumeApp");

        builder
            .with_routine::<Graph>()
            .expect("unable to initiate graphics")
            .with_routine::<Cam>()
            .expect("unable to initiate camera")
    }
}

impl AppBuilder {
    pub fn build(mut self, pool: usize) -> App {
        App {
            world: self.world,
            resources: self.resources,
            schedule: self.schedule.build(),
            pool: rayon::ThreadPoolBuilder::new()
                .num_threads(pool)
                .build()
                .unwrap(),
        }
    }
}

pub struct App {
    world: World,
    resources: Resources,
    schedule: Schedule,
    pool: rayon::ThreadPool,
}

impl App {
    pub fn run(&mut self) {
        log::info!("start");

        while self
            .resources
            .get::<ResumeApp>()
            .expect("resource `ResumeApp` is missing")
            .resume()
        {
            self.schedule
                .execute_in_thread_pool(&mut self.world, &mut self.resources, &self.pool);
        }

        log::info!("halt")
    }
}

pub trait Routine {
    fn setup(
        world: &mut World,
        resources: &mut Resources,
        schedule: &mut Builder,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// A resource that decides whether
/// the application should continue
/// iterating before coming to a halt
pub struct ResumeApp(bool);

impl ResumeApp {
    fn resume(&self) -> bool {
        self.0
    }

    pub fn end(&mut self) {
        self.0 = false;
    }
}

impl Default for ResumeApp {
    fn default() -> Self {
        Self(true)
    }
}
