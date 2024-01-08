use std::{default::Default, str::FromStr};

use eframe::egui::{ComboBox, Ui};

use crate::gui::type_gui;

pub struct CentralPanel{
    type_labels: Vec<&'static str>,
    selected_atk_type: type_gui::TypeGui,
}

impl CentralPanel {
    pub fn display_central_panel(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Effectiveness calculator");
            
            ui.columns(4, |columns| {
                columns[0].horizontal(|ui| self.create_combo_box("Attack type:", ui));

                columns[1].set_max_height(30.0);
                columns[1].horizontal_centered(|ui| {
                    ui.shrink_height_to_current();
                    ui.vertical_centered_justified(|ui| ui.strong("vs"));
                });

                columns[2].horizontal(|ui| self.create_combo_box("First defense type:", ui));
                columns[3].horizontal(|ui| self.create_combo_box("Second defense type:", ui));
                
            });

            ui.separator();
        });            
    }
    
    fn create_combo_box(&mut self, combo_label: &str, ui: &mut Ui) {
        let current_type_str = &self.selected_atk_type.to_string();
        
        ui.vertical(|ui| {
            ui.label(combo_label);
            
            ComboBox::from_id_source(combo_label)
            .selected_text(current_type_str)
            .show_ui(ui, |ui| {
                ui.set_min_width(90.0);
                
                for type_str in &self.type_labels {
                    if let Ok(type_gui) = type_gui::TypeGui::from_str(type_str) {
                        ui.selectable_value(
                            &mut self.selected_atk_type, 
                            type_gui,
                            type_str.to_string()
                        );
                    }
                }
            });
        });
    }
}

impl Default for CentralPanel {
    fn default() -> Self {
        let type_labels = type_gui::TypeGui::list_all_types();
        Self {
            type_labels,
            selected_atk_type: type_gui::TypeGui::None 
        }
    }
}