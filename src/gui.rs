use std::default::Default;

use eframe::{egui, NativeOptions};
use eframe::egui::{vec2, ViewportBuilder};

mod type_gui;
mod central_panel;
mod empty_widget;

use central_panel::CentralPanel;

pub fn run() -> Result<(), eframe::Error> {

	let view_build = ViewportBuilder {
		resizable: Some(false),
		maximize_button: Some(false),
		inner_size: Some(vec2(558.0, 230.0)),
		..Default::default()
	};

	let native_options = NativeOptions { 
		viewport: view_build,
		..Default::default()
	};

	eframe::run_native(
		"Pokemon type effectiveness calculator",
		native_options,
		Box::new(|_cc| Box::<EffectivenesCalculatorApp>::default())
	)
}

#[derive(Default)]
struct EffectivenesCalculatorApp {
	central_panel: CentralPanel,
}

impl eframe::App for EffectivenesCalculatorApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(
			ctx, 
			|ui| self.central_panel.display_central_panel(ui)
		);
	}
}
