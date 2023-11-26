use crate::{
    error::FizzBuzzError,
    fizz_buzz,
    token_rules::{Condition, Rule},
};
use pretty_assertions::assert_eq;
#[test]
fn test_fizz_butt_output() {
    // ------------------------- Passing zero ==> Error ------------------------- //
    let err = fizz_buzz(0, 1, 2, Vec::new()).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, err);

    let err = fizz_buzz(2, 0, 2, Vec::new()).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, err);

    let err = fizz_buzz(3, 1, 0, Vec::new()).unwrap_err();
    assert_eq!(FizzBuzzError::NonZeroValue, err);

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

    let err = Rule::new(conditions, crate::token_rules::RawToken::Buzz, 1).unwrap_err();
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
    let res = fizz_buzz(t, f, b, rules).unwrap();
    assert_eq!(&res, expected);
}
