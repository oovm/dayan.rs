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

#[cfg(test)]
mod exports {
    use crate::{DayanAlpha, DayanError};
    use std::{fs::File, io::Write, path::Path, str::FromStr};

    pub fn markdown(input: &str, file: &mut File) -> Result<(), DayanError> {
        writeln!(file, "| Node | Psi | Expression |")?;
        writeln!(file, "| ---- | --- | ---------- |")?;
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let node = DayanAlpha::from_str(line)?;
            let expr = node.as_expression()?;
            writeln!(file, "| ${:?}$ | ${}$ | ${}$ |", node, node, expr)?;
        }
        Ok(())
    }
    #[test]
    #[ignore]
    fn export_psi() -> Result<(), DayanError> {
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut file = File::create(here.join("src/alpha/alpha.md"))?;
        markdown(include_str!("alpha.txt"), &mut file)
    }
}
