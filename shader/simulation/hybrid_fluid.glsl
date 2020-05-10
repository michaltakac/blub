// Occupancy calculator: https://xmartlabs.github.io/cuda-calculator/
#define COMPUTE_PASS_PARTICLES layout(local_size_x = 512, local_size_y = 1, local_size_z = 1) in;
#define COMPUTE_PASS_VOLUME layout(local_size_x = 8, local_size_y = 8, local_size_z = 8) in;

layout(set = 1, binding = 0) uniform SimulationProperties { uint NumParticles; };

// Boundary is zero, so texel fetch outside of the domain always gives us boundary cells.
#define CELL_SOLID 0.0
#define CELL_FLUID 1.0
#define CELL_AIR 2.0