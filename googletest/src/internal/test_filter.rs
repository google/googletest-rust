//! Implements the googletest test filtering protocol.
//!
//! The Google test filtering protocol consists of the following
//! environment variable:
//!
//! * TESTBRIDGE_TEST_ONLY: string passed from --test_filter
//!
//! We interpret it as a colon-separated list of glob patterns.  If
//! any pattern in the list succeeds, the filter passes.
use crate::internal::glob::{is_glob_pattern, Pattern};
use std::sync::OnceLock;

static TEST_FILTER: OnceLock<Box<dyn TestFilter + Send + Sync>> = OnceLock::new();

pub fn test_should_run(test_name: &str) -> bool {
    let test_filter = TEST_FILTER.get_or_init(|| {
        if let Ok(testbridge_test_only) = std::env::var("TESTBRIDGE_TEST_ONLY") {
            Box::new(get_test_filter(&testbridge_test_only))
        } else {
            Box::new(AcceptAll)
        }
    });

    test_filter.filter(test_name)
}

trait TestFilter {
    /// Returns true if the test should run.
    fn filter(&self, test_name: &str) -> bool;
}

struct AcceptAll;
impl TestFilter for AcceptAll {
    fn filter(&self, _test_name: &str) -> bool {
        true
    }
}

struct Equals(String);
impl TestFilter for Equals {
    fn filter(&self, test_name: &str) -> bool {
        test_name == self.0
    }
}

struct Matches(Pattern);
impl TestFilter for Matches {
    fn filter(&self, test_name: &str) -> bool {
        self.0.matches(test_name)
    }
}

struct Collection {
    equals: Box<[Equals]>,
    matches: Box<[Matches]>,
}

impl TestFilter for Collection {
    fn filter(&self, test_name: &str) -> bool {
        self.equals.iter().any(|f| f.filter(test_name))
            || self.matches.iter().any(|f| f.filter(test_name))
    }
}

fn get_test_filter(testbridge_test_only: &str) -> Collection {
    let (with_globs, literals): (Vec<_>, Vec<_>) =
        testbridge_test_only.split(':').partition(|s| is_glob_pattern(s));
    Collection {
        equals: literals.into_iter().map(|s| Equals(s.to_string())).collect(),
        matches: with_globs.into_iter().map(|s| Matches(Pattern::new(s.to_string()))).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn accept_all_accepts_all() -> Result<()> {
        let filter = AcceptAll;

        verify_that!(filter.filter(""), is_true())?;
        verify_that!(filter.filter("abcdefg"), is_true())?;
        Ok(())
    }

    #[test]
    fn empty_filter_accepts_only_empty() -> Result<()> {
        let filter = get_test_filter("");

        verify_that!(filter.filter(""), is_true())?;
        verify_that!(filter.filter("abcdefg"), is_false())?;
        Ok(())
    }

    #[test]
    fn simple_literal_filter() -> Result<()> {
        let filter = get_test_filter("*magic*");

        verify_that!(filter.filter("this_is_magic"), is_true())?;
        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("magic"), is_true())?;
        verify_that!(filter.filter("science"), is_false())?;
        Ok(())
    }

    #[test]
    fn star_globs() -> Result<()> {
        let filter = get_test_filter("a*b");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("a"), is_false())?;
        verify_that!(filter.filter("ab"), is_true())?;
        verify_that!(filter.filter("a b"), is_true())?;
        verify_that!(filter.filter("b"), is_false())?;
        verify_that!(filter.filter("b a"), is_false())?;
        Ok(())
    }

    #[test]
    fn question_globs() -> Result<()> {
        let filter = get_test_filter("a?b");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("a"), is_false())?;
        verify_that!(filter.filter("ab"), is_false())?;
        verify_that!(filter.filter("aXb"), is_true())?;
        verify_that!(filter.filter("a b"), is_true())?;
        verify_that!(filter.filter("b"), is_false())?;
        verify_that!(filter.filter("b a"), is_false())?;
        Ok(())
    }

    #[test]
    fn collection() -> Result<()> {
        let filter = get_test_filter("a:b");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("a"), is_true())?;
        verify_that!(filter.filter("ab"), is_false())?;
        verify_that!(filter.filter("a b"), is_false())?;
        verify_that!(filter.filter("b"), is_true())?;
        verify_that!(filter.filter("b a"), is_false())?;
        verify_that!(filter.filter("c"), is_false())?;
        Ok(())
    }

    #[test]
    fn collection_with_globs() -> Result<()> {
        let filter = get_test_filter("*test1*:*test2*");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("this is test1"), is_true())?;
        verify_that!(filter.filter("and test2 is it"), is_true())?;
        verify_that!(filter.filter("but test3 is not"), is_false())?;
        Ok(())
    }
}
