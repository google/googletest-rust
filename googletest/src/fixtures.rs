use std::ops::Deref;

pub trait Fixture: Sized {
    fn set_up() -> crate::Result<Self>;
    fn tear_down(self) -> crate::Result<()> {
        Ok(())
    }
}

struct F<T: Default>(T);

impl<T: Default> Fixture for F<T> {
    fn set_up() -> crate::Result<Self> {
        Ok(Self(T::default()))
    }
}

impl<T: Default> Deref for F<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate as googletest;
    use crate::prelude::*;
    use super::F;
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
    fn not_a_fixture(not_a_fixture: &F<NotAFixture>) -> Result<()> {
        verify_that!(not_a_fixture.a_field, eq(33))
    }
}
