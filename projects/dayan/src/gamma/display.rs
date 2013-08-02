use super::*;

impl Debug for DayanGamma {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanGamma::Number(v) => f.write_fmt(format_args!("{}", v)),
            DayanGamma::Omega => f.write_str("(0)"),
            DayanGamma::Psi(v) => {
                f.write_str("(")?;
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

impl Display for DayanGamma {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanGamma::Number(v) => f.write_fmt(format_args!("{}", v)),
            DayanGamma::Omega => f.write_char('ω'),
            DayanGamma::Psi(v) => {
                f.write_str("φ(")?;
                for (index, node) in v.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    Display::fmt(node, f)?;
                }
                f.write_char(')')
            }
        }
    }
}
