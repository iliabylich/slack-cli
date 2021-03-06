use crate::Point;

#[derive(Debug, Clone)]
pub enum AtomicAction {
    ClearScreen,
    MoveAt { point: Point },
    MoveUp { n: i32 },
    MoveDown { n: i32 },
    MoveLeft { n: i32 },
    MoveRight { n: i32 },
    Print { char: char },
    SaveCursor,
    RestoreCursor,
}
