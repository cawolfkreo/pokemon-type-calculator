use std::{default::Default, str::FromStr};

use eframe::egui::{ComboBox, Ui};
use eframe::egui;

use super::type_gui;
use super::empty_widget::EmptyWidget;

use pokemon_type_effectiveness::{PokemonTypes, DefenseType, Fight};

enum DropdownSelect {
	Attack,
	Defense1,
	Defense2
}

pub struct CentralPanel{
	type_labels: Vec<&'static str>,
	selected_atk_type: type_gui::TypeGui,
	selected_def1_type: type_gui::TypeGui,
	selected_def2_type: type_gui::TypeGui,  
}

impl Default for CentralPanel {
	fn default() -> Self {
		let type_labels = type_gui::TypeGui::list_all_types();
		Self {
			type_labels,
			selected_atk_type: type_gui::TypeGui::None,
			selected_def1_type: type_gui::TypeGui::None,
			selected_def2_type: type_gui::TypeGui::None,
		}
	}
}

impl CentralPanel {
	pub fn display_central_panel(&mut self, ui: &mut Ui) {
		ui.vertical_centered_justified(|ui| {
			ui.heading("Effectiveness calculator");

			self.create_spacing(ui, 0.0, 30.0);
			
			egui::Grid::new("types grid")
			.num_columns(4)
			.min_col_width(109.0)
			.show(ui, |ui| {
				self.create_combo_box("Attack type:", ui, DropdownSelect::Attack);

				ui.vertical_centered_justified(|ui| {
					self.create_spacing(ui, 0.0, 20.0);
					ui.strong("vs");
				});
				
				self.create_combo_box("First defense type:", ui, DropdownSelect::Defense1);

				if self.selected_def1_type != type_gui::TypeGui::None {
					self.create_combo_box("(opt) Second defense type:", ui, DropdownSelect::Defense2);
				} else {
					self.selected_def2_type = type_gui::TypeGui::None;
				}
			});
			
			self.create_spacing(ui, 40.0, 15.0);
			
			ui.separator();
			
			self.create_spacing(ui, 40.0, 15.0);
			
			self.check_selections(ui);
		});            
	}
	
	fn create_combo_box(&mut self, combo_label: &str, ui: &mut Ui, dropdown_select: DropdownSelect) {
		let dropdown_value = match dropdown_select {
			DropdownSelect::Attack => &mut self.selected_atk_type,
			DropdownSelect::Defense1 => &mut self.selected_def1_type,
			DropdownSelect::Defense2 => &mut self.selected_def2_type,
		};
		
		let current_type_str = dropdown_value.to_string();
		
		ui.vertical(|ui| {
			ui.label(combo_label);

			ui.set_width_range(90.0..=120.0);
			
			ComboBox::from_id_source(combo_label)
			.selected_text(current_type_str)
			.show_ui(ui, |ui| {
				ui.set_min_width(95.0);
				
				for type_str in &self.type_labels {
					if let Ok(type_gui) = type_gui::TypeGui::from_str(type_str) {
						ui.selectable_value(
							dropdown_value, 
							type_gui,
							type_str.to_string()
						);
					}
				}
			});
		});
	}
	
	fn create_spacing(&self, ui: &mut Ui, width: f32, height: f32) {
		
		let position = ui.next_widget_position();
		
		let rect = egui::Rect::from_min_size(position, egui::Vec2 { x: width, y: height });
		
		ui.put(rect, EmptyWidget);
	}
	
	fn check_selections(&self, ui: &mut Ui) {
		let Some(attack): Option<PokemonTypes> = self.selected_atk_type.into() else {
			return;
		};
		
		let Some(def1_type): Option<PokemonTypes> = self.selected_def1_type.into() else {
			return;
		};

		let defense = if let Some(def2_type) = self.selected_def2_type.into() {
			// two defenses selected
			DefenseType::TwoTypes(def1_type, def2_type)
		} else {
			// one defense selected
			DefenseType::OneType(def1_type)
		};

		let fight = Fight::new(attack, defense);

		self.calculate_effectiveness(fight, ui);
	}

	fn calculate_effectiveness(&self, fight: Fight, ui: &mut Ui) {
		let effectiveness = fight.calculate_effectiveness();

		ui.label(format!("{fight} {effectiveness}"));
	}
}
