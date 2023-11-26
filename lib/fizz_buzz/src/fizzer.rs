use crate::{
    error::{FizzBuzzError, Result},
    token_rules::{Rule, Token},
};

/// Loops through the range `1..=t` and returns a `String`, that conforms to
/// to the passed set of `rules`.
///
/// E.g. if these rules are:
/// Add a new line containing `t`, if none of the following criteria are met:
///  -  if t % f == 0                        ==> add a new line containing `Fizz`
///  -  if t % b == 0                        ==> add a new line containing `Buzz`
///  -  if (t % f == 0) && (t % b == 0)      == > add a new line containing `FizzBuzz`
///  -  if during the loop, the condition `t % f == 0` is met repeatedly, without the
///     condition for `Buzz` is met          ==> Fizz++
///     For each additional `t % f == 0` a `plus` is appended to the string.
///     Same goes for repeatedly meeting `t % b == 0`
/// Where
/// * `t` - Number of iterations.
/// * `f` - Rule to trigger `Fizz` outputs.
/// * `b` - Rule to trigger `Buzz` outputs.
///
/// # Error
/// Returns an [`FizzBuzzError`] if any of the passed arguments equal to zero.
pub fn fizz_buzz(t: u32, rules: Vec<Rule>) -> Result<String> {
    if t == 0 {
        return Err(FizzBuzzError::NonZeroValue);
    }
    Rule::validate_rule_set(&rules)?;

    let mut tokenizer = Tokenizer::new(rules);
    let mut tokens = Vec::new();
    for i in 1..=t {
        tokens.push(tokenizer.create_next_token(i)?);
    }

    Ok(Tokenizer::construct_output(&tokens))
}

struct Tokenizer {
    rules: Vec<Rule>,
}

impl Tokenizer {
    /// Create new `Tokenizer` object.
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    /// Validate all rules and create the appropriate [`Token`] for the iteration `i`.
    fn create_next_token(&mut self, i: u32) -> Result<Token> {
        let mut token_candidates: Vec<(Token, u32)> = Vec::new();
        for r in self.rules.iter() {
            let res = r.try_tokenize(i)?;
            if let Some(token) = res {
                token_candidates.push((token, r.priority));
            }
        }

        // if there are multiple tokens registered with the same priority,
        // this will get the last token with the highest priority.
        let highest_priority = token_candidates
            .iter()
            .max_by_key(|t| t.1)
            .map(|t| t.0)
            .unwrap_or(Token::Value { v: i });
        Ok(highest_priority)
    }

    /// Constructs the output string from the list of all `tokens`.
    /// Any future formatting changes to how the tokens are combined,
    /// should be made here.
    fn construct_output(tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|t| format!("{t}\n"))
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}
