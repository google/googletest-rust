// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    sync::Mutex,
};

/// Interface for structure to be set up and torn down as part of a test.
/// Types implementing `Fixture` can be passed as a reference argument to a
/// test function.
///
/// ```ignore
/// struct MyFixture { ... }
///
/// impl Fixture for MyFixture { ... }
///
/// #[gtest]
/// fn test_with_fixture(my_fixture: &MyFixture) {...}
/// ```
pub trait Fixture: Sized {
    /// Factory method of the `Fixture`.
    ///
    /// This method is called by the test harness before the test case
    /// that uses this fixture. If this method returns an `Err(...)`,
    /// then the test case is not evaluated, automatically fails, and
    /// only the fixtures previously set up are torn down.
    fn set_up() -> crate::Result<Self>;

    /// Clean up method for the fixture.
    ///
    /// This method is called by the test harness after the test case
    /// that uses this fixture. If the `Fixture` has been set up, the
    /// test harness will call this method, even if the test case failed
    /// or panicked.
    fn tear_down(self) -> crate::Result<()>;
}

/// Interface for structure to be set up before the test case.
/// Types implementing `ConsumableFixture` can be passed by value to
/// a test function.
///
/// ```ignore
/// struct MyFixture { ... }
///
/// impl ConsumableFixture for MyFixture { ... }
///
/// #[gtest]
/// fn test_with_fixture(my_fixture: MyFixture) {...}
/// ```
pub trait ConsumableFixture: Sized {
    /// Factory method of the `ConsumableFixture`.
    ///
    /// This method is called by the test harness before the test case
    /// that uses this fixture. If this method returns an `Err(...)`,
    /// then the test case is not evaluated, automatically fails, and
    /// only the fixtures previously set up are torn down.
    fn set_up() -> crate::Result<Self>;
}

/// Generic adapter to implement `ConsumableFixture` on any type implementing
/// `Default`.
pub struct FixtureOf<T>(T);

impl<T: Default> ConsumableFixture for FixtureOf<T> {
    fn set_up() -> crate::Result<Self> {
        Ok(Self(T::default()))
    }
}

impl<T> Deref for FixtureOf<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FixtureOf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Interface for structure to be set up only once before all tests.
/// Types implementing `StaticFixture` can be passed as a double referenced
/// argument to a test function.
///
/// ```ignore
/// struct MyFixture{ ... }
///
/// impl StaticFixture for MyFixture { ... }
///
/// #[gtest]
/// fn test_with_fixture(my_fixture: &&MyFixture){...}
/// ```
pub trait StaticFixture: Sized + Sync + Send {
    /// Factory method of the `StaticFixture`.
    ///
    /// This method is called by the test harness before the first test case
    /// using this fixture. If this method returns an `Err(...)`, then every
    /// test case using this fixture is not evaluated and automatically fails.
    fn set_up_once() -> crate::Result<Self>;

    /// Clean up method of the `StaticFixture`.
    ///
    /// This method is called at most once, when the test harness calls
    /// `tear_down_static_fixtures` after the last test case. It is not
    /// called if `set_up_once` returned an `Err(...)`.
    ///
    /// To report a failure, use a non-fatal assertion such as `expect_that!`
    /// or [`and_log_failure`](crate::GoogleTestSupport::and_log_failure). A
    /// panic is not caught.
    ///
    /// The fixture is not dropped, since test cases hold `&'static`
    /// references to it.
    ///
    /// Test harnesses which never call `tear_down_static_fixtures`, such as
    /// the default Rust test harness, never call this method.
    fn tear_down_once(&self) {}
}

