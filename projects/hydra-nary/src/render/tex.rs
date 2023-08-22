use super::*;

impl Latexify for NAryHydra {
    type Context = NAryHydraTeX;

    fn fmt<W: Write>(&self, context: &Self::Context, f: &mut W) -> std::fmt::Result {
        match context.function {
            Some(c) => match self {
                NAryHydra::Head { order, .. } => f.write_fmt(format_args!("{}", order))?,
                NAryHydra::Body { ranks, terms, .. } => {
                    f.write_char(c)?;
                    match ranks.len() {
                        0 => {}
                        1 => {
                            f.write_fmt(format_args!("_{{{}}}", ranks[0]))?;
                        }
                        _ => {
                            f.write_char('_')?;
                            f.write_char('{')?;
                            for (index, rank) in ranks.iter().enumerate() {
                                if index != 0 {
                                    f.write_str(", ")?;
                                }
                                f.write_fmt(format_args!("{}", rank))?;
                            }
                            f.write_char('}')?;
                        }
                    }
                    f.write_char(c)?;
                    f.write_char('(')?;
                    for (index, term) in terms.iter().enumerate() {
                        if index != 0 {
                            f.write_str(", ")?;
                        }
                        Latexify::fmt(term, context, f)?;
                    }
                    f.write_char(')')?
                }
            },
            None => f.write_fmt(format_args!("{}", self))?,
        }
        Ok(())
    }
}
