use std::{
    fmt::{Display, Formatter},
    num::NonZeroUsize,
};

#[derive(Clone)]
struct BashicuMatrixSystem {
    matrix: Vec<Vec<usize>>,
    expand: NonZeroUsize,
}

impl BashicuMatrixSystem {
    pub fn new(s: Vec<Vec<usize>>) -> BashicuMatrixSystem {
        BashicuMatrixSystem { matrix: s, expand: unsafe { NonZeroUsize::new_unchecked(2) } }
    }
    pub fn with_expand_steps(mut self, steps: NonZeroUsize) -> BashicuMatrixSystem {
        self.expand = steps;
        self
    }

    fn expand(&self) -> BashicuMatrixSystem {
        let s = &self.matrix;
        let xs = self.xs();
        let ys = self.ys();
        let s1 = self.matrix[..xs - 1].to_vec();
        let r = match self.get_bad_root() {
            Some(r) => r,
            None => return BashicuMatrixSystem::new(s1).with_expand_steps(self.expand),
        };
        let mut delta = diff(&s[xs - 1], &s[r]);
        let lmnz = match self.get_lowermost_nonzero(&s[xs - 1]) {
            Some(s) => s,
            None => return BashicuMatrixSystem::new(s1).with_expand_steps(self.expand),
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
        BashicuMatrixSystem::new(s1).with_expand_steps(self.expand)
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
        result.push_str(&format!("[{}]", self.expand));
        write!(f, "{}", result)
    }
}

fn diff(a: &[usize], b: &[usize]) -> Vec<usize> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x - y).collect()
}

#[test]
fn test() {
    //  (0,0,0)(1,1,1)(2,1,0)(1,1,0)(2,2,1)(3,1,0)(2,2,1)
    let sequence =
        vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 1, 0], vec![1, 1, 0], vec![2, 2, 1], vec![3, 1, 0], vec![2, 2, 1]];
    let bms = BashicuMatrixSystem::new(sequence.clone());
    println!("{}", bms);
    let bms = BashicuMatrixSystem::new(sequence.clone()).expand();
    println!("{}", bms);
    let bms = bms.expand();
    println!("{}", bms);
    let bms = bms.expand();
    println!("{}", bms);
}
