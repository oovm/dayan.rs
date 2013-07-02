use super::*;

impl Display for ExpressionTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionTree::Number(v) => f.write_fmt(format_args!("{}", v))?,
            ExpressionTree::Letter(v) => f.write_fmt(format_args!("{}", v))?,
            ExpressionTree::Sum { add } => {
                for (i, term) in add.iter().enumerate() {
                    if i != 0 {
                        f.write_str(" + ")?;
                    }
                    Display::fmt(term, f)?;
                }
            }
            ExpressionTree::Product { mul } => {
                for (i, term) in mul.iter().enumerate() {
                    if i != 0 {
                        f.write_str(" × ")?;
                    }
                    Display::fmt(term, f)?;
                }
            }
            ExpressionTree::Sup { base: head, rest } => {
                Display::fmt(head, f)?;
                f.write_char('^')?;
                write_bracketed(f, rest)?
            }
            ExpressionTree::Sub { head, rest } => {
                Display::fmt(head, f)?;
                f.write_char('_')?;
                write_bracketed(f, rest)?
            }
        }
        Ok(())
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
