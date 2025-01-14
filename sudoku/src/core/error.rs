#[derive(Debug)]
pub enum SudokuError {
    OutOfBounds,
    InvalidValue,
}
