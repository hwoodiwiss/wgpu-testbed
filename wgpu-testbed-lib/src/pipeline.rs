pub trait Bindable {
    fn layout_entries() -> Vec<wgpu::BindGroupLayoutEntry>;
    fn bind_group_entries(&self) -> Vec<wgpu::BindGroupEntry>;
}

pub struct Binder<T: Bindable> {
    pub(crate) layout: wgpu::BindGroupLayout,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Bindable> Binder<T> {
    pub fn new(device: &wgpu::Device, label: Option<&str>) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &T::layout_entries(),
        });
        Self {
            layout,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn create_bind_group(
        &self,
        data: &T,
        device: &wgpu::Device,
        label: Option<&str>,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &self.layout,
            entries: &data.bind_group_entries(),
        })
    }
}

#[derive(Default)]
pub struct RenderPipelineBuilder<'a> {
    label: String,
    layout: Option<Box<wgpu::PipelineLayout>>,
    vertex_entry_point: Option<String>,
    vertex_shader_module: Option<Box<wgpu::ShaderModule>>,
    vertex_buffer_layouts: Option<&'a [wgpu::VertexBufferLayout<'a>]>,
    fragment_entry_point: Option<String>,
    fragment_shader_module: Option<Box<wgpu::ShaderModule>>,
    fragment_shader_targets: Option<Vec<wgpu::ColorTargetState>>,
    depth_stencil_format: Option<wgpu::TextureFormat>,
    depth_write: Option<bool>,
    depth_compare: Option<wgpu::CompareFunction>,
    stencil_state: Option<wgpu::StencilState>,
}

impl RenderPipelineBuilder<'_> {
    pub fn create(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            ..Default::default()
        }
    }

    pub fn vertex_shader<'a>(
        &'a mut self,
        entry_point: &str,
        shader_module: wgpu::ShaderModule,
        buffer_layouts: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> &'a mut Self {
        self.vertex_entry_point = Some(String::from(entry_point));
        self.vertex_shader_module = Some(Box::new(shader_module));
        self.vertex_buffer_layouts = Some(buffer_layouts);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let depth_format = self.depth_stencil_format;
        let depth_write = self.depth_write;
        let depth_compare = self.depth_compare;
        let stencil_state = self.stencil_state;
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(self.label.as_str()),
            layout: Some(
                self.layout
                    .as_ref()
                    .expect("Render pipline layout is required"),
            ),
            vertex: wgpu::VertexState {
                module: self
                    .vertex_shader_module
                    .as_ref()
                    .expect("Vertex shader module is required"),
                entry_point: self
                    .vertex_entry_point
                    .expect("Vertex shader entrypoint is required")
                    .as_str(),
                buffers: self
                    .vertex_buffer_layouts
                    .expect("Vertex buffer layouts are required")
                    .as_ref(),
            },
            fragment: Some(wgpu::FragmentState {
                module: self
                    .fragment_shader_module
                    .as_ref()
                    .expect("Fragment Shader Module is Required"),
                entry_point: self
                    .fragment_entry_point
                    .expect("Fragment shader entrypoint is required")
                    .as_str(),
                targets: self
                    .fragment_shader_targets
                    .expect("Fragment shader target descriptors are required")
                    .as_ref(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: depth_write.map_or(false, |write| write),
                depth_compare: depth_compare
                    .expect("Depth Comparison function required if depth stencil format is set"),
                stencil: stencil_state.unwrap_or_default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        })
    }
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    shader: wgpu::ShaderModuleDescriptor,
    targets: &[wgpu::ColorTargetState],
    label: Option<&str>,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(&shader);

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_main",
            targets: targets,
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState {
                front: wgpu::StencilFaceState {
                    compare: wgpu::CompareFunction::Always,
                    fail_op: wgpu::StencilOperation::Keep,
                    depth_fail_op: wgpu::StencilOperation::Keep,
                    pass_op: wgpu::StencilOperation::Replace,
                },
                back: wgpu::StencilFaceState {
                    compare: wgpu::CompareFunction::Never,
                    fail_op: wgpu::StencilOperation::Keep,
                    depth_fail_op: wgpu::StencilOperation::Keep,
                    pass_op: wgpu::StencilOperation::Keep,
                },
                read_mask: 0xFF,
                write_mask: 0xFF,
            },
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    });
}

pub fn create_compute_pipeline(
    device: &wgpu::Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    shader: &wgpu::ShaderModuleDescriptor,
    label: Option<&str>,
) -> wgpu::ComputePipeline {
    let shader = device.create_shader_module(shader);

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label,
        layout: Some(&layout),
        module: &shader,
        entry_point: "main",
    })
}
