use crate::gui::AutopilotGUI;

pub fn gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    let autopilot_gui = AutopilotGUI {
        active_tab: crate::gui::Tabs::Overview,
    };

    eframe::run_native(
        "Autopilot-rs",
        options,
        Box::new(|_cc| Ok(Box::new(autopilot_gui))),
    )
}
