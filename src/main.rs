#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor; // Load `editor.rs` as a module into this file
use editor::Editor; // Import `Editor` from the `editor` module, so that we can use `Editor` below

fn main() {
    Editor::default().run(); // Here we use `Editor` directly (not `editor::Editor`), since we have
                             // imported it
}
