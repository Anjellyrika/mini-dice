#[derive(Debug, Default, PartialEq)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    #[default]
    D20,
    D100,
}