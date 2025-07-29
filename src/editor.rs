mod terminal;
mod view;

use crossterm::event::{
  read,
  Event::{self, Key, Resize},
  KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::panic::{set_hook, take_hook};
use terminal::{Position, Size, Terminal};
use view::View;

pub struct Editor {
  position: Position,
  should_quit: bool,
  view: View,
}

impl Drop for Editor {
  fn drop(&mut self) {
    let _ = Terminal::terminate();
    if self.should_quit {
      let _ = Terminal::print("Goodbye.\r\n");
    }
  }
}

impl Editor {
  pub fn new() -> Result<Self, std::io::Error> {
    let current_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
      let _ = Terminal::terminate();
      current_hook(panic_info);
    }));

    Terminal::initialize()?;
    let mut view = View::default();
    let args: Vec<String> = std::env::args().collect();

    // For now, we will only handle one file.
    // Notice that the index 0 element is the name of the program itself, so we start from index 1.
    if let Some(file_name) = args.get(1) {
      view.load(file_name);
    }

    Ok(Self {
      position: Position::default(),
      should_quit: false,
      view,
    })
  }

  pub fn run(&mut self) {
    loop {
      self.refresh_screen();
      if self.should_quit {
        break;
      }
      match read() {
        Ok(event) => self.evaluate_event(event),
        Err(err) => {
          #[cfg(debug_assertions)]
          {
            panic!("Could not read event: {err:?}");
          }
        }
      }
    }
  }

  fn move_cursor(&mut self, key_code: KeyCode) {
    self.view.move_cursor(key_code);
    self.position = Position {
      x: self.view.location.column - self.view.scroll_offset.column,
      y: self.view.location.row - self.view.scroll_offset.row,
    }
  }

  fn evaluate_event(&mut self, event: Event) {
    match event {
      Key(KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        ..
      }) => {
        match code {
          // ctrl + q: quit
          KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
            self.should_quit = true;
          }
          // up, down, left, right, pageup, pagedown, home, end: move cursor
          KeyCode::Up
          | KeyCode::Down
          | KeyCode::Left
          | KeyCode::Right
          | KeyCode::PageUp
          | KeyCode::PageDown
          | KeyCode::Home
          | KeyCode::End => {
            self.move_cursor(code);
          }
          _ => (),
        }
      }
      Resize(width_u16, height_u16) => {
        // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        self.view.resize(Size { height, width });
      }
      _ => {}
    }
  }

  fn refresh_screen(&mut self) {
    let _ = Terminal::hide_cursor();
    self.view.render();
    let _ = Terminal::move_cursor_to(self.position);
    let _ = Terminal::show_cursor();
    let _ = Terminal::execute();
  }
}
