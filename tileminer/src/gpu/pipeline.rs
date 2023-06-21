use crate::gpu::window::WindowContext;
use wgpu::util::DeviceExt;
use wgpu::*;

impl WindowContext {
    pub fn write_buffer(&self, buffer: &Buffer, data: &[u8]) {
        self.queue.write_buffer(buffer, 0, data);
    }

    pub fn buffer_init(&self, label: &str, data: &[u8], usage: BufferUsages) -> Buffer {
        self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage,
        })
    }

    pub fn bind_group_layout_buffer(
        &self,
        label: &str,
        entries: &[(ShaderStages, BufferBindingType)],
    ) -> BindGroupLayout {
        let entries: Vec<_> = entries
            .iter()
            .enumerate()
            .map(|(i, (visibility, ty))| BindGroupLayoutEntry {
                binding: i as u32,
                visibility: *visibility,
                ty: BindingType::Buffer {
                    ty: *ty,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();

        self.device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: entries.as_slice(),
                label: Some(label),
            })
    }

    pub fn bind_group(
        &self,
        label: &str,
        layout: &BindGroupLayout,
        entries: &[BindingResource],
    ) -> BindGroup {
        let entries: Vec<_> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| BindGroupEntry {
                binding: i as u32,
                resource: e.clone(),
            })
            .collect();

        self.device.create_bind_group(&BindGroupDescriptor {
            layout,
            entries: entries.as_slice(),
            label: Some(label),
        })
    }

    pub fn bind_group_layout_texture(&self) -> BindGroupLayout {
        self.device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true }, // same
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering), // as here
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            })
    }

    // This could go right in create_render_pipeline but maybe its good to let you reuse layouts.
    pub fn pipeline_layout(&self, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        self.device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts,
                push_constant_ranges: &[],
            })
    }

    pub fn compute_pipeline(
        &self,
        label: &str,
        layout: &PipelineLayout,
        shader: &str,
    ) -> ComputePipeline {
        let shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(shader.into()),
        });
        self.device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some(label),
                layout: Some(layout),
                module: &shader,
                entry_point: "cs_main",
            })
    }

    pub fn render_pipeline(
        &self,
        label: &str,
        layout: &PipelineLayout,
        vertex_layouts: &[VertexBufferLayout],
        shader: &str,
    ) -> RenderPipeline {
        let shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(shader.into()),
        });
        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: vertex_layouts,
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: self.config.borrow().format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            })
    }

    pub fn command_encoder(&self, label: &str) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor { label: Some(label) })
    }

    pub fn render_pass<'f>(
        &self,
        encoder: &'f mut CommandEncoder,
        screen_texture: &'f TextureView,
    ) -> RenderPass<'f> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: screen_texture,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        })
    }
}
