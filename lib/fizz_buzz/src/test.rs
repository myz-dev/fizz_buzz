use pretty_assertions::assert_eq;

use crate::{
    error::FizzBuzzError, presets::ConsecutiveTokens, FormattingOptions, Numeric, Tokenizer,
    Traditional,
};

/// Function for ergonomic test set up.
/// Creates a [`Tokenizer`] that conforms the FizzBuzz++ rules.
fn create_default_tokenizer(f: u32, b: u32) -> (Tokenizer, FormattingOptions) {
    let options = crate::FormattingOptions {
        separator: Some("\n".to_string()),
        case: None,
    };

    let fall_back = Box::new(Numeric);
    let fizz = Box::new(ConsecutiveTokens::new("Fizz", "+", 1, f, vec![b]).unwrap());
    let buzz = Box::new(ConsecutiveTokens::new("Buzz", "+", 1, b, vec![f]).unwrap());
    let fizz_buzz = Box::new(Traditional::new("FizzBuzz", 2, vec![f, b]).unwrap());

    let tokenizer = Tokenizer::new(vec![fall_back, fizz, buzz, fizz_buzz]);
    (tokenizer, options)
}

#[test]
fn test_fizz_butt_output() {
    // ------------------------- Passing zero ==> Error ------------------------- //
    let rule_err = Traditional::new("Fizz", 1, vec![0]).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, rule_err);

    let rule_err = ConsecutiveTokens::new("Fizz", "+", 1, 0, vec![1, 2]).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, rule_err);

    let rule_err = ConsecutiveTokens::new("Fizz", "+", 1, 4, vec![1, 0]).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, rule_err);
    // -------------------------        Example run     ------------------------- //
    let f = 2;
    let b = 7;
    let t = 20;
    let expected = r#"1
Fizz
3
Fizz+
5
Fizz++
Buzz
Fizz
9
Fizz+
11
Fizz++
13
FizzBuzz
15
Fizz
17
Fizz+
19
Fizz++"#;

    let (tokenizer, options) = create_default_tokenizer(f, b);
    let output = tokenizer.produce_output(t, options).unwrap();
    assert_eq!(&output, expected);

    // ----------------------  Run with + on second rule  ----------------------- //
    let f = 5;
    let b = 3;
    let t = 15;
    let expected = r#"1
2
Buzz
4
Fizz
Buzz
7
8
Buzz+
Fizz
11
Buzz
13
14
FizzBuzz"#;
    let (tokenizer, options) = create_default_tokenizer(f, b);
    let output = tokenizer.produce_output(t, options).unwrap();
    assert_eq!(&output, expected);

    // -------------------------   f equals b   ------------------------- //
    let f = 2;
    let b = 2;
    let t = 10;
    let expected = r#"1
FizzBuzz
3
FizzBuzz
5
FizzBuzz
7
FizzBuzz
9
FizzBuzz"#;

    let (tokenizer, options) = create_default_tokenizer(f, b);
    let output = tokenizer.produce_output(t, options).unwrap();
    assert_eq!(&output, expected);
}

// #[test]
// fn test_three_tokens() {
//     let f = 2;
//     let b = 9;
//     let m = 5;
//     let t = 15;

//     let options = crate::FormattingOptions {
//         separator: Some("\n".to_string()),
//         case: None,
//     };

//     let fall_back = Box::new(Numeric);
//     let fizz = Box::new(ConsecutiveTokens::new("Fizz", "+", 1, f, vec![b]).unwrap());
//     let buzz = Box::new(ConsecutiveTokens::new("Buzz", "+", 1, b, vec![f]).unwrap());
//     let buzz = Box::new(ConsecutiveTokens::new("Mezz", "+", 1, b, vec![f]).unwrap());
//     let fizz_buzz = Box::new(Traditional::new("FizzBuzz", 2, vec![f, b]).unwrap());

//     let tokenizer = Tokenizer::new(vec![fall_back, fizz, buzz, fizz_buzz]);
// }
