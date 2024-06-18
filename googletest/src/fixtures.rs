use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Mutex, OnceLock},
};

pub trait Fixture: Sized {
    fn set_up() -> crate::Result<Self>;
    fn tear_down(self) -> crate::Result<()> {
        Ok(())
    }
}

pub trait ConsumableFixture: Sized {
    fn set_up() -> crate::Result<Self>;
}

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

pub trait StaticFixture: Sized + Sync + Send {
    fn set_up_once() -> crate::Result<Self>;
}

impl<F: StaticFixture + 'static> Fixture for &F {
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
}

#[cfg(test)]
mod tests {

    use std::sync::Once;

    use super::StaticFixture;
    use super::FixtureOf;
    use crate as googletest;
    use crate::prelude::*;
    use crate::test;

    #[test]
    fn no_fixture() -> Result<()> {
        Ok(())
    }

    struct AlwaysSucceed;

    impl Fixture for AlwaysSucceed {
        fn set_up() -> crate::Result<Self> {
            Ok(Self)
        }
    }

    #[test]
    fn one_fixture(_: &AlwaysSucceed) -> Result<()> {
        Ok(())
    }

    #[test]
    fn three_fixture(_: &AlwaysSucceed, _: &AlwaysSucceed, _: &AlwaysSucceed) -> Result<()> {
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
    fn not_a_fixture(not_a_fixture: FixtureOf<NotAFixture>) -> Result<()> {
        verify_that!(not_a_fixture.a_field, eq(33))
    }

    struct PanickyFixture;

    impl Fixture for PanickyFixture {
        fn tear_down(self) -> crate::Result<()> {
            panic!("Whoooops");
        }

        fn set_up() -> crate::Result<Self> {
            Ok(Self)
        }
    }

    #[test]
    #[should_panic(expected = "Whoooops")]
    fn teardown_called_even_if_test_fail(_: &PanickyFixture) {
        assert!(false);
    }

    struct OnlyOnce {}

    impl StaticFixture for OnlyOnce {
        fn set_up_once() -> crate::Result<Self> {
            static ONCE: Once = Once::new();
            assert!(!ONCE.is_completed());
            ONCE.call_once(|| {});
            Ok(Self {})
        }
    }

    struct AnotherStaticFixture;

    impl StaticFixture for AnotherStaticFixture {
        fn set_up_once() -> crate::Result<Self> {
            Ok(Self)
        }
    }

    #[test]
    fn using_once(_: &&OnlyOnce) {}
    #[test]
    fn using_once_twice(_: &&OnlyOnce, _: &&OnlyOnce) {}

    #[test]
    fn using_others(_: &&OnlyOnce, _: &&AnotherStaticFixture) {}
}
