#![warn(
  clippy::all,
  clippy::pedantic,
  clippy::print_stdout,
  clippy::arithmetic_side_effects,
  clippy::as_conversions,
  clippy::integer_division
)]
mod editor;
use editor::Editor;

fn main() {
  let editor = Editor::new();

  if let Ok(mut editor) = editor {
    editor.run();
  } else {
    println!("Failed to start editor. Make sure you are running in an interactive terminal.");
  }
}
