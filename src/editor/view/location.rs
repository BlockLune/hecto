use crate::editor::terminal::Position;

#[derive(Copy, Clone, Default)]
pub struct Location {
  pub column: usize,
  pub row: usize,
}

impl From<Location> for Position {
  fn from(location: Location) -> Self {
    Self {
      x: location.column,
      y: location.row,
    }
  }
}

impl Location {
  pub const fn subtract(&self, other: &Self) -> Self {
    Self {
      column: self.column.saturating_sub(other.column),
      row: self.row.saturating_sub(other.row),
    }
  }
}
