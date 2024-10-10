mod verify_pred {
    use googletest::prelude::*;
    use indoc::indoc;

    #[test]
    fn supports_function_call() -> Result<()> {
        fn f(_a: u32, _b: u32, _c: u32) -> bool {
            false
        }
        fn g(_a: u32) -> u32 {
            5
        }

        let a = 1;
        let res = verify_pred!(f(a, g(g(3)), 1 + 2));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {
                "
                f(a, g(g(3)), 1 + 2) was false with
                  a = 1,
                  g(g(3)) = 5,
                  1 + 2 = 3
                "
            })))
        )?;

        Ok(())
    }

    #[test]
    fn supports_trailing_comma() -> Result<()> {
        verify_that!(verify_pred!(false,), err(anything()))
    }

    #[test]
    fn supports_non_function() -> Result<()> {
        verify_pred!(true)?;
        verify_that!(verify_pred!(false), err(anything()))
    }

    #[test]
    fn supports_method_calls() -> Result<()> {
        struct Foo {
            b: Bar,
        }
        struct Bar;
        impl Bar {
            fn c(&self) -> bool {
                false
            }
        }

        let a = Foo { b: Bar };
        let res = verify_pred!(a.b.c());
        verify_that!(res, err(displays_as(contains_substring("a.b.c() was false"))))
    }

    #[test]
    fn supports_chained_method_calls() -> Result<()> {
        struct Foo;
        impl Foo {
            fn b(self) -> Bar {
                Bar
            }
        }
        struct Bar;
        impl Bar {
            fn c(self) -> bool {
                false
            }
        }

        let a = Foo;
        let res = verify_pred!(a.b().c());
        verify_that!(res, err(displays_as(contains_substring("a.b().c() was false"))))?;

        Ok(())
    }
}
