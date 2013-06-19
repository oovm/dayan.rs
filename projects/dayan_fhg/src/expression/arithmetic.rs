use super::*;

impl AddAssign for ExpressionTree {
    fn add_assign(&mut self, rhs: Self) {
        *self = ExpressionTree::Add {
            lhs: Box::new(replace(self, ExpressionTree::Number(0))),
            rhs: Box::new(rhs),
        }
    }
}

impl MulAssign for ExpressionTree {
    fn mul_assign(&mut self, rhs: Self) {
        *self = ExpressionTree::Mul {
            lhs: Box::new(replace(self, ExpressionTree::Number(1))),
            rhs: Box::new(rhs),
        }
    }
}

impl BitXorAssign for ExpressionTree {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = ExpressionTree::Sup {
            head: Box::new(replace(self, ExpressionTree::Number(0))),
            rest: Box::new(rhs),
        }
    }
}