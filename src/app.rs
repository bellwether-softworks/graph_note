use std::time::SystemTime;
use eframe::Frame;
use egui::{Context, Grid};
use time::OffsetDateTime;

struct Note {
    created_on: SystemTime,
    title: String,
    text: String,
}

impl Note {
    pub fn new() -> Self {
        Note {
            created_on: SystemTime::now(),
            title: String::new(),
            text: String::new(),
        }
    }
}

pub struct App {
    selected_note: usize,
    notes: Vec<Note>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            notes: vec![Note::new()],
            selected_note: 0,
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
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Add new note").clicked() {
                self.notes.push(Note::new());
                self.selected_note = self.notes.len() - 1;
            }

            let mut to_remove = None;
            let mut to_select = None;

            Grid::new("Notes")
                .show(ui, |ui| {
                    // Header row
                    ui.label("");
                    ui.label("Date");
                    ui.label("Title");
                    ui.end_row();

                    for (index, note) in self.notes.iter().enumerate() {
                        ui.label(if index == self.selected_note { "*" } else { "" });
                        ui.label(OffsetDateTime::from(note.created_on).to_string());
                        ui.label(note.title.as_str());
                        if ui.button("Edit").clicked() {
                            to_select = Some(index);
                        }
                        if ui.button("Delete").clicked() {
                            if self.notes.len() == 1 {
                                // This is the last note; at this point, our selection index is meaningless,
                                // but can't dip below 0, so we'll just ignore it for now
                                to_remove = Some(index);
                            } else if index == self.selected_note && index == self.notes.len() - 1 {
                                // This is the note at the end of the list; let's decrement and display
                                // the last available note
                                to_remove = Some(index);
                                to_select = Some(self.selected_note - 1);
                            } else if index < self.selected_note {
                                // Removing a note before our current selection, so we need to move
                                // with it accordingly
                                to_remove = Some(index);
                                to_select = Some(self.selected_note - 1);
                            } else {
                                to_remove = Some(index);
                            }
                        }
                        ui.end_row();
                    }
                });

            if let Some(to_select) = to_select {
                self.selected_note = to_select;
            }
            if let Some(to_remove) = to_remove {
                self.notes.remove(to_remove);
            }

            ui.separator();

            if let Some(selected_note) = self.notes.get_mut(self.selected_note) {
                ui.horizontal(|ui| {
                    ui.label("Title");
                    ui.text_edit_singleline(&mut selected_note.title);
                });

                ui.label(format!("Created at {:?}", OffsetDateTime::from(selected_note.created_on)));

                ui.label("Text");
                egui::TextEdit::multiline(&mut selected_note.text).show(ui);
            }
        });
    }
}
