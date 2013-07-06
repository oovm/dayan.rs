use std::fmt;

struct Bms {
    s: Vec<Vec<i32>>,
    b: Option<i32>,
    f: Option<fn(i32) -> i32>,
}

impl Bms {
    fn new(s: Vec<Vec<i32>>, b: Option<i32>, f: Option<fn(i32) -> i32>) -> Bms {
        Bms { s, b, f }
    }

    fn inc_bracket(&self) -> i32 {
        match self.f {
            Some(f) => (f)(self.b.unwrap()),
            None => self.b.unwrap(),
        }
    }

    fn expand(&self) -> Bms {
        let s = &self.s;
        let xs = self.xs();
        let ys = self.ys();

        let b1 = self.inc_bracket();
        let s1 = s.iter().map(|row| row[..xs - 1].to_vec()).collect::<Vec<_>>();

        let r = self.get_bad_root();
        if r == -1 {
            return Bms::new(s1, Some(b1), self.f);
        }

        let mut delta = sub(&s[xs - 1], &s[r as usize]);
        let lmnz = self.get_lowermost_nonzero(&s[xs - 1]);
        for y in lmnz..ys as i32 {
            delta[y as usize] = 0;
        }

        let a = self.get_ascension();
        let bs = xs - r as usize - 1;

        let mut s1 = s1;
        for i in 0..self.b.unwrap() {
            for x in 0..bs {
                let mut da = vec![0; ys];
                for y in 0..ys {
                    da[y] = s[r as usize + x][y] + delta[y] * a[x][y] * (i + 1);
                }
                s1.push(da);
            }
        }

        Bms::new(s1, Some(b1), self.f)
    }

    fn get_parent(&self, x: usize, y: usize) -> i32 {
        let mut p = x as i32;
        while p > 0 {
            if y != 0 {
                p = self.get_parent(p as usize, y - 1);
            }
            else {
                p -= 1;
            }
            if p == -1 {
                return p;
            }
            if self.s[p as usize][y] < self.s[x][y] {
                return p;
            }
        }
        -1
    }

    fn get_bad_root(&self) -> i32 {
        let xs = self.xs();
        let x = xs - 1;
        let y = self.get_lowermost_nonzero(&self.s[x]);
        if y == -1 {
            return -1;
        }
        self.get_parent(x, y as usize)
    }

    fn get_ascension(&self) -> Vec<Vec<i32>> {
        let xs = self.xs();
        let ys = self.ys();
        let r = self.get_bad_root();
        if r == -1 {
            return Vec::new();
        }
        let bs = xs - r as usize - 1;
        let mut a = vec![vec![0; ys]; bs];
        for y in 0..ys {
            a[0][y] = 1;
            for x in 1..bs {
                let p = self.get_parent(x + r as usize, y);
                if p != -1 && a[(p - r) as usize][y] == 1 {
                    a[x][y] = 1;
                }
            }
        }
        a
    }

    fn get_lowermost_nonzero(&self, c: &[i32]) -> i32 {
        for (y, &elem) in c.iter().enumerate().rev() {
            if elem > 0 {
                return y as i32;
            }
        }
        -1
    }

    fn xs(&self) -> usize {
        self.s.len()
    }

    fn ys(&self) -> usize {
        self.s[0].len()
    }
}

impl fmt::Display for Bms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for c in &self.s {
            result.push('(');
            for (i, &r) in c.iter().enumerate() {
                result.push_str(&r.to_string());
                if i != c.len() - 1 {
                    result.push(',');
                }
            }
            result.push(')');
        }
        if let Some(b) = self.b {
            result.push_str(&format!("[{}]", b));
        }
        write!(f, "{}", result)
    }
}

fn sub(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x - y).collect()
}

#[test]
fn test() {
    ///  (0,0,0)(1,1,1)(2,1,0)(1,1,0)(2,2,1)(3,1,0)(2,2,1)
    let sequence =
        vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 1, 0], vec![1, 1, 0], vec![2, 2, 1], vec![3, 1, 0], vec![2, 2, 1]];
    let b = Some(3);
    let bms = Bms::new(sequence, b, Some(|x| x));
    println!("{}", bms);
    let bms = bms.expand();
    println!("{}", bms);
}
