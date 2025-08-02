use std::{cmp, ops::Range};

pub struct Line {
  string: String,
}

impl Line {
  pub fn from(line_str: &str) -> Self {
    Self {
      string: String::from(line_str),
    }
  }

  /// Returns the substring from `start` to `end` (exclusive).
  /// If `end` exceeds the line length, the slice is silently truncated to the end
  /// of the line, avoiding the surprising `None` returned by the standard library.
  pub fn get(&self, range: Range<usize>) -> String {
    let start = range.start;
    let end = cmp::min(range.end, self.string.len());
    self.string.get(start..end).unwrap_or_default().to_string()
  }

  pub fn len(&self) -> usize {
    self.string.len()
  }
}
