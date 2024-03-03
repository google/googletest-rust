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

use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, ItemFn, ReturnType};

/// Marks a test to be run by the Google Rust test runner.
///
/// Annotate tests the same way ordinary Rust tests are annotated:
///
/// ```ignore
/// #[googletest::test]
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
/// #[googletest::test]
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
/// #[googletest::test]
/// #[should_panic]
/// fn passes_due_to_should_panic() {
///     let value = 2;
///     expect_that!(value, gt(0));
///     panic!("This panics");
/// }
/// ```
///
/// Using `#[should_panic]` modifies the behaviour of `#[googletest::test]` so
/// that the test panics (and passes) if any non-fatal assertion occurs.
/// For example, the following test passes:
///
/// ```ignore
/// #[googletest::test]
/// #[should_panic]
/// fn passes_due_to_should_panic_and_failing_assertion() {
///     let value = 2;
///     expect_that!(value, eq(0));
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
    let (outer_return_type, trailer) =
        if attrs.iter().any(|attr| attr.path().is_ident("should_panic")) {
            (quote! { () }, quote! { .unwrap(); })
        } else {
            (
                quote! { std::result::Result<(), googletest::internal::test_outcome::TestFailure> },
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

#[proc_macro]
#[allow(non_snake_case)]
pub fn TEST_F(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let TestWithFixtureArgs { fixture, function, block, .. } =
        parse_macro_input!(input as TestWithFixtureArgs);

    let test_name = match type_path_to_ident(fixture.to_token_stream()) {
        Ok(ident) => format!("{ident}{function}"),
        Err(e) => return e.into_compile_error().into(),
    };
    let test_name = syn::Ident::new(&test_name, fixture.span());

    quote!(
        impl #fixture {
            #[allow(non_snake_case)]
            fn #function (&mut self) -> googletest::Result<()> #block
        }

        #[allow(non_snake_case)]
        #[googletest::test]
        fn #test_name() -> googletest::Result<()> {
            let mut fixture = <#fixture as TestFixture>::create();
            fixture.set_up();
            fixture.#function()?;
            fixture.tear_down()
        }

    )
    .into()
}

struct TestWithFixtureArgs {
    fixture: syn::TypePath,
    _comma: syn::Token![,],
    function: syn::Ident,
    _comma2: syn::Token![,],
    block: syn::Block,
}

impl syn::parse::Parse for TestWithFixtureArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fixture: input.parse()?,
            _comma: input.parse()?,
            function: input.parse()?,
            _comma2: input.parse()?,
            block: input.parse()?,
        })
    }
}

fn type_path_to_ident(token_stream: proc_macro2::TokenStream) -> syn::Result<String> {
    let mut ident = String::new();
    for token in token_stream.into_iter() {
        match token {
            proc_macro2::TokenTree::Group(group) => {
                ident.push_str(&type_path_to_ident(group.stream())?);
            }
            proc_macro2::TokenTree::Ident(i) => ident.push_str(&i.to_string()),
            proc_macro2::TokenTree::Punct(_) => ident.push('_'),
            proc_macro2::TokenTree::Literal(literal) => ident.push_str(&literal.to_string()),
        }
    }
    Ok(ident)
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn TEST(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let TestWithSuiteArgs { suite, function, block, .. } =
        parse_macro_input!(input as TestWithSuiteArgs);

    let test_name = format!("{suite}_{function}");

    let test_name = syn::Ident::new(&test_name, suite.span());

    quote!(
        #[allow(non_snake_case)]
        #[googletest::test]
        fn #test_name() -> googletest::Result<()> #block

    )
    .into()
}

struct TestWithSuiteArgs {
    suite: syn::Ident,
    _comma: syn::Token![,],
    function: syn::Ident,
    _comma2: syn::Token![,],
    block: syn::Block,
}

impl syn::parse::Parse for TestWithSuiteArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            suite: input.parse()?,
            _comma: input.parse()?,
            function: input.parse()?,
            _comma2: input.parse()?,
            block: input.parse()?,
        })
    }
}
