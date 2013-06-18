use super::*;

impl Debug for DayanPsi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanPsi::Number(v) => {
                f.write_fmt(format_args!("{}", v))
            }
            DayanPsi::Omega => {
                f.write_str("ϕ(0)")
            }
            DayanPsi::Psi(v) => {
                f.write_str("ϕ(")?;
                for (index, node) in v.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    Debug::fmt(node, f)?;
                }
                f.write_char(')')
            }
        }
    }
}


impl Display for DayanPsi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanPsi::Omega => {
                f.write_char('ω')
            }
            _ => Debug::fmt(self, f),
        }
    }
}

impl Display for ExpressionTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionTree::Number(v) => {
                f.write_fmt(format_args!("{}", v))
            }
            ExpressionTree::Letter(v) => {
                f.write_fmt(format_args!("{}", v))
            }
            ExpressionTree::Add { lhs, rhs } => {
                Display::fmt(lhs, f)?;
                f.write_str("+")?;
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