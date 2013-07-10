use super::*;

impl Debug for BashicuMatrixSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for c in &self.matrix {
            result.push('(');
            for (i, &r) in c.iter().enumerate() {
                result.push_str(&r.to_string());
                if i != c.len() - 1 {
                    result.push(',');
                }
            }
            result.push(')');
        }
        result.push_str(&format!("[{}]", self.expand));
        write!(f, "{}", result)
    }
}

impl Display for BashicuMatrixSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for c in &self.matrix {
            result.push('(');
            for (i, &r) in c.iter().enumerate() {
                result.push_str(&r.to_string());
                if i != c.len() - 1 {
                    result.push(',');
                }
            }
            result.push(')');
        }
        write!(f, "{}", result)
    }
}

impl Latexify for BashicuMatrixSystem {
    type Context = BMSConfig;
    fn fmt<W: Write>(&self, c: &Self::Context, f: &mut W) -> std::fmt::Result {
        if c.display {
            for coloum in &self.matrix {
                f.write_char('(')?;
                for (i, &r) in coloum.iter().enumerate() {
                    if i != 0 {
                        f.write_str(",")?;
                    }
                    Latexify::fmt(&r, &(), f)?;
                }
                f.write_str(")")?;
            }
        }
        else {
            f.write_str("\\begin{bmatrix}\n")?;
            for i in 0..self.ys() {
                for j in 0..self.xs() {
                    if c.color {
                        // if self.get_bad_root() == Some(i) {
                        //     f.write_str("\\color{red}")?;
                        // }
                        // else if self.get_lowermost_nonzero(&self.matrix[j]) == Some(i) {
                        //     f.write_str("\\color{blue}")?;
                        // }
                    }
                    Latexify::fmt(&self.matrix[j][i], &(), f)?;
                    if j != self.xs() - 1 {
                        f.write_str(" & ")?;
                    }
                }
                f.write_str("\\\\\n")?;
            }
            f.write_str("\\end{bmatrix}")?;
        }
        Ok(())
    }
}

impl BMSConfig {
    /// Get the number of rows in the matrix
    pub fn render(&self, bms: &BashicuMatrixSystem) -> String {
        bms.to_latex(self)
    }
}

impl BashicuMatrixSystem {
    fn max_width(&self) -> usize {
        self.matrix.iter().map(|c| c.len()).max().unwrap_or(0)
    }
}
