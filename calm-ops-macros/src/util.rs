use syn::BinOp;

pub(crate) fn trait_name(operation: &BinOp) -> Option<&'static str> {
    Some(match operation {
        BinOp::Add(_) => "Add",
        BinOp::Sub(_) => "Sub",
        BinOp::Mul(_) => "Mul",
        BinOp::Div(_) => "Div",
        BinOp::Rem(_) => "Rem",
        BinOp::BitXor(_) => "BitXor",
        BinOp::BitAnd(_) => "BitAnd",
        BinOp::BitOr(_) => "BitOr",
        BinOp::Shl(_) => "Shl",
        BinOp::Shr(_) => "Shr",

        BinOp::AddAssign(_) => "AddAssign",
        BinOp::SubAssign(_) => "SubAssign",
        BinOp::MulAssign(_) => "MulAssign",
        BinOp::DivAssign(_) => "DivAssign",
        BinOp::RemAssign(_) => "RemAssign",
        BinOp::BitXorAssign(_) => "BitXorAssign",
        BinOp::BitAndAssign(_) => "BitAndAssign",
        BinOp::BitOrAssign(_) => "BitOrAssign",
        BinOp::ShlAssign(_) => "ShlAssign",
        BinOp::ShrAssign(_) => "ShrAssign",

        _ => return None
    })
}

pub(crate) fn method_name(operation: &BinOp) -> Option<&'static str> {
    Some(match operation {
        BinOp::Add(_) => "add",
        BinOp::Sub(_) => "sub",
        BinOp::Mul(_) => "mul",
        BinOp::Div(_) => "div",
        BinOp::Rem(_) => "rem",
        BinOp::BitXor(_) => "bitor",
        BinOp::BitAnd(_) => "bitand",
        BinOp::BitOr(_) => "bitor",
        BinOp::Shl(_) => "shl",
        BinOp::Shr(_) => "shr",

        BinOp::AddAssign(_) => "add_assign",
        BinOp::SubAssign(_) => "sub_assign",
        BinOp::MulAssign(_) => "mul_assign",
        BinOp::DivAssign(_) => "div_assign",
        BinOp::RemAssign(_) => "rem_assign",
        BinOp::BitXorAssign(_) => "bitxor_assign",
        BinOp::BitAndAssign(_) => "bitand_assign",
        BinOp::BitOrAssign(_) => "bitor_assign",
        BinOp::ShlAssign(_) => "shl_assign",
        BinOp::ShrAssign(_) => "shr_assign",

        _ => return None
    })
}