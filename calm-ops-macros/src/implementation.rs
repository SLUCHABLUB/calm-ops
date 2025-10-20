use anyhow::{Result, bail, ensure};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned;
use syn::{BinOp, Error, Expr, ExprBinary, ExprCast, ExprReference, ExprUnary, Token, UnOp};

struct Operator {
    trait_suffix: &'static str,
    method_suffix: &'static str,
    is_assignment: bool,
}

impl Operator {
    const CAST: Operator = Operator {
        trait_suffix: "Cast",
        method_suffix: "cast",
        is_assignment: false,
    };
    const NEG: Operator = Operator {
        trait_suffix: "Neg",
        method_suffix: "neg",
        is_assignment: false,
    };

    fn new(operation: BinOp) -> Result<Operator> {
        Ok(match operation {
            BinOp::Add(_) => Operator {
                trait_suffix: "Add",
                method_suffix: "add",
                is_assignment: false,
            },
            BinOp::Sub(_) => Operator {
                trait_suffix: "Sub",
                method_suffix: "sub",
                is_assignment: false,
            },
            BinOp::Mul(_) => Operator {
                trait_suffix: "Mul",
                method_suffix: "mul",
                is_assignment: false,
            },
            BinOp::Div(_) => Operator {
                trait_suffix: "Div",
                method_suffix: "div",
                is_assignment: false,
            },
            BinOp::Rem(_) => Operator {
                trait_suffix: "Rem",
                method_suffix: "rem",
                is_assignment: false,
            },
            BinOp::Shl(_) => Operator {
                trait_suffix: "Shl",
                method_suffix: "shl",
                is_assignment: false,
            },
            BinOp::Shr(_) => Operator {
                trait_suffix: "Shr",
                method_suffix: "shr",
                is_assignment: false,
            },

            BinOp::AddAssign(_) => Operator {
                trait_suffix: "AddAssign",
                method_suffix: "add_assign",
                is_assignment: true,
            },
            BinOp::SubAssign(_) => Operator {
                trait_suffix: "SubAssign",
                method_suffix: "sub_assign",
                is_assignment: true,
            },
            BinOp::MulAssign(_) => Operator {
                trait_suffix: "MulAssign",
                method_suffix: "mul_assign",
                is_assignment: true,
            },
            BinOp::DivAssign(_) => Operator {
                trait_suffix: "DivAssign",
                method_suffix: "div_assign",
                is_assignment: true,
            },
            BinOp::RemAssign(_) => Operator {
                trait_suffix: "RemAssign",
                method_suffix: "rem_assign",
                is_assignment: true,
            },
            BinOp::ShlAssign(_) => Operator {
                trait_suffix: "ShlAssign",
                method_suffix: "shl_assign",
                is_assignment: true,
            },
            BinOp::ShrAssign(_) => Operator {
                trait_suffix: "ShrAssign",
                method_suffix: "shr_assign",
                is_assignment: true,
            },

            _ => bail!("unsupported operator: `{}`", operation.to_token_stream()),
        })
    }
}

pub(crate) fn implementation(
    expression: Expr,
    trait_prefix: &str,
    method_prefix: &str,
) -> TokenStream {
    let span = expression.span();

    convert_expression(expression, trait_prefix, method_prefix)
        .unwrap_or_else(|error| Error::new(span, error).to_compile_error())
}

fn convert_expression(
    expression: Expr,
    trait_prefix: &str,
    method_prefix: &str,
) -> Result<TokenStream> {
    let mut arguments: Vec<Expr>;

    let operator;
    let generic;

    match expression {
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            arguments = vec![*left, *right];
            operator = Operator::new(op)?;
            generic = None;
        }
        Expr::Cast(ExprCast { expr, ty, .. }) => {
            arguments = vec![*expr];
            operator = Operator::CAST;
            generic = Some(ty);
        }
        Expr::Unary(ExprUnary { op, expr, .. }) => {
            arguments = vec![*expr];
            ensure!(
                matches!(op, UnOp::Neg(_)),
                "unsupported unary operator: `{}`",
                op.to_token_stream()
            );
            operator = Operator::NEG;
            generic = None;
        }
        _ => bail!("expected an operation or a cast"),
    };

    let trait_name = format_ident!("{trait_prefix}{}", operator.trait_suffix);
    let method_name = format_ident!("{method_prefix}_{}", operator.method_suffix);

    if operator.is_assignment {
        let span = arguments[0].span();

        arguments[0] = Expr::Reference(ExprReference {
            attrs: Vec::new(),
            and_token: Token![&](span),
            mutability: Some(Token![mut](span)),
            expr: Box::new(arguments[0].clone()),
        });
    }

    // TODO: use $crate
    Ok(quote! {
        calm_ops::#trait_name::<#generic>::#method_name(#(#arguments),*)
    })
}
