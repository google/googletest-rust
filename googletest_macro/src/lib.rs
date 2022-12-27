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

use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks a test to be run by the Google Rust test runner.
///
/// Annotate tests the same way ordinary Rust tests are annotated:
///
/// ```rust
/// #[google_test]
/// fn should_work() -> GoogleTestResult {
///     ...
///     Ok(())
/// }
/// ```
///
/// The test function should return `GoogleTestResult` so that one can use
/// `verify_that!` with the question mark operator to abort execution. The last
/// line of the test should return `Ok(())`.
///
/// Any function your test invokes which contains a `verify_that!` call should
/// be invoked with the `?` operator so that a failure in the subroutine aborts
/// the rest of the test execution:
///
/// ```rust
/// #[google_test]
/// fn should_work() -> GoogleTestResult {
///     ...
///     assert_that_everything_is_okay()?;
///     do_some_more_stuff();  // Will not be executed if assert failed.
///     Ok(())
/// }
///
/// fn assert_that_everything_is_okay() -> GoogleTestResult {
///     verify_that!(...)
/// }
/// ```
#[proc_macro_attribute]
pub fn google_test(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_fn = parse_macro_input!(input as ItemFn);
    let fn_name = parsed_fn.sig.ident.clone();
    let output = quote! {
        #[test]
        fn #fn_name() -> std::result::Result<(), ()> {
            #parsed_fn

            use googletest::internal::test_outcome::TestOutcome;
            TestOutcome::init_current_test_outcome();
            let result = #fn_name();
            TestOutcome::close_current_test_outcome(result)
        }
    };
    output.into()
}
