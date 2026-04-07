mod app;
mod metadata;
mod note;

use std::path::PathBuf;

fn main() -> eframe::Result {
    let notes_dir = PathBuf::from("notes");

    // Seed a couple of notes if the directory doesn't exist yet
    if !notes_dir.exists() {
        let _ = note::create_note(
            &notes_dir,
            "hello-world",
            "This is my **first** note.",
            vec!["intro".into()],
        );
        let _ = note::create_note(
            &notes_dir,
            "rust-tips",
            "- Use `?` for error propagation\n- Prefer `anyhow` for app code",
            vec!["rust".into()],
        );
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Markdown Notes",
        options,
        Box::new(|_cc| Ok(Box::new(app::NoteApp::new(notes_dir)))),
    )
}
