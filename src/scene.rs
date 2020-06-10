use crate::hybrid_fluid::HybridFluid;
use crate::wgpu_utils::{pipelines::PipelineManager, shader::ShaderDirectory};

pub struct FluidConfig {
    pub world_position: cgmath::Point3<f32>,
    pub grid_to_world_scale: f32,
    pub grid_dimension: wgpu::Extent3d,
}

// Data describing a new scene.
pub struct SceneConfig {
    // global gravity (in world space)
    pub gravity: cgmath::Vector3<f32>,
    pub fluid: FluidConfig,
}

// Scene data & simulation.
pub struct Scene {
    hybrid_fluid: HybridFluid,
    pub config: SceneConfig,
}

impl Scene {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        init_encoder: &mut wgpu::CommandEncoder,
        shader_dir: &ShaderDirectory,
        pipeline_manager: &mut PipelineManager,
        per_frame_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let config = SceneConfig {
            gravity: cgmath::vec3(0.0, -9.81, 0.0),
            fluid: FluidConfig {
                world_position: cgmath::point3(0.0, 0.0, 0.0),
                grid_to_world_scale: 1.0 / 128.0,
                grid_dimension: wgpu::Extent3d {
                    width: 128,
                    height: 64,
                    depth: 64,
                },
            },
        };

        let mut hybrid_fluid = HybridFluid::new(
            device,
            queue,
            config.fluid.grid_dimension,
            2000000,
            shader_dir,
            pipeline_manager,
            per_frame_bind_group_layout,
        );

        hybrid_fluid.add_fluid_cube(
            device,
            init_encoder,
            cgmath::Point3::new(1.0, 1.0, 1.0),
            cgmath::Point3::new(64.0, 40.0, 64.0),
        );
        hybrid_fluid.set_gravity_grid(config.gravity / config.fluid.grid_to_world_scale);

        Scene { hybrid_fluid, config }
    }

    pub fn step<'a>(&'a self, cpass: &mut wgpu::ComputePass<'a>, pipeline_manager: &'a PipelineManager, queue: &wgpu::Queue) {
        self.hybrid_fluid.step(cpass, pipeline_manager, queue);
    }

    pub fn fluid(&self) -> &HybridFluid {
        &self.hybrid_fluid
    }
}
