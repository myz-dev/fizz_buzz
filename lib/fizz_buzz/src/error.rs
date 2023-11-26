pub type Result<T> = std::result::Result<T, FizzBuzzError>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FizzBuzzError {
    #[error("Passed zero! Only `natural numbers` are allowed.")]
    NonZeroValue,
    #[error("The rule configuration is invalid! {}.",.0)]
    InvalidRuleConfiguration(String),
    #[error("The token tokenization rule does not comply with the provided conditions! {}.",.0)]
    InvalidTokenConfiguration(String),
}
