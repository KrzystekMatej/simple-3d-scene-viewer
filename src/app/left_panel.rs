pub struct LeftPanel;

impl LeftPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn ui(&mut self, egui_ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(200.0)
            .max_width(500.0)
            .default_width(260.0)
            .show(egui_ctx, |ui| {
                ui.heading("Panel");
                ui.separator();
            });
    }
}
