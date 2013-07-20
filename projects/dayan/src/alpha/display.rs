use super::*;

impl Debug for DayanAlpha {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanAlpha::Number(v) => f.write_fmt(format_args!("{}", v)),
            DayanAlpha::Omega => f.write_str("(0)"),
            DayanAlpha::Psi(v) => {
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

impl Display for DayanAlpha {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanAlpha::Number(v) => f.write_fmt(format_args!("{}", v)),
            DayanAlpha::Omega => f.write_char('ω'),
            DayanAlpha::Psi(v) => {
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
