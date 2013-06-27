use super::*;

impl Debug for DayanPsi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanPsi::Number(v) => f.write_fmt(format_args!("{}", v)),
            DayanPsi::Omega => f.write_str("(0)"),
            DayanPsi::Psi(v) => {
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

impl Display for DayanPsi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayanPsi::Number(v) => f.write_fmt(format_args!("{}", v)),
            DayanPsi::Omega => f.write_char('ω'),
            DayanPsi::Psi(v) => {
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
