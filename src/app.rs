use crate::note::{self, Note};
use eframe::egui;
use std::path::PathBuf;

pub struct NoteApp {
    notes_dir: PathBuf,
    notes: Vec<Note>,
    selected: Option<usize>, // index into `notes`
    search_query: String,
    new_note_title: String,
}

impl NoteApp {
    pub fn new(notes_dir: PathBuf) -> Self {
        let notes = note::load_all_notes(&notes_dir).unwrap_or_default();
        Self {
            notes_dir,
            notes,
            selected: None,
            search_query: String::new(),
            new_note_title: String::new(),
        }
    }

    fn reload(&mut self) {
        self.notes = note::load_all_notes(&self.notes_dir).unwrap_or_default();
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left panel — note list + controls
        egui::SidePanel::left("note_list")
            .resizable(true)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Notes");
                ui.separator();

                // Search box
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.search_query);
                });
                ui.separator();

                // Filtered note list
                let query = self.search_query.to_lowercase();
                let indices: Vec<usize> = self
                    .notes
                    .iter()
                    .enumerate()
                    .filter(|(_, n)| {
                        query.is_empty()
                            || n.title.to_lowercase().contains(&query)
                            || n.plain_text.to_lowercase().contains(&query)
                    })
                    .map(|(i, _)| i)
                    .collect();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in indices {
                        let note = &self.notes[i];
                        let selected = self.selected == Some(i);
                        if ui.selectable_label(selected, &note.title).clicked() {
                            self.selected = Some(i);
                        }
                        // Show tags as small dimmed text
                        if let Some(meta) = &note.metadata {
                            if !meta.tags.is_empty() {
                                ui.label(egui::RichText::new(meta.tags.join(", ")).small().weak());
                            }
                        }
                    }
                });

                ui.separator();

                // New note controls
                ui.label("New note:");
                ui.text_edit_singleline(&mut self.new_note_title);
                if ui.button("Create").clicked() && !self.new_note_title.is_empty() {
                    let title = self.new_note_title.drain(..).collect::<String>();
                    let _ =
                        note::create_note(&self.notes_dir, &title, "Write your note here.", vec![]);
                    self.reload();
                }
            });

        // Right panel — note viewer
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected.and_then(|i| self.notes.get(i)) {
                None => {
                    ui.centered_and_justified(|ui| {
                        ui.label("Select a note to view it.");
                    });
                }
                Some(note) => {
                    ui.heading(&note.title);
                    if let Some(meta) = &note.metadata {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("tags: {}", meta.tags.join(", ")))
                                    .weak()
                                    .small(),
                            );
                            if let Some(created) = &meta.created {
                                ui.label(
                                    egui::RichText::new(format!("created: {created}"))
                                        .weak()
                                        .small(),
                                );
                            }
                        });
                    }
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(&note.plain_text);
                    });
                }
            }
        });
    }
}
