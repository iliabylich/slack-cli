#[derive(Debug)]
pub enum Size {
    Absolute(i32),
    Percentage(i32)
}

impl Size {
    pub fn on_screen(&self, screen_size: i32) -> i32 {
        match self {
            Self::Absolute(value) => *value,
            Self::Percentage(value) => {
                ((screen_size as f32) / 100.0 * (*value as f32)).round() as i32
            }
        }
    }
}
