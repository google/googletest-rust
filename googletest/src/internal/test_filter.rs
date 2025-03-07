//! Implements the googletest test filtering protocol.
//!
//! The Google test filtering protocol consists of the following
//! environment variable:
//!
//! * TESTBRIDGE_TEST_ONLY: string passed from --test_filter
//!
//! The format of a filter is a ‘,‘-separated list of wildcard
//! patterns (called the positive patterns) optionally followed by a
//! ‘-’ and another ‘,‘-separated pattern list (called the negative
//! patterns). A test matches the filter if and only if it matches any
//! of the positive patterns but does not match any of the negative
//! patterns.  (Note that this is a deliberate devation from GTest
//! C++, which uses colons to separate the patterns, as colons will
//! unfortunately clash with Rust's "::" namespacing operator.)
//!
//! As an example: "*mount*-*doom*" will accept any string that contains the
//! substring "mount", as long as it also doesn't contain "doom"
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
    // The positive portion:
    positive_equals: Box<[Equals]>,
    positive_matches: Box<[Matches]>,

    // The negative portion:
    negative_equals: Box<[Equals]>,
    negative_matches: Box<[Matches]>,
}

impl TestFilter for Collection {
    fn filter(&self, test_name: &str) -> bool {
        (self.positive_equals.iter().any(|f| f.filter(test_name))
            || self.positive_matches.iter().any(|f| f.filter(test_name)))
            && (!self.negative_equals.iter().any(|f| f.filter(test_name)))
            && (!self.negative_matches.iter().any(|f| f.filter(test_name)))
    }
}

fn get_test_filter(testbridge_test_only: &str) -> Collection {
    let positive_negative: Vec<&str> = testbridge_test_only.splitn(2, '-').collect();

    let (positive_with_globs, positive_literals): (Vec<_>, Vec<_>) = {
        let positive = positive_negative[0];
        if positive.is_empty() {
            // Forces the empty positive filter to accept everything:
            (vec!["*"], vec![])
        } else {
            positive.split(',').partition(|s| is_glob_pattern(s))
        }
    };

    let (negative_with_globs, negative_literals): (Vec<_>, Vec<_>) = match positive_negative.get(1)
    {
        Some(negative) if !negative.is_empty() => {
            negative.split(',').partition(|s| is_glob_pattern(s))
        }
        _ => (vec![], vec![]),
    };

    Collection {
        positive_equals: positive_literals.into_iter().map(|s| Equals(s.to_string())).collect(),
        positive_matches: positive_with_globs
            .into_iter()
            .map(|s| Matches(Pattern::new(s.to_string())))
            .collect(),
        negative_equals: negative_literals.into_iter().map(|s| Equals(s.to_string())).collect(),
        negative_matches: negative_with_globs
            .into_iter()
            .map(|s| Matches(Pattern::new(s.to_string())))
            .collect(),
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
    fn empty_filter_accepts_all() -> Result<()> {
        let filter = get_test_filter("");

        verify_that!(filter.filter(""), is_true())?;
        verify_that!(filter.filter("abcdefg"), is_true())?;
        Ok(())
    }

    #[test]
    fn empty_negation_filter_accepts_all() -> Result<()> {
        let filter = get_test_filter("-");

        verify_that!(filter.filter(""), is_true())?;
        verify_that!(filter.filter("abcdefg"), is_true())?;
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
        let filter = get_test_filter("a,b");

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
        let filter = get_test_filter("*test1*,*test2*");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("this is test1"), is_true())?;
        verify_that!(filter.filter("and test2 is it"), is_true())?;
        verify_that!(filter.filter("but test3 is not"), is_false())?;
        Ok(())
    }

    #[test]
    fn collection_with_globs_negation() -> Result<()> {
        let filter = get_test_filter("*test*-*testbad");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("module"), is_false())?;
        verify_that!(filter.filter("module::my_test1"), is_true())?;
        verify_that!(filter.filter("module::my_test2"), is_true())?;
        verify_that!(filter.filter("module::my_testbad"), is_false())?;
        Ok(())
    }

    #[test]
    fn mount_doom() -> Result<()> {
        let filter = get_test_filter("*mount*-*doom*");

        verify_that!(filter.filter(""), is_false())?;
        verify_that!(filter.filter("mount rushmore"), is_true())?;
        verify_that!(filter.filter("doom mount"), is_false())?;
        verify_that!(filter.filter("dismount"), is_true())?;
        verify_that!(filter.filter("mountains of moria"), is_true())?;
        verify_that!(filter.filter("frodo and sam went to mount doom"), is_false())?;
        Ok(())
    }

    #[test]
    fn collection_with_only_negation() -> Result<()> {
        let filter = get_test_filter("-testbad1,testbad2");

        verify_that!(filter.filter(""), is_true())?;
        verify_that!(filter.filter("test"), is_true())?;
        verify_that!(filter.filter("testbad1"), is_false())?;
        verify_that!(filter.filter("testbad2"), is_false())?;
        verify_that!(filter.filter("testbad3"), is_true())?;
        Ok(())
    }

    #[test]
    fn magic_words_a_and_e() -> Result<()> {
        let filter = get_test_filter("a*,e*-abracadabra,elbereth,avada kedavra");

        verify_that!(filter.filter("alakazam"), is_true())?;
        verify_that!(filter.filter("abracadabra"), is_false())?;
        verify_that!(filter.filter("avada kedavra"), is_false())?;
        verify_that!(filter.filter("enchantment"), is_true())?;
        verify_that!(filter.filter("elbereth"), is_false())?;
        verify_that!(filter.filter("expecto patronum"), is_true())?;
        verify_that!(filter.filter("fuego"), is_false())?;
        Ok(())
    }
}
