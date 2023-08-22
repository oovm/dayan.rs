use super::*;

impl Display for NAryHydra {
    /// [ranks](terms)
    /// (ranks; terms)
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NAryHydra::Head { order, .. } => f.write_fmt(format_args!("{}", order))?,
            NAryHydra::Body { ranks, terms, .. } => {
                if !ranks.is_empty() {
                    f.write_char('[')?;
                    for (index, rank) in ranks.iter().enumerate() {
                        if index != 0 {
                            f.write_char(',')?;
                        }
                        f.write_fmt(format_args!("{}", rank))?;
                    }
                    f.write_char(']')?;
                }
                f.write_char('(')?;
                for (index, term) in terms.iter().enumerate() {
                    if index != 0 {
                        f.write_char(',')?;
                    }
                    Display::fmt(term, f)?;
                }
                f.write_char(')')?;
            }
        }
        Ok(())
    }
}
