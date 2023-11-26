use pretty_assertions::assert_eq;

use crate::{
    error::FizzBuzzError, presets::ConsecutiveTokens, FormattingOptions, Numeric, TokenCondition,
    Tokenizer, Traditional,
};

/// Function for ergonomic test set up.
/// Creates a [`Tokenizer`] that conforms to the FizzBuzz++ rules.
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

fn create_triple_tokenizer(f: u32, b: u32, m: u32) -> (Tokenizer, FormattingOptions) {
    let options = crate::FormattingOptions {
        separator: Some("\n".to_string()),
        case: None,
    };

    let fall_back = Box::new(Numeric);
    let fizz = Box::new(ConsecutiveTokens::new("Fizz", "+", 1, f, vec![b, m]).unwrap());
    let buzz = Box::new(ConsecutiveTokens::new("Buzz", "+", 1, b, vec![f, m]).unwrap());
    let mezz = Box::new(ConsecutiveTokens::new("Mezz", "+", 1, m, vec![f, b]).unwrap());
    let fizz_buzz = Box::new(Traditional::new("FizzBuzz", 2, vec![f, b]).unwrap());
    let mezz_buzz = Box::new(Traditional::new("MezzBuzz", 2, vec![m, b]).unwrap());
    let fizz_mezz = Box::new(Traditional::new("FizzMezz", 2, vec![f, m]).unwrap());
    let fizz_buzz_mezz = Box::new(Traditional::new("FizzBuzzMezz", 3, vec![f, b, m]).unwrap());

    let tokenizer = Tokenizer::new(vec![
        fall_back,
        fizz,
        buzz,
        mezz,
        fizz_buzz,
        mezz_buzz,
        fizz_mezz,
        fizz_buzz_mezz,
    ]);
    (tokenizer, options)
}

#[test]
fn test_three_tokens() {
    // -------------------------   f produces plusses   ------------------------- //

    let f = 2;
    let b = 9;
    let m = 5;
    let t = 18;

    let (tokenizer, options) = create_triple_tokenizer(f, b, m);

    let output = tokenizer.produce_output(t, options).unwrap();
    let expected = r#"1
Fizz
3
Fizz+
Mezz
Fizz
7
Fizz+
Buzz
FizzMezz
11
Fizz
13
Fizz+
Mezz
Fizz
17
FizzBuzz"#;
    assert_eq!(&output, expected);

    // -------------------------   FizzBuzzMezz   ------------------------- //
    let f = 2;
    let b = 3;
    let m = 5;
    let t = 30;

    let (tokenizer, options) = create_triple_tokenizer(f, b, m);

    let output = tokenizer.produce_output(t, options).unwrap();
    let expected = r#"1
Fizz
Buzz
Fizz
Mezz
FizzBuzz
7
Fizz
Buzz
FizzMezz
11
FizzBuzz
13
Fizz
MezzBuzz
Fizz
17
FizzBuzz
19
FizzMezz
Buzz
Fizz
23
FizzBuzz
Mezz
Fizz
Buzz
Fizz
29
FizzBuzzMezz"#;

    assert_eq!(&output, expected);
}

#[test]
fn add_custom_evaluation() {
    struct MyFallBack;
    impl TokenCondition for MyFallBack {
        fn tokenize(&self, i: u32) -> String {
            let s = i.to_string();
            let jap = s
                .chars()
                .map(|c| match c {
                    '0' => '0',
                    '1' => '一',
                    '2' => '二',
                    '3' => '三',
                    '4' => '四',
                    '5' => '五',
                    '6' => '六',
                    '7' => '七',
                    '8' => '八',
                    '9' => '九',
                    _ => '?',
                })
                .collect();
            jap
        }

        fn condition(&self, _i: u32) -> bool {
            true
        }

        fn get_priority(&self) -> u32 {
            0
        }
    }

    let f = 2;
    let b = 10;
    let t = 16;

    let options = crate::FormattingOptions {
        separator: Some("\n".to_string()),
        case: None,
    };

    let fall_back = Box::new(MyFallBack);

    let fizz = Box::new(ConsecutiveTokens::new("Fizz", "+", 1, f, vec![b]).unwrap());
    let buzz = Box::new(ConsecutiveTokens::new("Buzz", "+", 1, b, vec![f]).unwrap());
    let fizz_buzz = Box::new(Traditional::new("FizzBuzz", 2, vec![f, b]).unwrap());

    let tokenizer = Tokenizer::new(vec![fall_back, fizz, buzz, fizz_buzz]);

    let output = tokenizer.produce_output(t, options).unwrap();
    let expected = r#"一
Fizz
三
Fizz+
五
Fizz++
七
Fizz+++
九
FizzBuzz
一一
Fizz
一三
Fizz+
一五
Fizz++"#;
    assert_eq!(&output, expected);
}
