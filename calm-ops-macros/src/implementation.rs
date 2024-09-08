use anyhow::{anyhow, bail, ensure, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{BinOp, Error, Expr, ExprBinary, ExprCast, ExprUnary, UnOp};

fn trait_method_suffix(operation: &BinOp) -> Option<(&'static str, &'static str)> {
    Some(match operation {
        BinOp::Add(_) => ("Add", "add"),
        BinOp::Sub(_) => ("Sub", "sub"),
        BinOp::Mul(_) => ("Mul", "mul"),
        BinOp::Div(_) => ("Div", "div"),
        BinOp::Rem(_) => ("Rem", "rem"),
        BinOp::BitXor(_) => ("BitXor", "bitor"),
        BinOp::BitAnd(_) => ("BitAnd", "bitand"),
        BinOp::BitOr(_) => ("BitOr", "bitor"),
        BinOp::Shl(_) => ("Shl", "shl"),
        BinOp::Shr(_) => ("Shr", "shr"),

        _ => return None,
    })
}

const UNSUPPORTED_OPERATION: &str = "Unsupported operation";

pub(crate) fn implementation(
    expression: Expr,
    trait_prefix: &str,
    method_prefix: &str,
    supported_trait_suffixes: &[&str],
) -> TokenStream {
    convert_expression(
        &expression,
        trait_prefix,
        method_prefix,
        supported_trait_suffixes,
    )
    .unwrap_or_else(|error| Error::new(expression.span(), error).to_compile_error())
}

fn convert_expression(
    expression: &Expr,
    trait_prefix: &str,
    method_prefix: &str,
    supported_trait_suffixes: &[&str],
) -> Result<TokenStream> {
    // TODO: recurse
    let arguments: Vec<&Expr>;

    let trait_suffix;
    let method_suffix;

    let generic;

    match expression {
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            arguments = vec![left, right];
            (trait_suffix, method_suffix) =
                trait_method_suffix(op).ok_or(anyhow!(UNSUPPORTED_OPERATION))?;
            generic = None;
        }
        Expr::Cast(ExprCast { expr, ty, .. }) => {
            arguments = vec![expr];
            trait_suffix = "Cast";
            method_suffix = "cast";
            generic = Some(ty);
        }
        Expr::Unary(ExprUnary { op, expr, .. }) => {
            arguments = vec![expr];
            ensure!(matches!(op, UnOp::Neg(_)), UNSUPPORTED_OPERATION);
            trait_suffix = "Neg";
            method_suffix = "neg";
            generic = None;
        }
        _ => bail!(UNSUPPORTED_OPERATION),
    };

    if !supported_trait_suffixes.contains(&trait_suffix) {
        bail!("Unsupported operation")
    }

    let trait_name = format_ident!("{trait_prefix}{trait_suffix}");
    let method_name = format_ident!("{method_prefix}_{method_suffix}");

    Ok(quote! {
        calm_ops::#trait_name::<#generic>::#method_name(#(#arguments),*)
    })
}
