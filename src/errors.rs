use std::num::ParseIntError;

// #[derive(Clone)]
pub struct PuzzleError {
}

impl From<ParseIntError> for PuzzleError {
    fn from(_: ParseIntError) -> Self {
        PuzzleError { }
    }
}

// impl FromIterator<Result<String, std::io::Error>> for PuzzleError {
//     fn from_iter<T>(iter: T) -> Self 
//     where T: IntoIterator, std::iter::IntoIterator::Item = A {
//         for i in iter {

//         }
//     }
// }