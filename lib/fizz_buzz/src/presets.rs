//! This module contains a few preset token validation and creation
//! rules, that can be used to run the `FizzBuzz` game. They are meant
//! to serve as examples on how to use the API so that users can create
//! their own variants.

use crate::token_condition::TokenCondition;

/// The traditional token creation rule, that produces the defined `token`,
/// when all its `divisors` members divide the current iteration of the
/// `FizzBuzz` game cleanly.
/// This rule can be used to produce output like "Fizz" when only one
/// divisor is passed. It can also produce output like "FizzBuzz" when
/// multiple divisors are passed withing the `divisors` vector.
pub struct Traditional {
    token: &'static str,
    priority: u32,
    divisors: Vec<u32>,
}

impl Traditional {
    pub fn new(token: &'static str, priority: u32, divisors: Vec<u32>) -> Self {
        Self {
            token,
            priority,
            divisors,
        }
    }
}

impl TokenCondition for Traditional {
    fn tokenize(&self, _i: u32) -> String {
        self.token.to_string()
    }

    fn condition(&self, i: u32) -> bool {
        self.divisors.iter().all(|d| i % d == 0)
    }

    fn get_priority(&self) -> u32 {
        self.priority
    }
}

/// This token creation rule is generally used as a fallback rule in the `FizzBuzz` game.
/// It has the smallest possible priority and its print condition always evaluates to true.
/// Its token is the number of the current iteration converted into a `String`.
pub struct Numeric;
impl TokenCondition for Numeric {
    fn tokenize(&self, i: u32) -> String {
        i.to_string()
    }

    fn condition(&self, _i: u32) -> bool {
        true
    }

    fn get_priority(&self) -> u32 {
        0
    }
}
