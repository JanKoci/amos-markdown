use anyhow::Result;
use std::path::Path;

mod note;

fn main() -> Result<()> {
    let notes_dir = Path::new("notes");

    // Create a couple of test notes
    note::create_note(
        notes_dir,
        "hello-world",
        "# Hello World\n\nThis is my
  first note.",
    )?;
    note::create_note(
        notes_dir,
        "rust-tips",
        "# Rust Tips\n\n- Use `?` for
  error propagation\n- Prefer `anyhow` for app code",
    )?;

    // Load them all back
    let notes = note::load_all_notes(notes_dir)?;
    println!("Loaded {} notes:\n", notes.len());
    for n in &notes {
        println!("--- {} ---\n{}\n", n.title, n.body);
        println!("-- HTML --\n{}", n.html);
        println!("-- Plain text --\n{}\n", n.plain_text);
    }

    Ok(())
}