/// A `StaticFixture` which has been set up, with its type erased.
struct StaticFixtureEntry {
    type_id: TypeId,
    /// The leaked `crate::Result<F>` returned by `F::set_up_once`.
    set_up_result: &'static (dyn Any + Sync + Send),
    /// Calls `F::tear_down_once` on `set_up_result`.
    tear_down_once: fn(&'static (dyn Any + Sync + Send)),
}

/// The `StaticFixture`s which have been set up, in the order of their set-up.
static STATIC_FIXTURES: Mutex<Vec<StaticFixtureEntry>> = Mutex::new(Vec::new());

fn downcast_static_fixture<F: StaticFixture + 'static>(
    set_up_result: &'static (dyn Any + Sync + Send),
) -> &'static crate::Result<F> {
    set_up_result
        .downcast_ref::<crate::Result<F>>()
        .expect("Downcast failed. This is a bug in GoogleTest Rust")
}

fn tear_down_static_fixture<F: StaticFixture + 'static>(
    set_up_result: &'static (dyn Any + Sync + Send),
) {
    // A fixture whose set-up failed has nothing to tear down.
    if let Ok(fixture) = downcast_static_fixture::<F>(set_up_result) {
        fixture.tear_down_once();
    }
}

/// Calls [`StaticFixture::tear_down_once`] on every `StaticFixture` which has
/// been set up, in the reverse order of their set-up.
///
/// This is intended for test harnesses, which should call it once after the
/// last test case. The harness must be able to record non-fatal assertion
/// failures outside of a test case, since that is how `tear_down_once` reports
/// a failure.
///
/// The fixtures are forgotten afterwards: a test case which uses a
/// `StaticFixture` after this call sets up a new instance.
///
/// A poisoned fixture list is not supported: this function panics if a
/// `set_up_once` panicked.
///
/// **For internal use only. API stability is not guaranteed!**
#[doc(hidden)]
pub fn tear_down_static_fixtures() {
    // The lock is released before the fixtures are torn down, so that
    // `tear_down_once` may itself use a `StaticFixture`.
    let fixtures = std::mem::take(&mut *STATIC_FIXTURES.lock().expect("no `set_up_once` panicked"));
    for entry in fixtures.into_iter().rev() {
        (entry.tear_down_once)(entry.set_up_result);
    }
}

impl<F: StaticFixture + 'static> Fixture for &'static F {
    fn set_up() -> crate::Result<Self> {
        let mut fixtures = STATIC_FIXTURES.lock()?;
        let type_id = TypeId::of::<F>();
        let set_up_result = match fixtures.iter().find(|entry| entry.type_id == type_id) {
            Some(entry) => entry.set_up_result,
            None => {
                let set_up_result: &'static (dyn Any + Sync + Send) =
                    Box::leak(Box::new(F::set_up_once()));
                fixtures.push(StaticFixtureEntry {
                    type_id,
                    set_up_result,
                    tear_down_once: tear_down_static_fixture::<F>,
                });
                set_up_result
            }
        };
        match downcast_static_fixture::<F>(set_up_result) {
            Ok(fixture) => Ok(fixture),
            Err(e) => Err(e.clone()),
        }
    }

    // Note that this is `&F` being torn down, not `F`.
    fn tear_down(self) -> crate::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Once;

    use super::FixtureOf;
    use super::StaticFixture;
    use crate as googletest;
    use crate::prelude::*;
    use crate::test;
    use crate::Result;

    #[test]
    fn fixture_no_fixture() -> Result<()> {
        Ok(())
    }

    struct AlwaysSucceed;

    impl Fixture for AlwaysSucceed {
        fn set_up() -> crate::Result<Self> {
            Ok(Self)
        }

        fn tear_down(self) -> crate::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn fixture_one_fixture(_: &AlwaysSucceed) -> Result<()> {
        Ok(())
    }

    #[test]
    fn fixture_three_fixtures(
        _: &AlwaysSucceed,
        _: &AlwaysSucceed,
        _: &AlwaysSucceed,
    ) -> Result<()> {
        Ok(())
    }

    struct NotAFixture {
        a_field: i32,
    }

