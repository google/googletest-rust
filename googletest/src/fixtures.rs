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
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Mutex, OnceLock},
};

/// Interface for structure to be set up and torn down as part of a test.
/// Types implementing `Fixture` can be passed as a reference argument to a
/// test function.
///
/// ```ignore
/// strct MyFixture { ... }
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
    /// If this method returns an `Err(...)`, then the test case is not
    /// evaluated, automatically fails, and only the fixtures previously
    /// set up are torn down.
    fn set_up() -> crate::Result<Self>;

    /// Clean up method for the fixture.
    ///
    /// This method is called by the test harness after the test case.
    /// If the `Fixture` has been set up, the test harness will call this
    /// method, even if the test case failed or panicked.
    fn tear_down(self) -> crate::Result<()>;
}

/// Interface for structure to be set up before the test case.
/// Types implementing `ConsumableFixture` can be passed by value to
/// a test function.
///
/// ```ignore
/// strct MyFixture { ... }
///
/// impl ConsumableFixture for MyFixture { ... }
///
/// #[gtest]
/// fn test_with_fixture(my_fixture: MyFixture) {...}
/// ```
pub trait ConsumableFixture: Sized {
    /// Factory method of the `ConsumableFixture`.
    ///
    /// This method is called by the test harness before the test case.
    /// If this method returns an `Err(...)`, then the test case is not
    /// evaluated.
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
/// strct MyFixture{ ... }
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
}

impl<F: StaticFixture + 'static> Fixture for &'static F {
    fn set_up() -> crate::Result<Self> {
        static ONCE_FIXTURE_REPO: OnceLock<
            Mutex<HashMap<TypeId, &'static (dyn Any + Sync + Send)>>,
        > = OnceLock::new();
        let mut map = ONCE_FIXTURE_REPO.get_or_init(|| Mutex::new(HashMap::new())).lock()?;
        let any =
            map.entry(TypeId::of::<F>()).or_insert_with(|| Box::leak(Box::new(F::set_up_once())));
        match any.downcast_ref::<crate::Result<F>>() {
            Some(Ok(ref fixture)) => Ok(fixture),
            Some(Err(e)) => Err(e.clone()),
            None => panic!("Downcast failed. This is a bug in GoogleTest Rust"),
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
    fn fixture_teardown_called_even_if_test_fail(_: &PanickyFixture) {
        panic!("Test failed");
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

    struct FailingFixture;

    impl Fixture for FailingFixture {
        fn set_up() -> crate::Result<Self> {
            Err(googletest::TestAssertionFailure::create("sad fixture".into()))
        }

        fn tear_down(self) -> crate::Result<()> {
            unreachable!();
        }
    }

    #[test]
    #[should_panic(expected = "See failure output above")]
    fn failing_fixture_causes_test_failure(_: &FailingFixture) {}
}
