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
            ExpressionTree::Sup { base: head, rest } => {
                Display::fmt(head, f)?;
                f.write_char('^')?;
                write_bracketed(f, rest)
            }
            ExpressionTree::Sub { head, rest } => {
                Display::fmt(head, f)?;
                f.write_char('_')?;
                write_bracketed(f, rest)
            }
        }
    }
}

fn write_bracketed(f: &mut Formatter<'_>, rest: &ExpressionTree) -> std::fmt::Result {
    if rest.is_digit() {
        Display::fmt(rest, f)
    }
    else {
        f.write_str("{")?;
        Display::fmt(rest, f)?;
        f.write_char('}')
    }
}
