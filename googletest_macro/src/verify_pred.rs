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

use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Expr, Ident};

struct AccumulatePartsState {
    error_message_ident: Ident,
    formats: Vec<proc_macro2::TokenStream>,
}

fn expr_to_string(expr: &Expr) -> String {
    quote!(#expr).to_string()
}

impl AccumulatePartsState {
    fn new() -> Self {
        Self {
            error_message_ident: Ident::new(
                "__googletest__verify_pred__error_message",
                ::proc_macro2::Span::call_site(),
            ),
            formats: vec![],
        }
    }

    /// Accumulates error message formating parts for various parts of the
    /// expression.
    fn accumulate_parts(&mut self, expr: &Expr) {
        let expr_string = expr_to_string(expr);
        match expr {
            Expr::Group(group) => {
                // This is an invisible group added for correct precedence in the AST. Just pass
                // through without having a separate printing result.
                return self.accumulate_parts(&group.expr);
            }
            Expr::Call(call) => {
                // Format the args into the error message.
                self.format_args(&call.args);
            }
            Expr::MethodCall(method_call) => {
                // Format the args into the error message.
                self.format_args(&method_call.args);
            }
            _ => {}
        }
        let error_message_ident = &self.error_message_ident;
        self.formats.push(quote! {
            ::googletest::fmt::internal::__googletest__write_expr_value!(
                &mut #error_message_ident,
                #expr_string,
                #expr,
            );
        });
    }

    // Formats each argument expression into the error message.
    fn format_args(&mut self, args: &Punctuated<Expr, Comma>) {
        for pair in args.pairs() {
            let error_message_ident = &self.error_message_ident;
            let expr_string = expr_to_string(pair.value());
            let expr = pair.value();
            self.formats.push(quote! {
                ::googletest::fmt::internal::__googletest__write_expr_value!(
                    &mut #error_message_ident,
                    #expr_string,
                    #expr,
                );
            });
        }
    }
}

pub fn verify_pred_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as Expr);
    let error_message = quote!(#parsed).to_string() + " was false with";

    let mut state = AccumulatePartsState::new();
    state.accumulate_parts(&parsed);
    let AccumulatePartsState { error_message_ident, mut formats, .. } = state;

    let _ = formats.pop(); // The last one is the full expression itself.
    quote! {
        {
            if (#parsed) {
                Ok(())
            } else {
                let mut #error_message_ident = #error_message.to_string();
                #(#formats)*
                ::core::result::Result::Err(
                    ::googletest::internal::test_outcome::TestAssertionFailure::create(
                        #error_message_ident))
            }
        }
    }
    .into()
}
