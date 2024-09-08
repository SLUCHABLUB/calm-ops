use quote::quote;
use syn::{BinOp, Error, Expr, ExprBinary, ExprCast, ExprUnary, UnOp};
use syn::spanned::Spanned;
use anyhow::{bail, Result};
use proc_macro2::TokenStream;
use crate::util::{method_name, trait_name};

pub(crate) fn wrapping_implementation(expression: Expr) -> TokenStream {
    let expression = convert_expression(&expression).unwrap_or_else(|error| {
        Error::new(expression.span(), error).to_compile_error()
    });

    quote!({
        extern crate self as calm_ops;
        #expression
    })
}

pub fn convert_expression(expression: &Expr) -> Result<TokenStream> {
    Ok(match expression {
        Expr::Binary(ExprBinary { left, op, right, .. }) => {
            // TODO: *Assign operations
            if !matches!(op, BinOp::Add(_) | BinOp::Sub(_) | BinOp::Mul(_) | BinOp::Shl(_) | BinOp::Shr(_)) {
                bail!("The only supported binary operations are +, *, <<, >> and -")
            }

            let trait_name = "Wrapping".to_owned() + trait_name(op).unwrap();
            let method = "wrapping_".to_owned() + method_name(op).unwrap();

            // TODO: recurse

            quote! {
                calm_ops::#trait_name::#method(#left, #right)
            }
        },
        Expr::Cast(ExprCast { expr, ty, .. }) => {
            quote! {
                calm_ops::WrappingCast::<#ty>::wrapping_cast(#expr)
            }
        }
        Expr::Unary(ExprUnary { op, expr, .. }) => {
            if !matches!(op, UnOp::Neg(_)) {
                bail!("The only supported unary operation is -");
            }

            // TODO: recurse

            quote! {
                calm_ops::WrappingNeg::wrapping_neg(#expr)
            }
        }
        _ => bail!("Expected an operation or cast"),
    })
}