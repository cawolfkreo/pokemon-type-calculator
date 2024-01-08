use std::default::Default;

use eframe::{egui, NativeOptions};
use eframe::egui::{vec2, ViewportBuilder};

mod type_gui;
mod empty_widget;
mod top_panel;
mod central_panel;

use central_panel::CentralPanel;
use top_panel::TopPanel;

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
	top_panel: TopPanel,
	central_panel: CentralPanel,
}

impl eframe::App for EffectivenesCalculatorApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("Options Bar").show(
			ctx, 
			|ui| self.top_panel.display_top_panel(ui)
		);

		egui::CentralPanel::default().show(
			ctx, 
			|ui| self.central_panel.display_central_panel(ui)
		);
	}
}
