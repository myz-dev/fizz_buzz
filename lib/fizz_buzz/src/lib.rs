pub mod error;
mod fizzer;
mod token_rules;

pub use fizzer::fizz_buzz;
pub use token_rules::Rule;

#[cfg(test)]
mod test;
