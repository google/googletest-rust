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
    fn does_not_print_literals() -> Result<()> {
        trait Foo {
            fn f(&self, _a: u32, _b: i32, _c: u32, _d: &str) -> bool {
                false
            }
        }
        impl Foo for i32 {}

        let res = verify_pred!(0.f(1, 2_i32.abs(), 1 + 2, "hello"));
        verify_that!(
            res,
            err(displays_as(contains_substring(indoc! {r#"
                0.f(1, 2_i32.abs(), 1 + 2, "hello") was false with
                  2_i32.abs() = 2,
                  1 + 2 = 3,
                  at"#
            })))
        )
    }

    #[test]
    fn supports_chained_field_access_and_method_calls_with_non_debug_types() -> Result<()> {
        // Non-Debug
        struct Apple {
            b: Banana,
        }
        #[derive(Debug)]
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
                  a does not implement Debug,
                  a.b = Banana,
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
                  a does not implement Debug,
                  b.c(false) = false,
                  at"
            })))
        )?;

        verify_that!((a.0, b.0), eq((1, 11)))
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
        // Non-Debug: not printed on error.
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
                  a.b(v) does not implement Debug,
                  Cherry does not implement Debug,
                  at"
            })))
        )
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
    fn prints_values_for_mutating_expressions() -> Result<()> {
        let mut a = 1;
        let mut b = 2;
        let mut c = 0;
        trait Mutator {
            fn mutate_and_false(&mut self, b: &mut u32) -> bool;
        }
        impl Mutator for u32 {
            fn mutate_and_false(&mut self, b: &mut u32) -> bool {
                *self += 10;
                *b += 20;
                false
            }
        }

        // Macro to to avoid the inconsistency in how `;` and `&mut` are printed between
        // Rust versions when printing out the stringified version of the block.
        macro_rules! block_a {
            () => {{
                c += 10;
                &mut a
            }};
        }
        macro_rules! block_b {
            () => {{
                c += 100;
                &mut b
            }};
        }
        let res = verify_pred! { block_a!().mutate_and_false(block_b!()) };

        verify_that!(
            res,
            // Unfortunately prints the mutated values.
            err(displays_as(contains_substring(indoc! {"
                block_a! ().mutate_and_false(block_b! ()) was false with
                  block_a! () = 11,
                  block_b! () = 22,
                  at"
            })))
        )?;

        verify_that!((a, b, c), eq((11, 22, 110)))
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
