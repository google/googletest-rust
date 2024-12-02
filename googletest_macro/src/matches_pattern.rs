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

use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser as _},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprCall, Pat, Token,
};

/// This is an implementation detail of `googletest::matches_pattern!`. It
/// assumes that a few symbols from `googletest::matchers` have been imported
/// and that `$crate` has been aliased to `googletest` (which might otherwise
/// have been imported as a different alias), both of which are done
/// by `googletest::matches_pattern!` before calling this proc macro.
pub(crate) fn matches_pattern_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = parse_macro_input!(input as ParsedMatchPattern).into_matcher_expr();
    quote! { #result }.into()
}

/// Represents one of the root-level pseudo-patterns supported by
/// `match_pattern!`.
///
/// Examples with `struct_name` set and `group` being `None`:
/// * `true`
/// * `Enum::Variant`
/// * `&Enum::Variant`
///
/// Examples with `group` being non-`None`.
/// * `Struct { a: eq(1), b: ends_with("foo") }`
/// * `&Struct(ref eq(&1), ends_with("foo"))`
#[derive(Debug)]
struct ParsedMatchPattern {
    struct_name: TokenStream,
    group: Option<Group>,
}

impl Parse for ParsedMatchPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut struct_name: Vec<TokenTree> = vec![];
        let mut group: Option<Group> = None;

        // Use a TT-muncher (as opposed to parsing `Expr` or `Pat` or such) since:
        // - The `struct_name` can be a struct/enum type or literal value (`true`, `1`,
        //   `&"test"`).
        // - The braced part supports syntax that is not valid rust like `&Foo { a: ref
        //   eq(1) }`.
        input.step(|cursor| {
            let mut rest = *cursor;

            while let Some((tt, next)) = rest.token_tree() {
                // If we reached a group (`{...}` or `(...)`), that should be the end of the
                // pattern. Other groups (`[...]`) are not supported in struct names, but that's
                // checked later.
                if let TokenTree::Group(g) = tt {
                    group = Some(g);
                    return Ok(((), next));
                }

                // Everything before the group is the struct/enum name, possibly prefixed with
                // `&`.
                struct_name.push(tt);
                rest = next;
            }

            // If no group found, remove any trailing comma that might have been
            // incorporated into the struct/enum name.
            if matches!(struct_name.last(), Some(TokenTree::Punct(p)) if p.as_char() == ',') {
                struct_name.pop();
            }

            Ok(((), rest))
        })?;

        input.parse::<Option<Token![,]>>()?;
        let struct_name = struct_name.into_iter().collect();
        Ok(ParsedMatchPattern { struct_name, group })
    }
}

