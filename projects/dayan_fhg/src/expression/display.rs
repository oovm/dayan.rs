use super::*;

impl Display for ExpressionTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionTree::Number(v) => f.write_fmt(format_args!("{}", v)),
            ExpressionTree::Letter(v) => f.write_fmt(format_args!("{}", v)),
            ExpressionTree::Add { lhs, rhs } => {
                Display::fmt(lhs, f)?;
                f.write_str(" + ")?;
                Display::fmt(rhs, f)
            }
            ExpressionTree::Mul { lhs, rhs } => {
                Display::fmt(lhs, f)?;
                f.write_str(" × ")?;
                Display::fmt(rhs, f)
            }
            ExpressionTree::Sup { head, rest } => {
                Display::fmt(head, f)?;
                f.write_str("^{")?;
                Display::fmt(rest, f)?;
                f.write_char('}')
            }
            ExpressionTree::Sub { head, rest } => {
                Display::fmt(head, f)?;
                f.write_str("_{")?;
                Display::fmt(rest, f)?;
                f.write_char('}')
            }
        }
    }
}