    impl Default for NotAFixture {
        fn default() -> Self {
            Self { a_field: 33 }
        }
    }

    #[test]
    fn fixture_of_non_fixture(not_a_fixture: FixtureOf<NotAFixture>) -> Result<()> {
        verify_that!(not_a_fixture.a_field, eq(33))
    }

    #[test]
    fn fixture_of_non_fixture_mut(mut not_a_fixture: FixtureOf<NotAFixture>) -> Result<()> {
        not_a_fixture.a_field += 10;
        verify_that!(not_a_fixture.a_field, eq(43))
    }
    struct PanickyFixture;

    impl Fixture for PanickyFixture {
        fn set_up() -> crate::Result<Self> {
            Ok(Self)
        }

        fn tear_down(self) -> crate::Result<()> {
            panic!("Whoooops");
        }
    }

    #[test]
    #[should_panic(expected = "Whoooops")]
    fn fixture_teardown_called_even_if_test_fail(_: &PanickyFixture) -> Result<()> {
        Err(googletest::TestAssertionFailure::create("It must fail!".into()))
    }

    struct AbortIfNotTornDownFixture {
        has_been_torn_down: bool,
    }

    impl Fixture for AbortIfNotTornDownFixture {
        fn set_up() -> crate::Result<Self> {
            Ok(Self { has_been_torn_down: false })
        }
        fn tear_down(mut self) -> crate::Result<()> {
            self.has_been_torn_down = true;
            Ok(())
        }
    }

    impl Drop for AbortIfNotTornDownFixture {
        fn drop(&mut self) {
            if !self.has_been_torn_down {
                eprintln!("AbortIfNotTornDownFixture was not torn down");
                std::process::abort();
            }
        }
    }

    #[test]
    #[should_panic(expected = "simple_fail")]
    fn fixture_torn_down_after_test_panics(f: &AbortIfNotTornDownFixture) {
        expect_that!(f.has_been_torn_down, eq(false));
        panic!("simple_fail");
    }

    struct OnlyOnce;

    impl StaticFixture for OnlyOnce {
        fn set_up_once() -> crate::Result<Self> {
            static ONCE: Once = Once::new();
            assert!(!ONCE.is_completed());
            ONCE.call_once(|| {});
            Ok(Self)
        }
    }

    #[test]
    fn static_fixture_works(_: &&OnlyOnce) {}

    #[test]
    fn static_fixture_same_static_fixture_twice(once: &&OnlyOnce, twice: &&OnlyOnce) {
        // checks it points to the same memory address.
        let once: *const OnlyOnce = *once;
        let twice: *const OnlyOnce = *twice;
        expect_eq!(once, twice);
    }

    struct AnotherStaticFixture;

    impl StaticFixture for AnotherStaticFixture {
        fn set_up_once() -> crate::Result<Self> {
            Ok(Self)
        }
    }

    #[test]
    fn static_fixture_two_different_static_fixtures(_: &&OnlyOnce, _: &&AnotherStaticFixture) {}

    struct FailingSetUp;

    impl Fixture for FailingSetUp {
        fn set_up() -> crate::Result<Self> {
            Err(googletest::TestAssertionFailure::create("sad fixture".into()))
        }

        fn tear_down(self) -> crate::Result<()> {
            unreachable!();
        }
    }

    #[test]
    #[should_panic(expected = "See failure output above")]
    fn failing_fixture_causes_test_failure(_: &FailingSetUp) {
        unreachable!()
    }

    struct FailingTearDown;

    impl Fixture for FailingTearDown {
        fn set_up() -> crate::Result<Self> {
            Ok(Self)
        }

        fn tear_down(self) -> crate::Result<()> {
            Err(googletest::TestAssertionFailure::create("It must fail!".into()))
        }
    }

    #[test]
    #[should_panic(expected = "See failure output above")]
    fn failing_teardown_causes_test_failure(_: &FailingTearDown) {}
}