impl ParsedMatchPattern {
    fn into_matcher_expr(self) -> TokenStream {
        let Self { struct_name, group } = self;
        // `matcher_pattern` supports both its custom (not necessarily valid rust)
        // syntax and also native match pattern (like `Struct { .. }` and
        // `Struct(_)`). So we need to speculatively attempt the first and then
        // fall back to the latter if the first fails. If both fail, we return
        // the error from the first attempt.
        let mut first_err = None;

        // `matcher_pattern` special syntax.
        if let Some(ref g) = group {
            let res = match g.delimiter() {
                Delimiter::Parenthesis => parse_tuple_pattern_args(struct_name.clone(), g.stream()),
                Delimiter::Brace => parse_braced_pattern_args(struct_name.clone(), g.stream()),
                Delimiter::Bracket => compile_err(g.span(), "[...] syntax is not meaningful"),
                Delimiter::None => compile_err(g.span(), "undelimited group not supported"),
            };
            match res {
                Ok(res) => return res,
                Err(e) => first_err = Some(e),
            }
        }

        // Standard `match` pattern (prioritize `first_err` if both fail).
        into_match_pattern_expr(quote! { #struct_name #group })
            .map_err(|e| first_err.unwrap_or(e))
            .unwrap_or_else(syn::Error::into_compile_error)
    }
}

/// Returns a pattern match expression as long as `stream` is a valid pattern.
/// Otherwise, returns failure.
fn into_match_pattern_expr(stream: TokenStream) -> syn::Result<TokenStream> {
    // Only produce if stream successfully parses as a pattern. Otherwise, return
    // failure so that we can instead return the error due to failing to parse
    // the `matcher_pattern` custom syntax.
    Pat::parse_multi.parse2(stream.clone())?;
    Ok(quote! {
        googletest::matchers::__internal_unstable_do_not_depend_on_these::pattern_only(
            |v| matches!(v, #stream),
            concat!("is ", stringify!(#stream)),
            concat!("is not ", stringify!(#stream)))
    })
}

////////////////////////////////////////////////////////////////////////////////
// Parse tuple struct patterns

/// Each part in a tuple matcher pattern that's between the commas for use with
/// `Punctuated`'s parser.
struct TupleFieldPattern {
    ref_token: Option<Token![ref]>,
    matcher: Expr,
}

struct MaybeTupleFieldPattern(Option<TupleFieldPattern>);

impl Parse for MaybeTupleFieldPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pattern = match input.parse::<Option<Token![_]>>()? {
            Some(_) => None,
            None => Some(TupleFieldPattern { ref_token: input.parse()?, matcher: input.parse()? }),
        };
        Ok(MaybeTupleFieldPattern(pattern))
    }
}

/// Parses a tuple struct's fields into a match expression.
fn parse_tuple_pattern_args(
    struct_name: TokenStream,
    group_content: TokenStream,
) -> syn::Result<TokenStream> {
    let parser = Punctuated::<MaybeTupleFieldPattern, Token![,]>::parse_terminated;
    let fields = parser
        .parse2(group_content)?
        .into_iter()
        .enumerate()
        .filter_map(|(index, maybe_pattern)| maybe_pattern.0.map(|pattern| (index, pattern)))
        .map(|(index, TupleFieldPattern { ref_token, matcher })| {
            let index = syn::Index::from(index);
            quote! { googletest::matchers::field!(#struct_name.#index, #ref_token #matcher) }
        });
    Ok(quote! {
        googletest::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!(#struct_name),
            all!( #(#fields),* )
        )
    })
}

////////////////////////////////////////////////////////////////////////////////
// Parse braced structs patterns

enum FieldOrMethod {
    Field(Ident),
    Method(ExprCall),
}

impl Parse for FieldOrMethod {
    /// Parses the field name or method call along with the `:` that follows it.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = if input.peek2(Token![:]) {
            input.parse().map(FieldOrMethod::Field)
        } else {
            input.parse().map(FieldOrMethod::Method)
        }?;
        input.parse::<Token![:]>()?;
        Ok(value)
    }
}

/// Either field or method call matcher. E.g.:
/// * `field: starts_with("something")`
/// * `property(arg1, arg2): starts_with("something")
struct FieldOrMethodPattern {
    ref_token: Option<Token![ref]>,
    field_or_method: FieldOrMethod,
    matcher: Expr,
}

impl Parse for FieldOrMethodPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(FieldOrMethodPattern {
            field_or_method: input.parse()?,
            ref_token: input.parse()?,
            matcher: input.parse()?,
        })
    }
}

/// Parses a struct's fields or method calls into a match expression.
fn parse_braced_pattern_args(
    struct_name: TokenStream,
    group_content: TokenStream,
) -> syn::Result<TokenStream> {
    let parser = Punctuated::<FieldOrMethodPattern, Token![,]>::parse_terminated;
    let fields = parser.parse2(group_content)?.into_iter().map(
        |FieldOrMethodPattern { ref_token, field_or_method, matcher }| match field_or_method {
            FieldOrMethod::Field(ident) => {
                quote! { field!(#struct_name . #ident, #ref_token #matcher) }
            }
            FieldOrMethod::Method(call) => {
                quote! { property!(#struct_name . #call, #ref_token #matcher) }
            }
        },
    );

    Ok(quote! {
        googletest::matchers::__internal_unstable_do_not_depend_on_these::is(
            stringify!(#struct_name),
            all!( #(#fields),* )
        )
    })
}

fn compile_err<T>(span: Span, message: &str) -> syn::Result<T> {
    Err(syn::Error::new(span, message))
}
