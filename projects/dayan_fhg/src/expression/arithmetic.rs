use super::*;

impl From<char> for ExpressionTree {
    fn from(value: char) -> Self {
        ExpressionTree::Letter(value)
    }
}

impl From<u32> for ExpressionTree {
    fn from(value: u32) -> Self {
        ExpressionTree::Number(value)
    }
}

impl Add for ExpressionTree {
    type Output = Self;

    fn add(self, rhs: ExpressionTree) -> Self::Output {
        if rhs.is_zero() {
            return self;
        }
        else {
            ExpressionTree::Add { lhs: Box::new(self), rhs: Box::new(rhs) }
        }
    }
}

impl Mul for ExpressionTree {
    type Output = ExpressionTree;

    fn mul(self, rhs: ExpressionTree) -> Self::Output {
        match rhs {
            _ if rhs.is_zero() => ExpressionTree::Number(0),
            _ if rhs.is_one() => self,
            _ => ExpressionTree::Mul { lhs: Box::new(self), rhs: Box::new(rhs) },
        }
    }
}

impl BitXor for ExpressionTree {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match rhs {
            _ if rhs.is_zero() => ExpressionTree::Number(1),
            _ if rhs.is_one() => self,
            _ => Self::Sup { base: Box::new(self), rest: Box::new(rhs) },
        }
    }
}
