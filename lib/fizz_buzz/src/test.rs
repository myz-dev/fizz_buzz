use crate::token_rules::RawToken;
use crate::{
    error::FizzBuzzError,
    fizz_buzz,
    token_rules::{Condition, Rule},
};
use pretty_assertions::assert_eq;
#[test]
fn test_fizz_butt_output() {
    // ------------------------- Passing zero ==> Error ------------------------- //
    let rules = Rule::default_rule_set(1, 2);
    let err = fizz_buzz(0, rules).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, err);

    let zero_rule_err = Rule::new(vec![Condition::Divisor(0)], RawToken::Buzz, 1).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, zero_rule_err);

    // ------------   Constructing invalid `Rule` ==> Error  -------------------- //
    let conditions = vec![
        Condition::Consecutive {
            divisor: 1,
            rivals: vec![2, 3],
        },
        Condition::Consecutive {
            divisor: 4,
            rivals: vec![1, 6],
        },
    ];

    let err = Rule::new(conditions, RawToken::Buzz, 1).unwrap_err();
    match err {
        FizzBuzzError::InvalidRuleConfiguration(_) => (),
        _ => panic!("Wrong error type!"),
    }

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

    let rules = Rule::default_rule_set(f, b);
    let res = fizz_buzz(t, rules).unwrap();
    assert_eq!(&res, expected);

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

    let rules = Rule::default_rule_set(f, b);
    let res = fizz_buzz(t, rules).unwrap();
    assert_eq!(&res, expected);

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

    let rules = Rule::default_rule_set(f, b);
    let res = fizz_buzz(t, rules).unwrap();
    assert_eq!(&res, expected);
}
