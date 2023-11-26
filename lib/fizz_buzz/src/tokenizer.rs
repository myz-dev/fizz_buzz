use crate::error::Result;
use crate::formatting::FormattingOptions;
use crate::token_condition::TokenCondition;

pub struct Tokenizer {
    /// Vector of pointers to trait objects
    configured_tokens: Vec<Box<dyn TokenCondition>>,
}

impl Tokenizer {
    pub fn new(tokens: Vec<Box<dyn TokenCondition>>) -> Self {
        Self {
            configured_tokens: tokens,
        }
    }

    pub fn produce_output(&self, t: u32, options: FormattingOptions) -> Result<String> {
        let mut tokens = Vec::new();
        for i in 1..=t {
            let maybe_token = self
                .configured_tokens
                .iter()
                .filter(|c| c.condition(i))
                .max_by_key(|c| c.get_priority());

            if let Some(token) = maybe_token {
                let mut token = token.tokenize(i);
                options.apply_formatting(&mut token, i, t);
                tokens.push(token);
            }
            // unlike the original FizzBuzz game, this set up allows for iterations
            // that do not produce any output. If this should be disallowed in the
            // future, an error should be thrown here.
        }
        Ok(tokens.join(&options.separator.unwrap_or_default()))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatting::Case,
        presets::{Numeric, Traditional},
        tokenizer::{FormattingOptions, Tokenizer},
    };
    use pretty_assertions::assert_eq;

    /// This test mainly assures API changes are caught by it.
    /// The integration test in `test.rs` tests the complete
    /// functionality of the [`Tokenizer`].
    #[test]
    fn test_tokenizer() {
        let value = Box::new(Numeric);
        let fizz = Box::new(Traditional::new("Fizz", 1, vec![2]).unwrap());
        let buzz = Box::new(Traditional::new("Buzz", 1, vec![3]).unwrap());
        let fizz_buzz = Box::new(Traditional::new("FizzBuzz", 2, vec![2, 3]).unwrap());

        let tokenizer = Tokenizer::new(vec![fizz, buzz, fizz_buzz, value]);

        let options = FormattingOptions {
            separator: Some("\n".to_string()),
            case: Some(Case::Lower),
        };
        let output = tokenizer.produce_output(6, options).unwrap();
        let expected = r#"1
fizz
buzz
fizz
5
fizzbuzz"#;

        assert_eq!(&output, expected);
    }
}
