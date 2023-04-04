pub enum State {
    Selection,
    Result,
}

impl Default for State {
    fn default() -> Self {
        Self::Selection
    }
}
