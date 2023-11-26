//! This module contains a few preset token validation and creation
//! rules, that can be used to run the `FizzBuzz` game. They are meant
//! to serve as examples on how to use the API so that users can create
//! their own variants.

use crate::error::{FizzBuzzError, Result};
use crate::token_condition::TokenCondition;

/// The traditional token creation rule, that produces the defined `token`,
/// when all its `divisors` members divide the current iteration of the
/// `FizzBuzz` game cleanly.
/// This rule can be used to produce output like "Fizz" when only one
/// divisor is passed. It can also produce output like "FizzBuzz" when
/// multiple divisors are passed withing the `divisors` vector.
#[derive(Debug)]
pub struct Traditional {
    token: &'static str,
    priority: u32,
    divisors: Vec<u32>,
}

impl Traditional {
    pub fn new(token: &'static str, priority: u32, divisors: Vec<u32>) -> Result<Self> {
        if divisors.contains(&0) {
            return Err(FizzBuzzError::NonZeroValue);
        }
        Ok(Self {
            token,
            priority,
            divisors,
        })
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

/// This rule preset can be used to produce output that is akin to the
/// traditional `FizzBuzz` output, but does one extra sep of evaluations:
///
/// The exact formatting of the resulting output during any point of
/// the game depends on prior iterations.
/// If none of the `rivals` have divided the current iteration value cleanly
/// prior to another clean division by the defined `divisor`, the `suffix` is
/// appended to the defined `token`.
/// The `suffix` is applied as many times as prior clean divisions by `divisor`
/// have occurred, without being interrupted by one of its `rivals`.
#[derive(Debug)]
pub struct ConsecutiveTokens {
    token: &'static str,
    priority: u32,
    divisor: u32,
    rivals: Vec<u32>,
    suffix: &'static str,
}

impl ConsecutiveTokens {
    pub fn new(
        token: &'static str,
        suffix: &'static str,
        priority: u32,
        divisor: u32,
        rivals: Vec<u32>,
    ) -> Result<Self> {
        if rivals.contains(&0) || divisor == 0 {
            return Err(FizzBuzzError::NonZeroValue);
        }

        Ok(Self {
            token,
            priority,
            divisor,
            rivals,
            suffix,
        })
    }

    /// Calculates the number of clean divisions within 1..=i of `divisor`, up to the last
    /// clean division of the range by any member of `rivals`.
    /// Returns `0`, if the current iteration `i` can not be cleanly divided by `divisor`.
    ///
    /// I.e. for `i = 8`, `divisor = 2` and `rivals = &[5]`:
    ///     -   prior to 8, which is cleanly divided by `divisor`, `divisor` also divides
    ///         6 cleanly. As between 8 and 6 no `rival` divides any number cleanly, the number
    ///         of uninterrupted clean divisions is 2 (one at 8, the other one at 6).
    ///         The next clean division at 4 is interrupted by the rival's clean division of 5.
    ///
    /// # Panics
    /// This function assumes nor `divisor` neither any member of `rivals`
    /// is equal to zero and panics, if this it not the case.
    fn calculate_uninterrupted_divisions(i: u32, divisor: u32, rivals: &[u32]) -> usize {
        if i % divisor != 0 {
            return 0;
        }

        // first clean division
        if divisor == i {
            return 1;
        }

        // no rivals
        if rivals.is_empty() {
            return (i / divisor) as usize;
        }

        // If the smallest rival is already bigger than the current iteration, we can skip the check.
        if i < *rivals.iter().min().unwrap_or(&0) {
            let total_divisions = i / divisor;
            return total_divisions as usize; // save conversion on targets with at least 32 bit arch.
        }

        let mut uninterrupted_clean_division: Option<u32> = None;

        for r in rivals {
            let mut last_clean_divide_by_rival = None;
            // Check backwards for clean divisions
            for iteration in (1..=i).rev() {
                if iteration % r == 0 {
                    last_clean_divide_by_rival = Some(iteration);
                    break;
                }
            }

            // if there was a prior clean division by a rival, the number of uninterrupted
            // clean divisions by `divisor` is the number of times `divisor` fits between
            // the rivals last clean division and `i` plus one.
            if let Some(rival_division) = last_clean_divide_by_rival {
                let delta = i - rival_division;

                // check for the "FizzBuzz" case, where the last clean division by a rival is also
                // a clean division by divisor. This is considered an interrupt.
                let fizz_buzz = delta % divisor == 0;

                uninterrupted_clean_division = if let Some(v) = uninterrupted_clean_division {
                    Some(v.min((delta / divisor) + !fizz_buzz as u32))
                } else {
                    Some((delta / divisor) + !fizz_buzz as u32)
                };
            }
        }
        uninterrupted_clean_division.unwrap_or(0) as usize
    }
}

impl TokenCondition for ConsecutiveTokens {
    fn tokenize(&self, i: u32) -> String {
        let pluses = Self::calculate_uninterrupted_divisions(i, self.divisor, &self.rivals);
        let pulses = pluses.saturating_sub(1); // first occurrence without suffix!
        format!("{}{}", self.token, self.suffix.repeat(pulses))
    }

    fn condition(&self, i: u32) -> bool {
        Self::calculate_uninterrupted_divisions(i, self.divisor, &self.rivals) > 0
    }

    fn get_priority(&self) -> u32 {
        self.priority
    }
}

#[cfg(test)]
mod test {
    use super::ConsecutiveTokens;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_uninterrupted_divisions() {
        // ------------------------- Divisor does not divide `i` ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(7, 2, &vec![7]);
        let expected = 0;
        assert_eq!(res, expected);

        // --------------------- No prior rival clean divisions  ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(6, 2, &vec![7]);
        let expected = 3;
        assert_eq!(res, expected);

        // ---------------------  Interrupted directly before   ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(8, 2, &vec![3]);
        let expected = 1;
        assert_eq!(res, expected);

        // ---------------------   Rival smaller than divisor   ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(10, 5, &vec![3]);
        let expected = 1;
        assert_eq!(res, expected);

        // ---------------------  Iteration before any division ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(3, 5, &vec![6]);
        let expected = 0;
        assert_eq!(res, expected);

        // ---------------------   Check at FizzBuzz condition  ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(6, 2, &vec![3]);
        let expected = 0;
        assert_eq!(res, expected);

        // --------------------- Second rival interrupts first  ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(8, 2, &vec![9, 5]);
        let expected = 2;
        assert_eq!(res, expected);

        // ---------------------     FizzBuzz second rival      ------------------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(10, 2, &vec![9, 5]);
        let expected = 0;
        assert_eq!(res, expected);

        // --------------------- FizzBuzz interrupt second rival ------------------------ //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(24, 2, &vec![9, 5]);
        let expected = 2;
        assert_eq!(res, expected);

        // ---------------------           No rivals             ------------------------ //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(24, 2, &vec![]);
        let expected = 12;
        assert_eq!(res, expected);

        // --------------          No division at checked iteration        -------------- //
        let res = ConsecutiveTokens::calculate_uninterrupted_divisions(25, 2, &vec![]);
        let expected = 0;
        assert_eq!(res, expected);
    }
}
