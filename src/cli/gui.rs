use crate::gui::AutopilotGUI;

// `main` is the entry point of your program.
pub fn gui() -> eframe::Result<()> {
    // Configure the native window options.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]), // Set the initial window size
        ..Default::default()
    };
    let autopilot_gui = AutopilotGUI {
        active_tab: crate::gui::Tabs::Overview,
    };
    // Run the native application!
    eframe::run_native(
        "Autopilot-rs", // The title of your window
        options,        // The native options we just set
        // This closure creates an instance of your `MyApp` struct.
        Box::new(|_cc| Ok(Box::new(autopilot_gui))),
    )
}
