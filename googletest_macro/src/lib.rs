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
    let ItemFn { attrs, mut sig, block, .. } = parse_macro_input!(input as ItemFn);
    let (outer_return_type, trailer) =
        if attrs.iter().any(|attr| attr.path().is_ident("should_panic")) {
            (quote! { () }, quote! { .unwrap(); })
        } else {
            (
                quote! { std::result::Result<(), googletest::internal::test_outcome::TestFailure> },
                quote! {},
            )
        };

    let inner_sig = sig.clone();
    let closure_body = match closure_body(&inner_sig) {
        Ok(body) => body,
        Err(e) => return e.into_compile_error().into(),
    };
    sig.output = ReturnType::Default;
    sig.inputs = Punctuated::new();
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
                #inner_sig { #block }
                let test = move || {
                    #closure_body
                };
            },
            quote! {
                test()
            },
        )
    };
    let function = quote! {
        #(#attrs)*
        #sig -> #outer_return_type {
            #maybe_closure
            use googletest::internal::test_outcome::TestOutcome;
            TestOutcome::init_current_test_outcome();
            let result = #invocation;
            TestOutcome::close_current_test_outcome(result)
            #trailer
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
    match attr.path().segments.last() {
        Some(last_segment) => last_segment.ident == "test",
        None => false,
    }
}

struct Fixture {
    identifier: syn::Ident,
    ty: Box<syn::Type>,
    consumable: bool,
    mutability: Option<syn::token::Mut>,
}

impl Fixture {
    fn new(index: usize, ty: Box<syn::Type>) -> Self {
        match &*ty {
            Type::Reference(reference)
                if !reference
                    .lifetime
                    .as_ref()
                    .is_some_and(|lifetime| lifetime.to_string() == "'static") =>
            {
                Self {
                    identifier: syn::Ident::new(&format!("fixture_{index}"), ty.span()),
                    ty: reference.elem.clone(),
                    consumable: false,
                    mutability: reference.mutability,
                }
            }
            Type::Path(..) | Type::Reference(..) => Self {
                identifier: syn::Ident::new(&format!("fixture_{index}"), ty.span()),
                ty,
                consumable: true,
                mutability: None,
            },
            Type::Array(_) => todo!(),
            Type::BareFn(_) => todo!(),
            Type::Group(_) => todo!(),
            Type::ImplTrait(_) => todo!(),
            Type::Infer(_) => todo!(),
            Type::Macro(_) => todo!(),
            Type::Never(_) => todo!(),
            Type::Paren(_) => todo!(),
            Type::Ptr(_) => todo!(),
            Type::Slice(_) => todo!(),
            Type::TraitObject(_) => todo!(),
            Type::Tuple(_) => todo!(),
            Type::Verbatim(_) => todo!(),
            _ => todo!(),
        }
    }

    fn parameter(&self) -> proc_macro2::TokenStream {
        let Self { identifier, mutability, consumable, .. } = self;
        if *consumable { quote!(#identifier) } else { quote!(& #mutability #identifier) }
    }

    fn wrap_call(&self, inner_call: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let Self { identifier, mutability, ty, consumable } = self;
        if *consumable {
            quote!(
                let #identifier = <#ty as googletest::fixtures::ConsumableFixture>::set_up()?;
                (||{#inner_call})()
            )
        } else {
            quote!(
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
            Ok(Fixture::new(index, ty.clone()))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let mut call = {
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
        call = fixture.wrap_call(call);
    }

    Ok(call)
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
