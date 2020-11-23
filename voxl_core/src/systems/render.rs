use crate::ecs::*;
use voxl_graph::{
    bytemuck,
    cgmath::{Point3, Vector3},
    gfx::Render,
    uniforms::{Camera, Uniforms},
    wgpu::*,
};
use voxl_time::DeltaTime;

#[system(for_each)]
pub fn camera(
    #[state] _delta_time: &mut DeltaTime,
    #[resource] uniforms: &mut Uniforms,
    cam: &Camera,
    pos: &Point3<f32>,
) {
    let origin: Vector3<f32> = (-pos.x, -pos.y, -pos.z).into();
    uniforms.update_view_proj(&cam.build_view_projection(*pos, origin));
}

#[system]
pub fn render(
    #[state] delta_time: &mut DeltaTime,
    #[state] Render {
        surface,
        device,
        queue,
    }: &mut Render,
    #[state]
    RenderBunch {
        pipeline,
        diffuse_bg,
        uniform_bg,
        uniform_buff,
        vertex_buff,
        index_buff,
        num_indices,
    }: &RenderBunch,
    #[resource] sc_desc: &SwapChainDescriptor, //
    #[resource] uniforms: &Uniforms,
) {
    println!("dt: {} fps: {}", delta_time.val(), delta_time.tps());
    // Camera Projection
    queue.write_buffer(&uniform_buff, 0, bytemuck::cast_slice(&[*uniforms]));

    let frame = {
        device
            .create_swap_chain(&surface, &sc_desc)
            .get_current_frame()
            .expect("Timeout getting texture")
            .output
    };

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.39215686274,
                        g: 0.58431372549,
                        b: 0.9294117647,
                        a: 1.,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &diffuse_bg, &[]);
        pass.set_bind_group(1, &uniform_bg, &[]);

        pass.set_vertex_buffer(0, vertex_buff.slice(..));
        pass.set_index_buffer(index_buff.slice(..));
        pass.draw_indexed(0..*num_indices, 0, 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));
    delta_time.flush();
}
