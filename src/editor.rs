use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::cmp::min;
mod terminal;
use terminal::{Position, Size, Terminal};
mod view;
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor_location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_cursor(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location {
            mut row,
            mut column,
        } = self.cursor_location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            // up: cursor move up
            KeyCode::Up => {
                column = column.saturating_sub(1);
            }
            // down: cursor move down
            KeyCode::Down => {
                column = min(height.saturating_sub(1), column.saturating_add(1));
            }
            // left: cursor move left
            KeyCode::Left => {
                row = row.saturating_sub(1);
            }
            // right: cursor move right
            KeyCode::Right => {
                row = min(width.saturating_sub(1), row.saturating_add(1));
            }
            // pageup: cursor move to top
            KeyCode::PageUp => {
                row = 0;
            }
            // pagedown: cursor move to bottom
            KeyCode::PageDown => {
                row = height.saturating_sub(1);
            }
            // home: cursor move to leftmost
            KeyCode::Home => {
                column = 0;
            }
            // end: cursor move to rightmost
            KeyCode::End => {
                column = width.saturating_sub(1);
            }
            _ => {}
        }
        self.cursor_location = Location { row, column };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                // ctrl + q: quit
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
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
                    self.move_cursor(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            View::render()?;
            Terminal::move_cursor_to(Position {
                x: self.cursor_location.row,
                y: self.cursor_location.column,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
