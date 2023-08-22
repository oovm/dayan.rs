use super::*;

impl Latexify for NAryHydra {
    type Context = NAryHydraTeX;

    fn fmt<W: Write>(&self, context: &Self::Context, f: &mut W) -> std::fmt::Result {
        match context.function {
            Some(c) => match self {
                NAryHydra::Head { order, .. } => match *order {
                    0 => {
                        f.write_str(&context.placeholder)?;
                    }
                    _ => f.write_fmt(format_args!("{}", order))?,
                },
                NAryHydra::Body { ranks, terms, .. } => {
                    f.write_char(c)?;
                    match ranks.as_slice() {
                        [] => {}
                        [item] if *item < 10 => {
                            f.write_fmt(format_args!("_{}", item))?;
                        }
                        [item] => {
                            f.write_fmt(format_args!("_{{{}}}", item))?;
                        }
                        _ => {
                            f.write_str("_{[")?;
                            for (index, rank) in ranks.iter().enumerate() {
                                if index != 0 {
                                    f.write_str(", ")?;
                                }
                                f.write_fmt(format_args!("{}", rank))?;
                            }
                            f.write_str("]}")?;
                        }
                    }
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
