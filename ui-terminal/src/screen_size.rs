use terminal_size::{Width, Height, terminal_size};

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

impl TerminalScreenSize {
    pub fn update(&mut self) -> Result<(), ScreenSizeError> {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            self.lines = h as i32;
            self.cols = w as i32;
            Ok(())
        } else {
            Err(ScreenSizeError { message: String::from("Failed to get screen size") })
        }
    }

    pub fn new() -> Result<Self, ScreenSizeError> {
        let mut result = Self { lines: 0, cols: 0 };
        result.update()?;
        Ok(result)
    }
}
