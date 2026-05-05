use eframe::egui;

// This is the main app struct. It holds all the state for your application.
// For this simple example, it doesn't need to hold any data, so it's empty.

pub struct AutopilotGUI {
    pub active_tab: Tabs,
}
#[derive(PartialEq)]
pub enum Tabs {
    Overview,
    Jobs,
    Settings,
}

// The `eframe::App` trait is what integrates your UI with the eframe framework.
// You must implement the `update` method, which is called every frame.
impl eframe::App for AutopilotGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // `CentralPanel` is one of egui's built-in panel types.
        // It fills the entire central area of your window.
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Autopilot");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // This is where you place your UI widgets!
            match self.active_tab {
                Tabs::Overview => {
                    ui.centered_and_justified(|ui| {
                        ui.heading("WIP");
                    }
                    );
                }
                Tabs::Jobs => {}
                Tabs::Settings => {}
            }
        });
        egui::TopBottomPanel::bottom("bottombar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.active_tab == Tabs::Overview, "Overview")
                    .clicked()
                {
                    self.active_tab = Tabs::Overview;
                }
                if ui
                    .selectable_label(self.active_tab == Tabs::Jobs, "Jobs")
                    .clicked()
                {
                    self.active_tab = Tabs::Jobs;
                }
                if ui
                    .selectable_label(self.active_tab == Tabs::Settings, "Settings")
                    .clicked()
                {
                    self.active_tab = Tabs::Settings;
                }
            });
            ui.spacing();
        });
    }
}
