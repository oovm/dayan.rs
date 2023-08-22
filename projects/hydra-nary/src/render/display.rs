use super::*;

impl Display for NAryHydra {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Head { order, .. } => f.write_fmt(format_args!("{}", order)),
            Self::Body { ranks, terms, .. } => {
                if f.alternate() {
                    if !ranks.is_empty() {
                        f.write_char('[')?;
                        write_ranks(ranks, f)?;
                        f.write_char(']')?;
                    }
                    f.write_char('(')?;
                }
                else {
                    f.write_char('(')?;
                    if !ranks.is_empty() {
                        write_ranks(ranks, f)?;
                        f.write_char(';')?;
                        if !terms.is_empty() {
                            f.write_char(' ')?;
                        }
                    }
                }
                write_ranks(terms, f)?;
                f.write_char(')')
            }
        }
    }
}

fn write_ranks<T: Display>(items: &[T], f: &mut Formatter<'_>) -> std::fmt::Result {
    for (index, rank) in items.iter().enumerate() {
        if index != 0 {
            f.write_str(", ")?;
        }
        Display::fmt(rank, f)?;
    }
    Ok(())
}
