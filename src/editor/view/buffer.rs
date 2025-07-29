use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
  pub lines: Vec<String>,
}

impl Buffer {
  pub fn load(filename: &str) -> Result<Self, std::io::Error> {
    Ok(Self {
      lines: read_to_string(filename)?
        .lines()
        .map(String::from)
        .collect(),
    })
  }
}
