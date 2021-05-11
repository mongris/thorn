use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Power,
    BitAnd,
    BitOr,
    BitXor,
    BitshiftLeft,
    BitshiftRight,
    IsDistinctFrom,
    IsNotDistinctFrom,
    Concat,
    LogicalAnd,
    LogicalOr,
}

#[rustfmt::skip]
pub trait BinaryExt: Expr + Sized {
    #[inline]
    fn add<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Add }
    }
    #[inline]
    fn sub<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Sub }
    }
    #[inline]
    fn mul<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Mul }
    }
    #[inline]
    fn div<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Div }
    }
    #[inline]
    fn rem<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Rem }
    }
    #[inline]
    fn power<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Power }
    }
    #[inline]
    fn bit_and<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::BitAnd }
    }
    #[inline]
    fn bit_or<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::BitOr }
    }
    #[inline]
    fn bit_xor<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::BitXor }
    }
    #[inline]
    fn bitshift_left<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::BitshiftLeft }
    }
    #[inline]
    fn bitshift_right<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::BitshiftRight }
    }
    #[inline]
    fn is_distinct_from<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::IsDistinctFrom }
    }
    #[inline]
    fn is_not_distinct_from<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::IsNotDistinctFrom }
    }
    #[inline]
    fn concat<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::Concat }
    }
    #[inline]
    fn and<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::LogicalAnd }
    }
    #[inline]
    fn or<Rhs>(self, rhs: Rhs) -> BinaryExpr<Self, Rhs> {
        BinaryExpr { lhs: self, rhs, op: BinaryOp::LogicalOr }
    }
}

impl<T> BinaryExt for T where T: Expr {}

pub struct BinaryExpr<Lhs, Rhs> {
    lhs: Lhs,
    rhs: Rhs,
    op: BinaryOp,
}

impl<Lhs: Expr, Rhs: Expr> Expr for BinaryExpr<Lhs, Rhs> {}
impl<Lhs: Expr, Rhs: Expr> Collectable for BinaryExpr<Lhs, Rhs> {
    fn needs_wrapping(&self) -> bool {
        true
    }

    fn collect(&self, w: &mut dyn Write, t: &mut Collector) -> fmt::Result {
        self.lhs._collect(w, t)?;

        w.write_str(" ")?;
        w.write_str(match self.op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Rem => "%",
            BinaryOp::Power => "^",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "#",
            BinaryOp::BitshiftLeft => "<<",
            BinaryOp::BitshiftRight => ">>",
            BinaryOp::IsDistinctFrom => "IS DISTINCT FROM",
            BinaryOp::IsNotDistinctFrom => "IS NOT DISTINCT FROM",
            BinaryOp::Concat => "||",
            BinaryOp::LogicalAnd => "AND",
            BinaryOp::LogicalOr => "OR",
        })?;

        w.write_str(" ")?;
        self.rhs._collect(w, t)
    }
}
