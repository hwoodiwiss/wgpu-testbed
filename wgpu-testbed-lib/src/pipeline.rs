use std::rc::Rc;

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
    vertex_shader_module: Option<Rc<wgpu::ShaderModule>>,
    vertex_buffer_layouts: Option<Vec<wgpu::VertexBufferLayout<'a>>>,
    fragment_entry_point: Option<String>,
    fragment_shader_module: Option<Rc<wgpu::ShaderModule>>,
    fragment_shader_targets: Option<Vec<wgpu::ColorTargetState>>,
    depth_stencil_format: Option<wgpu::TextureFormat>,
    depth_write: Option<bool>,
    depth_compare: Option<wgpu::CompareFunction>,
    stencil_state: Option<wgpu::StencilState>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn create(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            ..Default::default()
        }
    }

    pub fn with_layout(&'a mut self, pipeline_layout: wgpu::PipelineLayout) -> &'a mut Self {
        self.layout = Some(Box::new(pipeline_layout));
        self
    }

    pub fn with_vertex_shader(
        &'a mut self,
        entry_point: &str,
        shader_module: Rc<wgpu::ShaderModule>,
        buffer_layouts: &'a [wgpu::VertexBufferLayout],
    ) -> &'a mut Self {
        self.vertex_entry_point = Some(String::from(entry_point));
        self.vertex_shader_module = Some(shader_module);
        self.vertex_buffer_layouts = Some((*buffer_layouts).to_vec());
        self
    }

    pub fn with_fragment_shader(
        &'a mut self,
        entry_point: &str,
        shader_module: Rc<wgpu::ShaderModule>,
    ) -> &'a mut Self {
        self.fragment_entry_point = Some(String::from(entry_point));
        self.fragment_shader_module = Some(shader_module);
        self
    }

    pub fn with_render_targets(
        &'a mut self,
        render_targets: &[wgpu::ColorTargetState],
    ) -> &'a mut Self {
        self.fragment_shader_targets = Some((*render_targets).to_vec());
        self
    }

    pub fn with_depth(
        &'a mut self,
        depth_format: wgpu::TextureFormat,
        write_depth: bool,
        depth_compare: wgpu::CompareFunction,
    ) -> &'a mut Self {
        self.depth_stencil_format = Some(depth_format);
        self.depth_write = Some(write_depth);
        self.depth_compare = Some(depth_compare);
        self
    }

    pub fn with_stencil(
        &'a mut self,
        face_stencil_ops: wgpu::StencilFaceState,
        backface_stencil_ops: wgpu::StencilFaceState,
        read_mask: Option<u32>,
        write_mask: Option<u32>,
    ) -> &'a mut Self {
        self.stencil_state = Some(wgpu::StencilState {
            front: face_stencil_ops,
            back: backface_stencil_ops,
            read_mask: read_mask.map_or(0xff, |mask| mask),
            write_mask: write_mask.map_or(0xff, |mask| mask),
        });
        self
    }
}

impl RenderPipelineBuilder<'_> {
    pub fn build(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let depth_format = self.depth_stencil_format;
        let depth_write = self.depth_write;
        let depth_compare = self.depth_compare;
        let stencil_state = self.stencil_state.take();
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
                    .take()
                    .expect("Vertex shader entrypoint is required")
                    .as_str(),
                buffers: self
                    .vertex_buffer_layouts
                    .take()
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
                    .take()
                    .expect("Fragment shader entrypoint is required")
                    .as_str(),
                targets: self
                    .fragment_shader_targets
                    .take()
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
