#[derive(Debug)]
pub enum State {
    Selection,
    Result,
    Reset,
}

impl Default for State {
    fn default() -> Self {
        Self::Selection
    }
}
