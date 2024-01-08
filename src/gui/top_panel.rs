use eframe::egui;
use eframe::egui::Ui;

#[derive(Default)]
pub struct TopPanel{}

impl TopPanel {
    pub fn display_top_panel(&self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            //ui.visuals_mut().button_frame = false;
            self.display_options_menu(ui);
        });
    }

    fn display_options_menu(&self, ui: &mut Ui) {
        egui::global_dark_light_mode_switch(ui);

        ui.separator();   
    }
}
