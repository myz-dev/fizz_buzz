pub trait TokenCondition {
    // Could be optimized to return a set of instructions
    // on how to construct the wished token, instead of
    // allocating a `String` for it.
    fn tokenize(&self, i: u32) -> String;

    fn condition(&self, i: u32) -> bool;

    fn get_priority(&self) -> u32;
}
#[cfg(test)]
mod test {
    use crate::token_condition::TokenCondition;

    #[test]
    fn test_trait() {
        struct Test {
            token: &'static str,
            divisor: u32,
            priority: u32,
        }

        impl Test {
            pub fn new(token: &'static str, divisor: u32, priority: u32) -> Self {
                Self {
                    token,
                    divisor,
                    priority,
                }
            }
        }
        impl TokenCondition for Test {
            fn tokenize(&self, i: u32) -> String {
                format!("[{:03}] testing {}", i, self.token)
            }

            fn condition(&self, i: u32) -> bool {
                i % self.divisor == 0
            }

            fn get_priority(&self) -> u32 {
                self.priority
            }
        }

        let fizz = Test::new("Fizz", 2, 1);
        let buzz = Test::new("Buzz", 3, 1);

        assert!(fizz.condition(2));
        assert!(fizz.condition(4));
        assert!(fizz.condition(36));

        assert!(buzz.condition(3));
        assert!(buzz.condition(6));
        assert!(buzz.condition(36));

        assert_eq!(&fizz.tokenize(2), "[002] testing Fizz");
    }
}
