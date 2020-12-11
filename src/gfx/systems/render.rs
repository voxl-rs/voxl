use crate::{
    core::ecs::*,
    graph::{
        camera::{Camera, Projection},
        gfx::{Render, RenderBunch},
        gpu::*,
        texture,
        uniforms::Uniforms,
        DrawFrame,
    },
    math::cg::{Matrix4, Point3},
};

use bytemuck::cast_slice;

#[system(for_each)]
#[filter(maybe_changed::<Camera>())]
#[filter(maybe_changed::<Point3<f32>>())]
pub fn camera(
    #[state] view_projection: &mut ResourceIfChanged<Matrix4<f32>>,
    #[resource] uniforms: &mut ResourceIfChanged<Uniforms>,
    #[resource] projection: &mut Projection,
    #[resource] sc_desc: &SwapChainDescriptor,
    view: &Camera,
    pos: &Point3<f32>,
) {
    projection.re_size(sc_desc.width as f32 / sc_desc.height as f32);

    let new_projection = projection.perspective() * view.matrix(*pos);

    if view_projection.data != new_projection {
        view_projection.update(|v| *v = new_projection);
        uniforms.update(|u| u.update_view_proj(&view_projection));
    }
}

#[system]
pub fn render(
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
    #[resource] uniforms: &mut ResourceIfChanged<Uniforms>,
    #[resource] draw_frame: &mut DrawFrame,
) {
    if **draw_frame {
        //println!("{:?}", procinfo::pid::statm_self().unwrap());

        // Write on buffers only when you really need to.
        uniforms.do_if_changed(|u| queue.write_buffer(&uniform_buff, 0, cast_slice(&[*u])));

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
                            r: 139. / 255., //r: 0.39215686274, g: 0.58431372549, b: 0.9294117647,
                            g: 0.,
                            b: 0.,
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

            pass.draw_indexed(0..*num_indices, 0, 0..3);
        }

        queue.submit(std::iter::once(encoder.finish()));
        **draw_frame = false;
    }
}
