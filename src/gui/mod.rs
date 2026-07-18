use eframe::egui;

pub struct AutopilotGUI {
    pub active_tab: Tabs,
}
#[derive(PartialEq)]
pub enum Tabs {
    Overview,
    Jobs,
    Settings,
}

impl eframe::App for AutopilotGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Autopilot");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            Tabs::Overview => {
                ui.centered_and_justified(|ui| {
                    ui.heading("WIP");
                });
            }
            Tabs::Jobs => {}
            Tabs::Settings => {}
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
