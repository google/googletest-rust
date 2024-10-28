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
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Attribute, DeriveInput, FnArg,
    ItemFn, PatType, ReturnType, Signature, Type,
};

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
    let ItemFn { attrs, sig, block, .. } = parse_macro_input!(input as ItemFn);
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

    let is_rstest_enabled = is_rstest_enabled(&attrs);
    let outer_sig = {
        let mut outer_sig = sig.clone();
        outer_sig.output = ReturnType::Default;
        if !is_rstest_enabled {
            outer_sig.inputs = Punctuated::new();
        }
        outer_sig
    };

    let (output_type, result) = match sig.output {
        ReturnType::Default => (None, quote! {googletest::Result::Ok(())}),
        ReturnType::Type(_, ref ty) => (Some(quote! {#ty}), quote! {result}),
    };

    let (maybe_closure, invocation, invocation_result_type) =
        match (sig.asyncness.is_some(), is_rstest_enabled) {
            (true, false) if !sig.inputs.is_empty() => {
                // TODO add support for fixtures in async tests.
                return syn::Error::new(
                sig.span(),
                "Googletest does not currently support fixture with async. Consider using rstest",
            )
            .into_compile_error()
            .into();
            }
            (true, _) => {
                (
                    // In the async case, the ? operator returns from the *block* rather than the
                    // surrounding function. So we just put the test content in an async block.
                    // Async closures are still unstable (see https://github.com/rust-lang/rust/issues/62290),
                    // so we can't use the same solution as the sync case below.
                    quote! {},
                    quote! {
                        async { #block }.await
                    },
                    output_type.unwrap_or_else(|| quote! {()}),
                )
            }
            (false, false) => {
                let closure_body = match closure_body(&sig) {
                    Ok(body) => body,
                    Err(e) => return e.into_compile_error().into(),
                };

                (
                    // In the sync case, the ? operator returns from the surrounding function. So
                    // we redeclare the original test function internally.
                    quote! {
                        #sig { #block }
                        let test = move || {
                            #closure_body
                        };
                    },
                    quote! {
                        test()
                    },
                    output_type.unwrap_or_else(|| quote! {googletest::Result<()>}),
                )
            }
            (false, true) => {
                (
                    // Rstest may refer in block to its fixtures. Hence, we only wrap it in a
                    // closure to capture them.
                    quote! {
                        let test = move || {
                            #block
                        };
                    },
                    quote! {
                        test()
                    },
                    output_type.unwrap_or_else(|| quote! {()}),
                )
            }
        };
    let function = quote! {
        #(#attrs)*
        #outer_sig -> #outer_return_type {
            #maybe_closure
            use googletest::internal::test_outcome::TestOutcome;
            TestOutcome::init_current_test_outcome();
            let result: #invocation_result_type = #invocation;
            TestOutcome::close_current_test_outcome(#result)
            #trailer
        }
    };

    let output = if attrs.iter().any(is_test_attribute) || is_rstest_enabled {
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
/// Use `#[test]` instead of `#[gtest]` to satisfy compatibility
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
    match attr.path().segments.last() {
        Some(last_segment) => last_segment.ident == "test",
        None => false,
    }
}

fn is_rstest_enabled(attributes: &[Attribute]) -> bool {
    for attr in attributes {
        if matches!(attr.path().segments.last(), Some(last_segment) if last_segment.ident == "rstest")
        {
            return true;
        }
    }
    false
}

struct Fixture {
    identifier: syn::Ident,
    ty: Box<syn::Type>,
    consumable: bool,
    mutability: Option<syn::token::Mut>,
}

impl Fixture {
    fn new(index: usize, ty: Box<syn::Type>) -> syn::Result<Self> {
        let identifier = syn::Ident::new(&format!("__googletest__fixture__{index}"), ty.span());
        match &*ty {
            Type::Reference(reference) => Ok(Self {
                identifier,
                ty: reference.elem.clone(),
                consumable: false,
                mutability: reference.mutability,
            }),
            Type::Path(..) => Ok(Self { identifier, ty, consumable: true, mutability: None }),
            _ => Err(syn::Error::new(
                ty.span(),
                "Unexpected fixture type. Only references (&T or &mut T) and paths (T) are supported.",
            )),
        }
    }

    fn parameter(&self) -> proc_macro2::TokenStream {
        let Self { identifier, mutability, consumable, .. } = self;
        if *consumable {
            quote!(#identifier)
        } else {
            quote!(& #mutability #identifier)
        }
    }

    fn wrap_call(&self, inner_call: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let Self { identifier, mutability, ty, consumable } = self;
        if *consumable {
            quote!(
                #[allow(non_snake_case)]
                let #identifier = <#ty as googletest::fixtures::ConsumableFixture>::set_up()?;
                (||{#inner_call})()
            )
        } else {
            quote!(
                #[allow(non_snake_case)]
                let #mutability #identifier = <#ty as googletest::fixtures::Fixture>::set_up()?;
                let result = std::panic::catch_unwind(|| {#inner_call});
                let tear_down_result = googletest::fixtures::Fixture::tear_down(#identifier);
                match result {
                    Ok(result) => result.and(tear_down_result),
                    Err(panic_error) => std::panic::resume_unwind(panic_error)
                }
            )
        }
    }
}

fn closure_body(signature: &Signature) -> syn::Result<proc_macro2::TokenStream> {
    let input_types = signature
        .inputs
        .iter()
        .enumerate()
        .map(|(index, typed)| {
            let FnArg::Typed(PatType { ty, .. }) = typed else {
                return Err(syn::Error::new(
                    typed.span(),
                    "`self` receiver is not accepted as test argument",
                ));
            };
            Fixture::new(index, ty.clone())
        })
        .collect::<syn::Result<Vec<Fixture>>>()?;

    let mut block = {
        let parameters = input_types.iter().map(Fixture::parameter);

        let test_name = &signature.ident;
        match signature.output {
            ReturnType::Default => {
                quote!({#test_name(#(#parameters, )*); googletest::Result::Ok(())})
            }
            ReturnType::Type(_, _) => quote!(#test_name(#(#parameters, )*)),
        }
    };

    for fixture in input_types.iter().rev() {
        block = fixture.wrap_call(block);
    }

    Ok(block)
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

mod verify_pred;

/// This is an implementation detail of `verify_pred!`.
///
/// It's not intended to be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __googletest_macro_verify_pred(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    verify_pred::verify_pred_impl(input)
}
