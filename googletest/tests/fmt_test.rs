// Copyright 2024 Google LLC
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

mod write_expr_value {
    use googletest::prelude::*;

    // Converts the formatting call to a `String` for testing.
    macro_rules! write_expr_value {
        ($expr_str:expr, $expr: expr $(,)?) => {{
            let mut s = String::new();
            ::googletest::fmt::internal::__googletest__write_expr_value!(s, $expr_str, $expr);
            s
        }};
    }

    #[test]
    fn test_with_debug_value_references() -> Result<()> {
        #[derive(Debug)]
        struct Foo;
        let mut val = Foo;

        verify_that!(write_expr_value!("val", val), eq("\n  val = Foo,"))?;
        verify_that!(write_expr_value!("val", &val), eq("\n  val = Foo,"))?;
        verify_that!(write_expr_value!("val", &&val), eq("\n  val = Foo,"))?;
        verify_that!(write_expr_value!("val", &mut val), eq("\n  val = Foo,"))?;
        verify_that!(write_expr_value!("val", &mut &mut val), eq("\n  val = Foo,"))?;

        Ok(())
    }

    #[test]
    fn test_with_non_debug_value_references() -> Result<()> {
        struct Foo;
        let mut val = Foo;

        verify_that!(write_expr_value!("val", val), eq("\n  val does not implement Debug,"))?;
        verify_that!(write_expr_value!("val", &val), eq("\n  val does not implement Debug,"))?;
        verify_that!(write_expr_value!("val", &&val), eq("\n  val does not implement Debug,"))?;
        verify_that!(write_expr_value!("val", &mut val), eq("\n  val does not implement Debug,"))?;
        verify_that!(
            write_expr_value!("val", &mut &mut val),
            eq("\n  val does not implement Debug,")
        )?;

        Ok(())
    }
}
