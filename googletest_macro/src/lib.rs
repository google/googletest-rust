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
use syn::{parse_macro_input, Attribute, DeriveInput, ItemFn, ReturnType};

/// Marks a test to be run by the Google Rust test runner.
///
/// Annotate tests the same way ordinary Rust tests are annotated:
///
/// ```ignore
/// #[gtest]
/// fn should_work() {
///     ...
/// }
/// ```
///
/// The test function is not required to have a return type. If it does have a
/// return type, that type must be [`googletest::Result`]. One may do this if
/// one wishes to use both fatal and non-fatal assertions in the same test. For
/// example:
///
/// ```ignore
/// #[gtest]
/// fn should_work() -> googletest::Result<()> {
///     let value = 2;
///     expect_that!(value, gt(0));
///     verify_that!(value, eq(2))
/// }
/// ```
///
/// This macro can be used with `#[should_panic]` to indicate that the test is
/// expected to panic. For example:
///
/// ```ignore
/// #[gtest]
/// #[should_panic]
/// fn passes_due_to_should_panic() {
///     let value = 2;
///     expect_that!(value, gt(0));
///     panic!("This panics");
/// }
/// ```
///
/// Using `#[should_panic]` modifies the behaviour of `#[gtest]` so
/// that the test panics (and passes) if any non-fatal assertion occurs.
/// For example, the following test passes:
///
/// ```ignore
/// #[gtest]
/// #[should_panic]
/// fn passes_due_to_should_panic_and_failing_assertion() {
///     let value = 2;
///     expect_that!(value, eq(0));
/// }
/// ```
///
/// [`googletest::Result`]: type.Result.html
#[proc_macro_attribute]
pub fn gtest(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut parsed_fn = parse_macro_input!(input as ItemFn);
    let attrs = parsed_fn.attrs.drain(..).collect::<Vec<_>>();
    let (mut sig, block) = (parsed_fn.sig, parsed_fn.block);
    let (outer_return_type, trailer) = if attrs
        .iter()
        .any(|attr| attr.path().is_ident("should_panic"))
    {
        (quote! { () }, quote! { .unwrap(); })
    } else {
        (
            quote! { ::std::result::Result<(), googletest::internal::test_outcome::TestFailure> },
            quote! {},
        )
    };
    let output_type = match sig.output.clone() {
        ReturnType::Type(_, output_type) => Some(output_type),
        ReturnType::Default => None,
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
    let function = if let Some(output_type) = output_type {
        quote! {
            #(#attrs)*
            #sig -> #outer_return_type {
                #maybe_closure
                use googletest::internal::test_outcome::TestOutcome;
                TestOutcome::init_current_test_outcome();
                let result: #output_type = #invocation;
                TestOutcome::close_current_test_outcome(result)
                #trailer
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #sig -> #outer_return_type {
                #maybe_closure
                use googletest::internal::test_outcome::TestOutcome;
                TestOutcome::init_current_test_outcome();
                #invocation;
                TestOutcome::close_current_test_outcome(googletest::Result::Ok(()))
                #trailer
            }
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

/// Alias for [`googletest::gtest`].
///
/// Generally, prefer using `#[gtest]` to mark googletest-based tests.
///
/// Use `#[gtest]` instead of `#[gtest]` to satisfy compatibility
/// requirements. For example, the rstest crate can be composed with other test
/// attributes but it requires the attribute to be named `test`.
///
/// ```ignore
/// #[rstest]
/// #[gtest]
/// fn rstest_with_googletest() -> Result<()> {
///   verify_that!(1, eq(1))
/// }
/// ```
///
/// [`googletest::gtest`]: attr.gtest.html
#[proc_macro_attribute]
pub fn test(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    gtest(args, input)
}

fn is_test_attribute(attr: &Attribute) -> bool {
    let first_segment = match attr.path().segments.first() {
        Some(first_segment) => first_segment,
        None => return false,
    };
    let last_segment = match attr.path().segments.last() {
        Some(last_segment) => last_segment,
        None => return false,
    };
    last_segment.ident == "test"
        || (first_segment.ident == "rstest"
            && last_segment.ident == "rstest"
            && attr.path().segments.len() <= 2)
}

#[proc_macro_derive(MatcherBase)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput { ident, generics, .. } = ast;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics MatcherBase for #ident #ty_generics #where_clause {}
    }
    .into()
}
