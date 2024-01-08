use eframe::egui::{Widget, Sense};

pub struct EmptyWidget;

impl EmptyWidget {
    
}

impl Widget for EmptyWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let size = ui.style().spacing.interact_size;
        let (_, response) = ui.allocate_exact_size(size, Sense::hover());

        response
    }
}