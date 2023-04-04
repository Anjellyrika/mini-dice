pub enum State {
    Initial,
    Selection { die_size: u32 },
    Result
}

impl Default for State {
    fn default() -> Self {
        Self::Initial
    }
}