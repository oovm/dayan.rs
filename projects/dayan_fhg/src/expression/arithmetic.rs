use super::*;

impl Add for ExpressionTree {
    type Output = ExpressionTree;

    fn add(self, rhs: ExpressionTree) -> Self::Output {
        if rhs.is_zero() {
            return self;
        }
        else {
            ExpressionTree::Add { lhs: Box::new(self), rhs: Box::new(rhs) }
        }
    }
}

impl Add<u32> for ExpressionTree {
    type Output = ExpressionTree;

    fn add(self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            return self;
        }
        else {
            ExpressionTree::Add { lhs: Box::new(self), rhs: Box::new(ExpressionTree::Number(rhs)) }
        }
    }
}

impl Mul<u32> for ExpressionTree {
    type Output = ExpressionTree;

    fn mul(self, rhs: u32) -> Self::Output {
        match rhs {
            0 => ExpressionTree::Number(0),
            1 => self,
            _ => ExpressionTree::Mul { lhs: Box::new(self), rhs: Box::new(ExpressionTree::Number(rhs)) },
        }
    }
}

impl BitXorAssign for ExpressionTree {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = ExpressionTree::Sup { head: Box::new(replace(self, ExpressionTree::Number(0))), rest: Box::new(rhs) }
    }
}

impl BitXor for ExpressionTree {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
