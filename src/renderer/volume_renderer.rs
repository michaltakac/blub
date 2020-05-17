use crate::hybrid_fluid::HybridFluid;
use crate::shader::ShaderDirectory;
use crate::wgpu_utils::pipelines::*;
use std::{path::Path, rc::Rc};

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum VolumeVisualizationMode {
    None,
    Velocity,
    DivergenceUncorrected,
    PseudoPressure,
    Marker,
}

pub struct VolumeRenderer {
    velocity_render_pipeline: RenderPipelineHandle,
    divergence_render_pipeline_desc: RenderPipelineHandle,
    pressure_render_pipeline_desc: RenderPipelineHandle,
    marker_render_pipeline_desc: RenderPipelineHandle,
}

impl VolumeRenderer {
    pub fn new(
        device: &wgpu::Device,
        shader_dir: &ShaderDirectory,
        pipeline_manager: &mut PipelineManager,
        per_frame_bind_group_layout: &wgpu::BindGroupLayout,
        fluid_renderer_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let layout = Rc::new(device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&per_frame_bind_group_layout, &fluid_renderer_group_layout],
        }));

        let mut velocity_render_pipeline_desc = RenderPipelineCreationDesc::new(
            layout.clone(),
            Path::new("volume_visualization_velocity.vert"),
            Some(Path::new("vertex_color.frag")),
        );
        velocity_render_pipeline_desc.primitive_topology = wgpu::PrimitiveTopology::LineList;

        let divergence_render_pipeline_desc = RenderPipelineCreationDesc::new(
            layout.clone(),
            Path::new("volume_visualization_divergence.vert"),
            Some(Path::new("sphere_particles.frag")),
        );

        let pressure_render_pipeline_desc = RenderPipelineCreationDesc::new(
            layout.clone(),
            Path::new("volume_visualization_pressure.vert"),
            Some(Path::new("sphere_particles.frag")),
        );

        let marker_render_pipeline_desc = RenderPipelineCreationDesc::new(
            layout.clone(),
            Path::new("volume_visualization_marker.vert"),
            Some(Path::new("sphere_particles.frag")),
        );

        VolumeRenderer {
            velocity_render_pipeline: pipeline_manager.create_render_pipeline(device, shader_dir, velocity_render_pipeline_desc),
            divergence_render_pipeline_desc: pipeline_manager.create_render_pipeline(device, shader_dir, divergence_render_pipeline_desc),
            pressure_render_pipeline_desc: pipeline_manager.create_render_pipeline(device, shader_dir, pressure_render_pipeline_desc),
            marker_render_pipeline_desc: pipeline_manager.create_render_pipeline(device, shader_dir, marker_render_pipeline_desc),
        }
    }

    fn num_grid_cells(dimension: wgpu::Extent3d) -> u32 {
        dimension.width * dimension.height * dimension.depth
    }

    pub fn draw<'a>(
        &'a self,
        rpass: &mut wgpu::RenderPass<'a>,
        pipeline_manager: &'a PipelineManager,
        fluid: &'a HybridFluid,
        mode: VolumeVisualizationMode,
    ) {
        match mode {
            VolumeVisualizationMode::None => {}
            VolumeVisualizationMode::Velocity => {
                rpass.set_pipeline(pipeline_manager.get_render(&self.velocity_render_pipeline));
                rpass.set_bind_group(1, fluid.bind_group_renderer(), &[]);
                rpass.draw(0..2, 0..Self::num_grid_cells(fluid.grid_dimension()) * 3);
            }
            VolumeVisualizationMode::DivergenceUncorrected => {
                rpass.set_pipeline(pipeline_manager.get_render(&self.divergence_render_pipeline_desc));
                rpass.set_bind_group(1, fluid.bind_group_renderer(), &[]);
                rpass.draw(0..6, 0..Self::num_grid_cells(fluid.grid_dimension()));
            }
            VolumeVisualizationMode::PseudoPressure => {
                rpass.set_pipeline(pipeline_manager.get_render(&self.pressure_render_pipeline_desc));
                rpass.set_bind_group(1, fluid.bind_group_renderer(), &[]);
                rpass.draw(0..6, 0..Self::num_grid_cells(fluid.grid_dimension()));
            }
            VolumeVisualizationMode::Marker => {
                rpass.set_pipeline(pipeline_manager.get_render(&self.marker_render_pipeline_desc));
                rpass.set_bind_group(1, fluid.bind_group_renderer(), &[]);
                rpass.draw(0..6, 0..Self::num_grid_cells(fluid.grid_dimension()));
            }
        }
    }
}
