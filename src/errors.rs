use std::num::ParseIntError;

pub struct PuzzleError {
}

impl From<ParseIntError> for PuzzleError {
    fn from(_: ParseIntError) -> Self {
        PuzzleError { }
    }
}