mod verify_pred {
    use googletest::prelude::*;
    use indoc::indoc;

    #[test]
    fn supports_nested_function_calls() -> Result<()> {
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
            err(displays_as(contains_substring(indoc! {"
                f(a, g(g(3)), 1 + 2) was false with
                  a = 1,
                  g(g(3)) = 5,
                  1 + 2 = 3,
                  at"
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
    fn supports_chained_field_access_and_method_calls() -> Result<()> {
        // Non-Debug
        struct Apple {
            b: Banana,
        }
        #[derive(Debug)]
        struct Banana;
        impl Banana {
            fn c(&self) -> bool {
                false
            }
        }

        let a = Apple { b: Banana };
        let res = verify_pred! { a.b.c() };
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                a.b.c() was false with
                  a.b = Banana,
                  at"
            })))
        )?;

        Ok(())
    }

    #[test]
    fn supports_chained_method_calls() -> Result<()> {
        #[derive(Debug)]
        struct Apple;
        impl Apple {
            fn b(&self, _b: u32) -> Banana {
                Banana
            }
        }
        // Non-Debug: Not printed on error.
        struct Banana;
        impl Banana {
            fn c(&self, _c0: u32, _c1: Cherry) -> bool {
                false
            }
        }
        // Non-Debug: not printed on error.
        #[derive(Copy, Clone)]
        struct Cherry;

        let a = Apple;
        let v = 10;
        let res = verify_pred!(a.b(v).c(11, Cherry));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                a.b(v).c(11, Cherry) was false with
                  a = Apple,
                  v = 10,
                  11 = 11,
                  at"
            })))
        )?;

        Ok(())
    }

    #[test]
    fn values_should_be_accessible_after_test() -> Result<()> {
        // Not `Copy` and should not be consumed by the generated test code.
        #[derive(Debug)]
        struct Apple;
        impl Apple {
            fn b(&self, _c: &mut u32) -> bool {
                false
            }
        }

        let mut c = 0;
        let a = Apple;
        let res = verify_pred!(a.b(&mut c));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                a.b(& mut c) was false with
                  a = Apple,
                  & mut c = 0,
                  at"
            })))
        )?;

        // `a` and `&mut c` should still be accessible after the test despite not being
        // `Copy`.
        let _ = a.b(&mut c);

        Ok(())
    }

    #[test]
    fn values_can_be_insulated_with_parens() -> Result<()> {
        // Not `Copy` and has a consuming method.
        struct Apple;
        impl Apple {
            fn b(self) -> Banana {
                Banana
            }
        }
        #[derive(Debug)]
        struct Banana;
        impl Banana {
            fn c(&self) -> bool {
                false
            }
        }

        let a = Apple;
        let res = verify_pred!({ a.b() }.c());
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {"
                { a.b() }.c() was false with
                  { a.b() } = Banana,
                  at"
            })))
        )
    }
}
