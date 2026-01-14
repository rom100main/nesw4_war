use eframe::egui;

pub fn title(ui: &mut egui::Ui) {
    ui.heading(egui::RichText::new("NESW4: New Extreme Strategical Warfare 4").size(24.0));
    ui.add_space(20.0);
}

pub fn heading(ui: &mut egui::Ui, text: impl Into<String>) {
    ui.label(egui::RichText::new(text).size(18.0));
    ui.add_space(5.0);
}

pub fn heading_small(ui: &mut egui::Ui, text: impl Into<String>) {
    ui.label(egui::RichText::new(text).size(16.0));
    ui.add_space(5.0);
}
