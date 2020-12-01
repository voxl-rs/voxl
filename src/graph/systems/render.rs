use crate::{
    core::ecs::*,
    graph::{
        camera::{Camera, Projection},
        gfx::{Render, RenderBunch},
        gpu::*,
        texture,
        uniforms::Uniforms,
    },
    math::cg::Point3,
    time::DeltaTime,
};

use bytemuck::cast_slice;

#[system(for_each)]
pub fn camera(
    #[state] _delta_time: &mut DeltaTime,
    #[resource] uniforms: &mut Uniforms,
    #[resource] projection: &Projection,
    cam: &Camera,
    pos: &Point3<f32>,
) {
    //let origin: Vector3<f32> = (-pos.x, -pos.y, -pos.z).into();
    //let view_position = pos.to_homogeneous();
    let view_projection = projection.matrix() * cam.matrix(*pos);
    uniforms.update_view_proj(&view_projection);
    //uniforms.update_view_proj(&cam.build_view_projection(*pos, origin));
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
        instance_buff,
        num_indices,
    }: &RenderBunch,
    #[resource] sc_desc: &SwapChainDescriptor,
    #[resource] uniforms: &Uniforms,
    #[resource] projection: &mut Projection,
) {
    println!("dt: {} fps: {}", delta_time.val(), delta_time.tps());

    projection.re_size(sc_desc.width as f32 / sc_desc.height as f32);

    // Camera Projection
    queue.write_buffer(&uniform_buff, 0, cast_slice(&[*uniforms]));

    let frame = {
        device
            .create_swap_chain(&surface, &sc_desc)
            .get_current_frame()
            .expect("Timeout getting texture")
            .output
    };

    let depth_texture =
        texture::Texture::create_depth_texture(device, sc_desc, Some("Depth Texture"));

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
                        r: 139. / 255.,
                        g: 0.,
                        b: 0.,
                        //r: 0.39215686274,
                        //g: 0.58431372549,
                        //b: 0.9294117647,
                        a: 1.,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
                attachment: &depth_texture.view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &diffuse_bg, &[]);
        pass.set_bind_group(1, &uniform_bg, &[]);

        pass.set_vertex_buffer(0, vertex_buff.slice(..));

        pass.set_vertex_buffer(1, instance_buff.slice(..));
        pass.set_index_buffer(index_buff.slice(..));
        pass.draw_indexed(0..*num_indices, 0, 0..IN as _);
    }

    queue.submit(std::iter::once(encoder.finish()));
    delta_time.flush();
}

const IN: u32 = 64 * 64 * 24;
