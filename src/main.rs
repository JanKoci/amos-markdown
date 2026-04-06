use anyhow::Result;
use std::path::Path;

mod metadata;
mod note;

fn main() -> Result<()> {
    let notes_dir = Path::new("notes");

    // Create a couple of test notes
    note::create_note(
        notes_dir,
        "hello-world",
        "This is my **first** note.",
        vec!["intro".into(), "test".into()],
    )?;
    note::create_note(
        notes_dir,
        "rust-tips",
        "- Use `?` for error propagation\n- Prefer `anyhow` for app code",
        vec!["rust".into()],
    )?;

    // Load them all back
    let notes = note::load_all_notes(notes_dir)?;
    println!("Loaded {} notes:\n", notes.len());
    for n in &notes {
        println!("=== {} ===", n.title);
        if let Some(meta) = &n.metadata {
            println!("tags: {:?}", meta.tags);
        }
        println!("{}\n", n.plain_text);
    }

    Ok(())
}
