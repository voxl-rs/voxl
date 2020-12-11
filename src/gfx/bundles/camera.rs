use crate::{
    app::Bundle,
    core::ecs::{systems::Builder, ResourceIfChanged, *},
    graph::{
        camera::Projection, gpu::SwapChainDescriptor, systems::render::camera_system,
        uniforms::Uniforms,
    },
    math::cg::{Deg, Matrix4, SquareMatrix},
};

#[derive(Debug)]
pub struct Cam;
impl Bundle for Cam {
    fn arrange(
        _: &mut World,
        resources: &mut Resources,
        schedule: &mut Builder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sc_desc = {
            resources.get::<SwapChainDescriptor>().expect(
            "SwapChainDescriptor does not exist in Resources, please insert SwapChainDescriptor.",
        ).clone()
        };

        resources.insert(Projection::new(
            sc_desc.width,
            sc_desc.height,
            Deg(45f32),
            0.1,
            100.,
        ));
        log::debug!("resource loaded -> Projection");

        resources.insert(ResourceIfChanged::new(Uniforms::default()));
        log::debug!("resource loaded -> ResourceIfChanged<Uniforms>");

        schedule.add_system(camera_system(ResourceIfChanged::new(
            Matrix4::<f32>::identity(),
        )));

        Ok(())
    }
}
