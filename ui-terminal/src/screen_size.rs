use std::process::Command;

#[derive(Debug)]
pub struct TerminalScreenSize {
    pub lines: i32,
    pub cols: i32
}

#[derive(Debug)]
pub struct ScreenSizeError {
    pub message: String
}

impl From<std::io::Error> for ScreenSizeError {
    fn from(e: std::io::Error) -> Self {
        Self { message: format!("Failed to read: {}", e) }
    }
}

impl From<std::str::Utf8Error> for ScreenSizeError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self { message: format!("Failed to parse to UTF-8: {}", e) }
    }
}

impl From<std::num::ParseIntError> for ScreenSizeError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self { message: format!("Failed to parse int: {}", e) }
    }
}

fn spawn_and_parse_output_as_integer(program: &str, arg: &str) -> Result<i32, ScreenSizeError> {
    let mut output = Command::new(program)
        .args(vec![arg])
        .output()?
        .stdout;

    output.pop(); // drop '\n'

    let output = std::str::from_utf8(&output)?;
    let output = output.parse::<i32>()?;

    Ok(output)
}

fn env_lines() -> Result<i32, ScreenSizeError> {
    spawn_and_parse_output_as_integer("tput", "lines")
}

fn env_cols() -> Result<i32, ScreenSizeError> {
    spawn_and_parse_output_as_integer("tput", "cols")
}

impl TerminalScreenSize {
    pub fn update(&mut self) -> Result<(), ScreenSizeError> {
        let lines = env_lines()?;
        let cols = env_cols()?;
        self.lines = lines;
        self.cols = cols;
        Ok(())
    }

    pub fn new() -> Result<Self, ScreenSizeError> {
        let mut result = Self { lines: 0, cols: 0 };
        result.update()?;
        Ok(result)
    }
}
