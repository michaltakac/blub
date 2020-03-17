// TODO: Not a particle renderer yet.
// The idea is to have different render backend for the fluid, which one being the particle renderer which renders the fluid as particles (sprites)

use super::shader::*;

pub struct ParticleRenderer {
    render_pipeline: wgpu::RenderPipeline,
    pipeline_layout: wgpu::PipelineLayout,
    bind_group: wgpu::BindGroup,
}

impl ParticleRenderer {
    pub fn new(device: &wgpu::Device) -> ParticleRenderer {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });
        let render_pipeline = Self::create_pipeline_state(device, &pipeline_layout).unwrap();

        ParticleRenderer {
            render_pipeline,
            pipeline_layout,
            bind_group,
        }
    }

    fn create_pipeline_state(device: &wgpu::Device, pipeline_layout: &wgpu::PipelineLayout) -> Option<wgpu::RenderPipeline> {
        let vs_module = create_glsl_shader_module(device, include_str!("shaders/shader.vert"), ShaderStage::Vertex)?;
        let fs_module = create_glsl_shader_module(device, include_str!("shaders/shader.frag"), ShaderStage::Fragment)?;

        Some(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: super::Application::backbuffer_format(),
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        }))
    }

    pub fn try_reload_shaders(&mut self, device: &wgpu::Device) {
        if let Some(render_pipeline) = Self::create_pipeline_state(device, &self.pipeline_layout) {
            self.render_pipeline = render_pipeline;
        }
    }

    pub fn draw(&self, rpass: &mut wgpu::RenderPass) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}
