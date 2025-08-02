use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;

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
    self
      .string
      .graphemes(true)
      .skip(start)
      .take(end.saturating_sub(start))
      .collect()
  }

  pub fn len(&self) -> usize {
    self.string[..].graphemes(true).count()
  }
}
