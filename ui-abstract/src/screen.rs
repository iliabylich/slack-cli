use crate::IoResult;

pub trait Screen {
    fn draw(&mut self) -> IoResult;

    fn clear(&mut self) -> IoResult;

    fn redraw(&mut self) -> IoResult {
        self.clear()?;
        self.draw()
    }
}
