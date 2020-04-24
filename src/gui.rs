use imgui::im_str;
use std::convert::TryFrom;

struct State {
    selected_simulation_mode: usize,
    selected_simulation_time_seconds: f32,
}

pub struct GUI {
    imgui_context: imgui::Context,
    imgui_platform: imgui_winit_support::WinitPlatform,
    imgui_renderer: imgui_wgpu::Renderer,
    state: State,
}

impl GUI {
    pub fn new(device: &wgpu::Device, window: &winit::window::Window, command_queue: &mut wgpu::Queue) -> Self {
        let mut imgui_context = imgui::Context::create();

        let mut imgui_platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);
        imgui_platform.attach_window(imgui_context.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);

        imgui_context.set_ini_filename(None);

        let hidpi_factor = 1.0;
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui_context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let imgui_renderer = imgui_wgpu::Renderer::new(&mut imgui_context, &device, command_queue, crate::Screen::FORMAT_BACKBUFFER, None);

        GUI {
            imgui_context,
            imgui_platform,
            imgui_renderer,

            state: State {
                selected_simulation_mode: crate::SimulationMode::SimulateAndRender as usize,
                selected_simulation_time_seconds: 60.0 * 60.0, // (an hour)
            },
        }
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        window: &winit::window::Window,
        encoder: &mut wgpu::CommandEncoder,
        command_queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        simulation: &mut crate::Simulation,
    ) {
        let context = &mut self.imgui_context;
        let state = &mut self.state;
        //self.imgui_context.io_mut().update_delta_time(timer.frame_delta()); // Needed?
        self.imgui_platform
            .prepare_frame(context.io_mut(), window)
            .expect("Failed to prepare imgui frame");
        let ui = context.frame();
        {
            let window = imgui::Window::new(im_str!("Blub"));
            window
                .position([0.0, 0.0], imgui::Condition::FirstUseEver)
                .always_auto_resize(true)
                .build(&ui, || {
                    ui.text(im_str!(
                        "{:3.2}ms, FPS: {:3.2}",
                        simulation.timer.duration_for_last_frame().as_secs_f64() * 1000.0,
                        1000.0 / 1000.0 / simulation.timer.duration_for_last_frame().as_secs_f64()
                    ));
                    ui.separator();
                    ui.text(im_str!(
                        "num simulation steps current frame: {}",
                        simulation.timer.num_simulation_steps_performed_for_current_frame()
                    ));
                    ui.text(im_str!("rendered time:  {:.2}", simulation.timer.total_render_time().as_secs_f64()));
                    ui.text(im_str!("simulated time: {:.2}", simulation.timer.total_simulated_time().as_secs_f64()));
                    ui.text(im_str!(
                        "total num simulation steps: {}",
                        simulation.timer.num_simulation_steps_performed()
                    ));

                    ui.separator();
                    imgui::ComboBox::new(im_str!("Simulation Mode")).build_simple_string(
                        &ui,
                        &mut state.selected_simulation_mode,
                        &[im_str!("Simulate & Render"), im_str!("Simulate, Render Result"), im_str!("Record")],
                    );
                    ui.input_float(im_str!("target simulation time (s)"), &mut state.selected_simulation_time_seconds)
                        .step(0.1)
                        .build();
                    if ui.small_button(im_str!("Apply & Reset")) {
                        simulation.mode = crate::SimulationMode::try_from(state.selected_simulation_mode).unwrap();
                        simulation.simulation_length = std::time::Duration::from_secs_f32(state.selected_simulation_time_seconds);
                        simulation.restart(device, command_queue);
                    }
                    //ui.separator();
                });
        }

        self.imgui_platform.prepare_render(&ui, &window);
        self.imgui_renderer
            .render(ui.render(), &device, encoder, view)
            .expect("IMGUI rendering failed");
    }

    pub fn handle_event(&mut self, window: &winit::window::Window, event: &winit::event::Event<()>) {
        self.imgui_platform.handle_event(self.imgui_context.io_mut(), window, event);
    }
}