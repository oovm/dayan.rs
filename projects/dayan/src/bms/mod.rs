use crate::DayanError;
use latexify::Latexify;
use ndarray::Array2;
use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Formatter, Write},
    num::NonZeroUsize,
};

mod display;
mod parser;

/// Get the number of rows in the matrix
#[derive(Clone)]
pub struct BashicuMatrixSystem {
    // TODO: use nd array
    matrix: Vec<Vec<u32>>,
}

/// A configuration for the BMS
#[derive(Copy, Clone, Debug)]
pub struct BMSConfig {
    /// Whether to use color
    pub color: bool,
    /// Whether to display the matrix
    pub display: bool,
    /// Whether to use ellipsis
    pub ellipsis: bool,
}

impl Default for BMSConfig {
    fn default() -> Self {
        Self { color: false, display: false, ellipsis: false }
    }
}

impl BashicuMatrixSystem {
    /// Get the number of rows in the matrix
    pub fn new<I>(s: I) -> Result<Self, DayanError>
    where
        I: IntoIterator<Item = Vec<u32>>,
    {
        let mut out = Self { matrix: s.into_iter().collect() };
        // must be called in order!
        unsafe {
            out.fill_align()?;
            out.fill_zero()?;
        }
        Ok(out)
    }
    /// Get the number of rows in the matrix
    pub fn set_expand_steps(&mut self, steps: usize) -> bool {
        match NonZeroUsize::new(steps) {
            Some(s) => {
                // self.expand = s;
                true
            }
            None => false,
        }
    }

    /// Get the number of rows in the matrix
    pub fn with_expand_steps(mut self, steps: usize) -> Self {
        self.set_expand_steps(steps);
        self
    }
    /// Fill homogeneous columns based on first number
    ///
    /// Make sure the first column is not empty
    unsafe fn fill_align(&mut self) -> Result<(), DayanError> {
        match self.matrix.get(0) {
            Some(s) if s.len() == 0 => {
                Err(DayanError::too_less_argument("The first column of BMS cannot be empty", 0).with_min_argument(1))?
            }
            Some(s) => {
                let len = s.len();
                let head = s.get_unchecked(0);
                if *head != 0 {
                    for i in (0..*head).rev() {
                        self.matrix.insert(0, vec![i; len]);
                    }
                }
            }
            None => Err(DayanError::too_less_argument("This is an empty BMS", 0).with_min_argument(1))?,
        }
        Ok(())
    }
    /// Fill zeros by first column, must call after [`BashicuMatrixSystem::fill_align`]!!!
    unsafe fn fill_zero(&mut self) -> Result<(), DayanError> {
        let count = self.matrix.get_unchecked(0).len();
        for column in self.matrix.iter_mut().skip(1) {
            if column.len() > count {
                Err(DayanError::too_less_argument(
                    "The remaining columns of BMS cannot exceed the length of the first column",
                    count,
                )
                .with_min_argument(0)
                .with_max_argument(count as u32))?
            }
            while column.len() < count {
                column.push(0);
            }
        }
        Ok(())
    }
    /// Get the number of rows in the matrix
    pub fn expand(&self, steps: usize) -> Self {
        let s = &self.matrix;
        let xs = self.term();
        let ys = self.rank();
        let s1 = self.matrix[..xs - 1].to_vec();
        let r = match self.get_bad_root() {
            Some(r) => r,
            None => return Self { matrix: s1 },
        };
        let mut delta = diff(&s[xs - 1], &s[r]);
        let lmnz = match self.get_lowermost_nonzero(&s[xs - 1]) {
            Some(s) => s,
            None => return Self { matrix: s1 },
        };
        for y in lmnz..ys {
            delta[y] = 0;
        }
        let a = self.get_ascension();
        let bs = xs - r - 1;
        let mut s1 = s1;
        for i in 0..steps {
            for x in 0..bs {
                let mut da = vec![0; ys];
                for y in 0..ys {
                    da[y] = s[r + x][y] + delta[y] * a[x][y] * (i + 1) as u32;
                }
                s1.push(da);
            }
        }
        Self { matrix: s1 }
    }

    fn get_parent(&self, x: usize, y: usize) -> Option<usize> {
        let mut p = x;
        while p > 0 {
            if y != 0 {
                p = self.get_parent(p, y - 1)?;
            }
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
        let xs = self.term();
        let x = xs - 1;
        let y = self.get_lowermost_nonzero(&self.matrix[x])?;
        let p = self.get_parent(x, y)?;
        Some(p)
    }

    fn get_ascension(&self) -> Vec<Vec<u32>> {
        let xs = self.term();
        let ys = self.rank();
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

    fn get_lowermost_nonzero(&self, c: &[u32]) -> Option<usize> {
        for (y, &elem) in c.iter().enumerate().rev() {
            if elem > 0 {
                return Some(y);
            }
        }
        None
    }

    fn term(&self) -> usize {
        self.matrix.len()
    }

    fn rank(&self) -> usize {
        self.matrix[0].len()
    }
}

impl BashicuMatrixSystem {
    /// Convert the BMS to 0-Y Sequence
    pub fn as_y_sequence(&self) -> Vec<u32> {
        let xs = self.term();
        let ys = self.rank();
        let mut parent_matrix: Vec<Vec<u32>> = Vec::new();
        for j in 0..ys {
            for i in 0..xs {
                let s = (0..=i).rev().find(|&p| self.matrix[p][j] < self.matrix[i][j]).map(|p| p as u32);
                let p = if j == 0 {
                    parent_matrix.push(Vec::new());
                    s.unwrap_or(0)
                }
                else {
                    s.unwrap_or(parent_matrix[i][j - 1])
                };
                parent_matrix[i].push(p);
            }
        }
        let mut y: Vec<u32> = vec![1; xs];
        for j in (0..ys).rev() {
            for i in 0..xs {
                y[i] = if self.matrix[i][j] == 0 { 1 } else { y[i] + y[parent_matrix[i][j] as usize] };
            }
        }
        y
    }
}

fn diff(a: &[u32], b: &[u32]) -> Vec<u32> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x - y).collect()
}
