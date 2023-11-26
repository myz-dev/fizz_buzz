use crate::args::Config;
use fizz_buzz::{
    error::Result, ConsecutiveTokens, FormattingOptions, Numeric, Tokenizer, Traditional,
};

pub fn play_traditional(cfg: Config) -> Result<String> {
    let options = FormattingOptions {
        separator: Some("\n".to_string()),
        case: None,
    };
    let Config { t, f, b } = cfg;
    let fall_back = Box::new(Numeric);
    let fizz = Box::new(ConsecutiveTokens::new("Fizz", "+", 1, f, vec![b])?);
    let buzz = Box::new(ConsecutiveTokens::new("Buzz", "+", 1, b, vec![f])?);
    let fizz_buzz = Box::new(Traditional::new("FizzBuzz", 2, vec![f, b])?);

    let tokenizer = Tokenizer::new(vec![fall_back, fizz, buzz, fizz_buzz]);
    tokenizer.produce_output(t, options)
}
