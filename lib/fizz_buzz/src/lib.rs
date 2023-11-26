pub mod error;

mod formatting;
mod presets;
mod token_condition;
mod tokenizer;

// API:
pub use formatting::{Case, FormattingOptions};
pub use presets::{ConsecutiveTokens, Numeric, Traditional};
pub use token_condition::TokenCondition;
pub use tokenizer::Tokenizer;

#[cfg(test)]
mod test;
