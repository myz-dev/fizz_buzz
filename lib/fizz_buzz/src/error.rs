pub type Result<T> = std::result::Result<T, FizzBuzzError>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FizzBuzzError {
    #[error(
        "Passed zero! FizzBuzz only operates on `natural numbers` (integers bigger than zero)."
    )]
    NonZeroValue,
}
