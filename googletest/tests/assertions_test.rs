mod verify_pred {
    use googletest::prelude::*;
    use indoc::indoc;

    #[test]
    fn supports_function_call_with_non_debug_types() -> Result<()> {
        // Non-Debug - cannot be printed.
        struct Apple;
        fn f(_a: &Apple, _b: u32, _c: u32) -> bool {
            false
        }
        fn g(_a: u32) -> u32 {
            5
        }

        let a = &Apple;
        let res = verify_pred!(f(a, g(g(3)), 1 + 2));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                f(a, g(g(3)), 1 + 2) was false with
                  a does not implement Debug,
                  g(g(3)) = 5,
                  1 + 2 = 3,
                  at"
            })))
        )
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
    fn supports_method_calls_with_non_debug_types() -> Result<()> {
        struct Apple {
            b: Banana,
        }
        struct Banana;
        impl Banana {
            fn c(&self, _c: &Cherry, _d: u32) -> bool {
                false
            }
        }
        // Non-Debug - cannot be printed.
        struct Cherry;

        let a = Apple { b: Banana };
        let c = &Cherry;
        let d = 3;
        let res = verify_pred!(a.b.c(c, d));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                a.b.c(c, d) was false with
                  c does not implement Debug,
                  d = 3,
                  at"
            })))
        )
    }

    #[test]
    fn evaluates_functions_and_arguments_exactly_once() -> Result<()> {
        let mut a = 0;
        let mut foo = |_b: u32| {
            a += 1;
            false
        };
        let mut b = 0;
        let mut bar = || {
            b += 10;
            b
        };

        let res = verify_pred!(foo(bar()));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                foo(bar()) was false with
                  bar() = 10,
                  at"
            })))
        )?;

        verify_that!((a, b), eq((1, 10)))
    }

    #[test]
    fn evaluates_methods_and_arguments_exactly_once() -> Result<()> {
        struct Apple(u32);
        impl Apple {
            fn c(&mut self, _b: bool) -> bool {
                self.0 += 1;
                false
            }
        }
        let mut a = Apple(0);
        let mut b = Apple(10);

        let res = verify_pred!(a.c(b.c(false)));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                a.c(b.c(false)) was false with
                  b.c(false) = false,
                  at"
            })))
        )?;

        verify_that!((a.0, b.0), eq((1, 11)))
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
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                a.b().c() was false with
                  at"
            })))
        )
    }
}
