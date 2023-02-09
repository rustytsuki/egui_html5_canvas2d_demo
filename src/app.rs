use eframe::egui;
use egui_demo_lib;

pub struct Html5Canvas2dApp {
    demo_windows: egui_demo_lib::DemoWindows,
}

impl Html5Canvas2dApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_styles(&cc.egui_ctx);

        Self {
            demo_windows: egui_demo_lib::DemoWindows::default(),
        }
    }
}

impl eframe::App for Html5Canvas2dApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_windows.ui(ctx);
    }
}

fn setup_custom_styles(ctx: &egui::Context) {
    let style = egui::Style {
        animation_time: 0., // running in canvas2d is slower, so we disable animation
        ..egui::Style::default()
    };
    ctx.set_style(style);
}
