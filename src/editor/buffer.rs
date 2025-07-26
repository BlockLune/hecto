#[derive(Default)]
pub struct Buffer {
  pub lines: Vec<String>,
}

impl Buffer {
  pub fn new(filename: &str) -> Self {
    match std::fs::read_to_string(filename) {
      Ok(file_contents) => Self {
        lines: file_contents.lines().map(String::from).collect(),
      },
      _ => Self { lines: vec![] },
    }
  }
}
