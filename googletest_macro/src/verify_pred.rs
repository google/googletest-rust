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
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, Ident};

struct AccumulatePartsState {
    var_num: usize,
    error_message_ident: Ident,
    statements: Vec<proc_macro2::TokenStream>,
}

fn expr_to_string(expr: &Expr) -> String {
    quote!(#expr).to_string()
}

impl AccumulatePartsState {
    fn new() -> Self {
        Self {
            var_num: 0,
            error_message_ident: Ident::new(
                "__googletest__verify_pred__error_message",
                ::proc_macro2::Span::call_site(),
            ),
            statements: vec![],
        }
    }

    /// Takes an expression with chained field accesses and method calls and
    /// accumulates intermediate expressions used for computing `verify_pred!`'s
    /// expression, including intermediate variable assignments to evaluate
    /// parts of the expression exactly once, and the format string used to
    /// output intermediate values on condition failure. It returns the new form
    /// of the input expression with parts of it potentially replaced by the
    /// intermediate variables.
    fn accumulate_parts(&mut self, expr: Expr) -> Expr {
        // Literals don't need to be printed or stored in intermediate variables.
        if is_literal(&expr) {
            return expr;
        }
        let expr_string = expr_to_string(&expr);
        let new_expr = match expr {
            Expr::Group(mut group) => {
                // This is an invisible group added for correct precedence in the AST. Just pass
                // through without having a separate printing result.
                *group.expr = self.accumulate_parts(*group.expr);
                return Expr::Group(group);
            }
            Expr::Field(mut field) => {
                // Don't assign field access to an intermediate variable to avoid moving out of
                // non-`Copy` fields.
                *field.base = self.accumulate_parts(*field.base);
                Expr::Field(field)
            }
            Expr::Call(mut call) => {
                // Cache args into intermediate variables.
                call.args = self.define_variables_for_args(call.args);
                // Cache function value into an intermediate variable.
                self.define_variable(&Expr::Call(call))
            }
            Expr::MethodCall(mut method_call) => {
                *method_call.receiver = self.accumulate_parts(*method_call.receiver);
                // Cache args into intermediate variables.
                method_call.args = self.define_variables_for_args(method_call.args);
                // Cache method value into an intermediate variable.
                self.define_variable(&Expr::MethodCall(method_call))
            }
            Expr::Binary(mut binary) => {
                *binary.left = self.accumulate_parts(*binary.left);
                *binary.right = self.accumulate_parts(*binary.right);
                Expr::Binary(binary)
            }
            Expr::Unary(mut unary) => {
                *unary.expr = self.accumulate_parts(*unary.expr);
                Expr::Unary(unary)
            }
            // A path expression doesn't need to be stored in an intermediate variable.
            // This avoids moving out of an existing variable.
            Expr::Path(_) => expr,
            // By default, assume it's some expression that needs to be cached to avoid
            // double-evaluation.
            _ => self.define_variable(&expr),
        };
        let error_message_ident = &self.error_message_ident;
        self.statements.push(quote! {
            ::googletest::fmt::internal::__googletest__write_expr_value!(
                &mut #error_message_ident,
                #expr_string,
                #new_expr,
            );
        });
        new_expr
    }

    // Defines a variable for each argument expression so that it's evaluated
    // exactly once.
    fn define_variables_for_args(
        &mut self,
        args: Punctuated<Expr, Comma>,
    ) -> Punctuated<Expr, Comma> {
        args.into_pairs()
            .map(|mut pair| {
                // Don't need to assign literals to intermediate variables.
                if is_literal(pair.value()) {
                    return pair;
                }

                let var_expr = self.define_variable(pair.value());
                let error_message_ident = &self.error_message_ident;
                let expr_string = expr_to_string(pair.value());
                self.statements.push(quote! {
                    ::googletest::fmt::internal::__googletest__write_expr_value!(
                        &mut #error_message_ident,
                        #expr_string,
                        #var_expr,
                    );
                });

                *pair.value_mut() = var_expr;
                pair
            })
            .collect()
    }

    /// Defines a new variable assigned to the expression and returns the
    /// variable as an expression to be used in place of the passed-in
    /// expression.
    fn define_variable(&mut self, value: &Expr) -> Expr {
        let var_name =
            Ident::new(&format!("__googletest__verify_pred__var{}", self.var_num), value.span());
        self.var_num += 1;
        self.statements.push(quote! {
            #[allow(non_snake_case)]
            let mut #var_name = #value;
        });
        syn::parse::<Expr>(quote!(#var_name).into()).unwrap()
    }
}

// Whether it's a literal or unary operator applied to a literal (1, -1).
fn is_literal(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Unary(unary) => matches!(&*unary.expr, Expr::Lit(_)),
        _ => false,
    }
}

pub fn verify_pred_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(input as Expr);
    let error_message = quote!(#parsed).to_string() + " was false with";

    let mut state = AccumulatePartsState::new();
    let pred_value = state.accumulate_parts(parsed);
    let AccumulatePartsState { error_message_ident, mut statements, .. } = state;

    let _ = statements.pop(); // The last statement prints the full expression itself.
    quote! {
        {
            let mut #error_message_ident = #error_message.to_string();
            #(#statements)*
            if (#pred_value) {
                Ok(())
            } else {
                ::core::result::Result::Err(
                    ::googletest::internal::test_outcome::TestAssertionFailure::create(
                        #error_message_ident))
            }
        }
    }
    .into()
}
