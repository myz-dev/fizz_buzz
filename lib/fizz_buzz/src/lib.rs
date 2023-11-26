pub mod error;
mod fizzer;
mod token_rules;

mod token_condition;
mod tokenizer;
mod formatting;
mod presets;

pub use fizzer::fizz_buzz;
pub use token_rules::Rule;

#[cfg(test)]
mod test;
