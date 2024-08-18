use eframe::Frame;
use egui::Context;

pub struct App {
    note: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            note: String::new()
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Access to the CreationContext during instantiation allows us to take
        // ownership of some aspects of our app, such as wiring up fonts.
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Note text");

            egui::TextEdit::multiline(&mut self.note).show(ui);
        });
    }
}
