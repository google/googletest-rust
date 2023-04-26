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
use syn::{parse_macro_input, Attribute, ItemFn, ReturnType};

/// Marks a test to be run by the Google Rust test runner.
///
/// Annotate tests the same way ordinary Rust tests are annotated:
///
/// ```ignore
/// #[googletest::test]
/// fn should_work() -> googletest::Result {
///     ...
///     Ok(())
/// }
/// ```
///
/// The test function should return [`googletest::Result`] so that one can use
/// `verify_that!` with the question mark operator to abort execution. The last
/// line of the test should return `Ok(())`.
///
/// Any function your test invokes which contains a `verify_that!` call should
/// be invoked with the `?` operator so that a failure in the subroutine aborts
/// the rest of the test execution:
///
/// ```ignore
/// #[googletest::test]
/// fn should_work() -> googletest::Result {
///     ...
///     assert_that_everything_is_okay()?;
///     do_some_more_stuff();  // Will not be executed if assert failed.
///     Ok(())
/// }
///
/// fn assert_that_everything_is_okay() -> googletest::Result {
///     verify_that!(...)
/// }
/// ```
///
/// [`googletest::Result`]: type.Result.html
#[proc_macro_attribute]
pub fn test(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut parsed_fn = parse_macro_input!(input as ItemFn);
    let attrs = parsed_fn.attrs.drain(..).collect::<Vec<_>>();
    let (mut sig, block) = (parsed_fn.sig, parsed_fn.block);
    let ReturnType::Type(_, output_type) = sig.output.clone() else {
        return quote! {
            compile_error!(
                "Test function with the #[googletest::test] attribute must return googletest::Result<()>"
            );
        }.into();
    };
    sig.output = ReturnType::Default;
    let (maybe_closure, invocation) = if sig.asyncness.is_some() {
        (
            // In the async case, the ? operator returns from the *block* rather than the
            // surrounding function. So we just put the test content in an async block. Async
            // closures are still unstable (see https://github.com/rust-lang/rust/issues/62290),
            // so we can't use the same solution as the sync case below.
            quote! {},
            quote! {
                async { #block }.await
            },
        )
    } else {
        (
            // In the sync case, the ? operator returns from the surrounding function. So we must
            // create a separate closure from which the ? operator can return in order to capture
            // the output.
            quote! {
                let test = move || #block;
            },
            quote! {
                test()
            },
        )
    };
    let function = quote! {
        #(#attrs)*
        #sig -> std::result::Result<(), ()> {
            #maybe_closure
            use googletest::internal::test_outcome::TestOutcome;
            TestOutcome::init_current_test_outcome();
            let result: #output_type = #invocation;
            TestOutcome::close_current_test_outcome(result)
        }
    };
    let output = if attrs.iter().any(is_test_attribute) {
        function
    } else {
        quote! {
            #[::core::prelude::v1::test]
            #function
        }
    };
    output.into()
}

fn is_test_attribute(attr: &Attribute) -> bool {
    let Some(first_segment) = attr.path().segments.first() else {
        return false;
    };
    let Some(last_segment) = attr.path().segments.last() else {
        return false;
    };
    last_segment.ident == "test"
        || (first_segment.ident == "rstest"
            && last_segment.ident == "rstest"
            && attr.path().segments.len() <= 2)
}
