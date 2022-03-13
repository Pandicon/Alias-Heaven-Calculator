include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

#[path = "./calculator.rs"] mod calculator;

use calculator::Calculator;
use dotenv::dotenv;
use eframe::{
    emath::Vec2,
    egui::{
        self, CentralPanel, ScrollArea
    },
    epi::{self, App, IconData, NativeOptions},
    run_native
};
use std::env;
use winconsole;

const W: f32 = 640.0;
const H: f32 = 360.0;

impl App for Calculator {
    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        self.initialise_fonts(&ctx);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        if self.info_active {
            self.render_info(&ctx);
        }
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::new([false, true]).show(ui, |mut ui| {
                self.render_window(&mut ui);
            });
            self.render_footer(ctx);
        });
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        self.name()
    }
}

fn main() {
    dotenv().ok();
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "INFO");
    }
    match env::var("LOGS") {
        Ok(val) => if val.to_lowercase() != "on" && cfg!(target_os = "windows") {
            winconsole::window::hide();
        },
        Err(_) => if cfg!(target_os = "windows") {
            winconsole::window::hide()
        }
    }
    tracing_subscriber::fmt::init();

    let icon_rgba: Vec<u8> = ICON_RGBA.iter().cloned().collect();
    let build_date = BUILD_DATE.iter().cloned().map(|val| val.to_string()).collect();
    let version = VERSION.to_string();
    let general_legacies = GENERAL_LEGACIES.iter().cloned().collect();
    let counting_legacies = COUNTING_LEGACIES.iter().cloned().collect();

    let app = Calculator::new(build_date, version, general_legacies, counting_legacies, SECRET_AREA_COST);
    let mut window_options = NativeOptions::default();
    window_options.icon_data = Some(IconData {rgba: icon_rgba, width: ICON_WIDTH, height: ICON_HEIGHT});
    window_options.initial_window_size = Some(Vec2::new(W, H));
    window_options.resizable = false;
    run_native(Box::new(app), window_options);
}