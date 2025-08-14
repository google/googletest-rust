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
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, Attribute,
    DeriveInput, Expr, ExprLit, FnArg, ItemFn, Lit, MetaNameValue, PatType, ReturnType, Signature,
    Type,
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
    let ItemFn { mut attrs, sig, block, .. } = parse_macro_input!(input as ItemFn);

    let sig_ident = &sig.ident;
    let test_case_hash: u64 = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();

        // Only consider attrs and name for stability. Changing the function body should
        // not affect the test case distribution.
        attrs.hash(&mut h);
        sig_ident.hash(&mut h);
        h.finish()
    };

    let (skipped_test_result, outer_return_type, trailer) = attrs
        .iter()
        .find(|attr| attr.path().is_ident("should_panic"))
        .map(|attr| {
            let error_message = extract_should_panic_expected(attr).unwrap_or("".to_string());
            (
                quote! {
                    {
                        panic!("{}", #error_message);
                    }
                },
                quote! { () },
                quote! { .unwrap(); }
            )})
        .unwrap_or_else(||
        (
            quote! {Ok(())},
            quote! { ::core::result::Result<(), googletest::internal::test_outcome::TestFailure> },
            quote! {},
        ));

    let is_rstest_enabled = is_rstest_enabled(&attrs);
    let outer_sig = {
        let mut outer_sig = sig.clone();
        outer_sig.output = ReturnType::Default;
        if !is_rstest_enabled {
            outer_sig.inputs = Punctuated::new();
        }
        outer_sig
    };

    let output_type = match sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ref ty) => Some(quote! {#ty}),
    };

    let (maybe_closure, result, invocation, invocation_result_type) =
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
                    match output_type {
                        Some(_) => quote! {result},
                        None => quote! {googletest::Result::Ok(())},
                    },
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
                    quote! {result},
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
                    match output_type {
                        Some(_) => quote! {result},
                        None => quote! {googletest::Result::Ok(())},
                    },
                    quote! {
                        test()
                    },
                    output_type.unwrap_or_else(|| quote! {()}),
                )
            }
        };
    if !attrs.iter().any(is_test_attribute) && !is_rstest_enabled {
        let test_attr: Attribute = parse_quote! {
            #[::core::prelude::v1::test]
        };
        attrs.push(test_attr);
    };
    let function = quote! {
        #(#attrs)*
        #outer_sig -> #outer_return_type {
            #maybe_closure
            if !googletest::internal::test_filter::test_should_run(concat!(module_path!(), "::", stringify!(#sig_ident))) {
                #skipped_test_result
            } else if googletest::internal::test_sharding::test_should_run(#test_case_hash) {
                use googletest::internal::test_outcome::TestOutcome;
                TestOutcome::init_current_test_outcome();
                let result: #invocation_result_type = #invocation;
                TestOutcome::close_current_test_outcome(#result)
            } else {
                #skipped_test_result
            }
            #trailer
        }
    };
    function.into()
}

/// Extract the optional "expected" string literal from a `should_panic`
/// attribute.
fn extract_should_panic_expected(attr: &Attribute) -> Option<String> {
    let Ok(name_value) = attr.parse_args::<MetaNameValue>() else {
        return None;
    };
    match name_value.value {
        Expr::Lit(ExprLit { lit: Lit::Str(expected), .. })
            if name_value.path.is_ident("expected") =>
        {
            Some(expected.value())
        }
        _ => None,
    }
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
/// #[googletest::test]
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
    attributes.iter().any(|attr| matches!(attr.path().segments.last(), Some(last_segment) if last_segment.ident == "rstest"))
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

mod matches_pattern;

/// This is an implementation detail of `googletest::matches_pattern!`.
///
/// It's not intended to be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __googletest_macro_matches_pattern(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    matches_pattern::matches_pattern_impl(input)
}

mod verify_pred;

/// This is an implementation detail of `googletest::verify_pred!`.
///
/// It's not intended to be used directly.
#[doc(hidden)]
#[proc_macro]
pub fn __googletest_macro_verify_pred(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    verify_pred::verify_pred_impl(input)
}

/// Stringifies its argument (like `stringify!()` from the standard library) but
/// limits the output to a provided maximum length.
///
/// The input is a tuple of `target` and `max_length` seprated by a comma.
/// The `max_length` is the maximum number of characters to include in the
/// abbreviated string. For example:
///
/// ```ignore
/// #[rstest]
/// #[gtest]
/// fn test_abbreviated_string() -> Result<()> {
///   verifiy_eq!(__abbreviated_stringify!(|x| x + 1, 6), "|x|...")?;
///   Ok(())
/// }
/// ```
#[doc(hidden)]
#[proc_macro]
pub fn __abbreviated_stringify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();
    let abbreviated = abbreviated_string(&input).unwrap();
    quote! {
        #abbreviated
    }
    .into()
}

fn abbreviated_string(target: &str) -> Result<std::borrow::Cow<'_, str>, &'static str> {
    use std::borrow::Cow;
    match target.rsplit_once(',') {
        None => Err("Expect a `max_length` argument, but got none"),
        Some((expr, limit)) => match limit.trim().parse::<usize>() {
            Ok(limit) if expr.len() > limit => {
                if limit >= 4 {
                    Ok(Cow::Owned(format!("{}...", &expr[..limit - 3])))
                } else {
                    Err("The `max_length` argument is too small. It must be at least 4.")
                }
            }
            Ok(_) => Ok(Cow::Borrowed(expr)),
            Err(_) => Err("The `max_length` argument is not a number."),
        },
    }
}
