use crate::texture::Texture;

pub trait RenderPass {
    fn get_name(&self) -> String;
    fn get_desc(&self) -> &wgpu::RenderPassDescriptor;
}

pub struct DeferredRenderPass {
    diffuse_texture: Texture,
    normal_texture: Texture,
    specular_texture: Texture,
    depth_texture: Texture,
}

impl DeferredRenderPass {
    pub fn new(
        diffuse_texture: Texture,
        normal_texture: Texture,
        specular_texture: Texture,
        depth_texture: Texture,
    ) -> DeferredRenderPass {
        Self {
            diffuse_texture,
            normal_texture,
            specular_texture,
            depth_texture,
        }
    }
}

impl RenderPass for DeferredRenderPass {
    fn get_desc(&self) -> &wgpu::RenderPassDescriptor {
        &wgpu::RenderPassDescriptor {
            label: Some(self.get_name().as_ref()),
            color_attachments: &[
                wgpu::RenderPassColorAttachment {
                    view: &self.diffuse_texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                },
                wgpu::RenderPassColorAttachment {
                    view: &self.normal_texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                },
                wgpu::RenderPassColorAttachment {
                    view: &self.specular_texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        }
    }

    fn get_name(&self) -> String {
        "Deferred Render Pass".to_owned()
    }
}
