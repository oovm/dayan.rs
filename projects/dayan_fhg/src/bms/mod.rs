use latexify::Latexify;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    num::NonZeroUsize,
};

mod display;
mod parser;

/// Get the number of rows in the matrix
#[derive(Clone)]
pub struct BashicuMatrixSystem {
    // TODO: use nd array
    matrix: Vec<Vec<usize>>,
    expand: NonZeroUsize,
}

/// A configuration for the BMS
#[derive(Copy, Clone, Debug)]
pub struct BMSConfig {
    pub color: bool,
    pub display: bool,
    pub ellipsis: bool,
}

impl Default for BMSConfig {
    fn default() -> Self {
        Self { color: false, display: false, ellipsis: false }
    }
}

impl BashicuMatrixSystem {
    /// Get the number of rows in the matrix
    pub fn new(s: Vec<Vec<usize>>) -> Self {
        Self { matrix: s, expand: unsafe { NonZeroUsize::new_unchecked(2) } }
    }
    /// Get the number of rows in the matrix
    pub fn with_expand_steps(mut self, steps: NonZeroUsize) -> Self {
        self.expand = steps;
        self
    }
    /// Get the number of rows in the matrix
    pub fn expand(&self) -> Self {
        let s = &self.matrix;
        let xs = self.xs();
        let ys = self.ys();
        let s1 = self.matrix[..xs - 1].to_vec();
        let r = match self.get_bad_root() {
            Some(r) => r,
            None => return Self { matrix: s1, expand: self.expand },
        };
        let mut delta = diff(&s[xs - 1], &s[r]);
        let lmnz = match self.get_lowermost_nonzero(&s[xs - 1]) {
            Some(s) => s,
            None => return Self { matrix: s1, expand: self.expand },
        };
        for y in lmnz..ys {
            delta[y] = 0;
        }
        let a = self.get_ascension();
        let bs = xs - r - 1;
        let mut s1 = s1;
        for i in 0..self.expand.get() {
            for x in 0..bs {
                let mut da = vec![0; ys];
                for y in 0..ys {
                    da[y] = s[r + x][y] + delta[y] * a[x][y] * (i + 1);
                }
                s1.push(da);
            }
        }
        Self { matrix: s1, expand: self.expand }
    }

    fn get_parent(&self, x: usize, y: usize) -> Option<usize> {
        let mut p = x;
        while p > 0 {
            if y != 0 {
                p = self.get_parent(p, y - 1)?;
            }
            // else if p == 0 {
            //     return None;
            // }
            else {
                p -= 1
            }
            if self.matrix[p][y] < self.matrix[x][y] {
                return Some(p);
            }
        }
        None
    }

    fn get_bad_root(&self) -> Option<usize> {
        let xs = self.xs();
        let x = xs - 1;
        let y = self.get_lowermost_nonzero(&self.matrix[x])?;
        let p = self.get_parent(x, y)?;
        Some(p)
    }

    fn get_ascension(&self) -> Vec<Vec<usize>> {
        let xs = self.xs();
        let ys = self.ys();
        let r = match self.get_bad_root() {
            Some(r) => r,
            None => return vec![],
        };
        let bs = xs - r - 1;
        let mut a = vec![vec![0; ys]; bs];
        for y in 0..ys {
            a[0][y] = 1;
            for x in 1..bs {
                let p = match self.get_parent(x + r, y) {
                    Some(s) => s,
                    None => continue,
                };
                // FIXME: Strange case
                // println!("p: {}, r: {}", p, r);
                if p < r {
                    continue;
                }
                if a[p - r][y] == 1 {
                    a[x][y] = 1;
                }
            }
        }
        a
    }

    fn get_lowermost_nonzero(&self, c: &[usize]) -> Option<usize> {
        for (y, &elem) in c.iter().enumerate().rev() {
            if elem > 0 {
                return Some(y);
            }
        }
        None
    }

    fn xs(&self) -> usize {
        self.matrix.len()
    }

    fn ys(&self) -> usize {
        self.matrix[0].len()
    }
}

fn diff(a: &[usize], b: &[usize]) -> Vec<usize> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x - y).collect()
}
