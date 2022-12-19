#[derive(Debug)]
struct AStruct {
    field1: i32,
    field2: String,
    field3: String,
}

#[cfg(test)]
mod tests {
    #[cfg(not(google3))]
    use crate as googletest;
    use crate::AStruct;
    use googletest::matcher::*;
    #[cfg(not(google3))]
    use googletest::matchers;
    use googletest::{google_test, verify_that, Result};
    use matchers::{anything, contains_substring, eq, lt, not, Anything};

    // Can avoid dynamic dispatch with https://rust-lang.github.io/rfcs/2528-type-changing-struct-update-syntax.html
    struct AStructMatcher<'a> {
        field1: &'a dyn Matcher<i32>,
        field2: &'a dyn Matcher<String>,
    }

    // Macro generated code
    impl<'a> Matcher<AStruct> for AStructMatcher<'a> {
        fn matches(&self, actual: &AStruct) -> MatcherResult {
            match (self.field1.matches(&actual.field1), self.field2.matches(&actual.field2)) {
                (MatcherResult::Matches, MatcherResult::Matches) => MatcherResult::Matches,
                _ => MatcherResult::DoesNotMatch,
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> String {
            "".to_string()
        }
    }

    pub const STATIC_ANYTHING: Anything = Anything {};

    impl<'a> Default for AStructMatcher<'a> {
        fn default() -> Self {
            Self { field1: &STATIC_ANYTHING, field2: &STATIC_ANYTHING }
        }
    }
    // Generated until here.

    #[google_test]
    fn check() -> Result<()> {
        verify_that!(
            AStruct { field1: 12, field2: "toto".to_string(), field3: "tata".to_string() },
            AStructMatcher { field1: &eq(12), ..Default::default() }
        )
    }
}
