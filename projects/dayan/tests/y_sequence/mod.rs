use ndarray::{Array2, Axis};


fn bms_to_y(b: &Array2<usize>) -> Vec<usize> {
    let x = b.shape()[0];
    let y = b.shape()[1];
    let mut parent_matrix: Vec<Vec<usize>> = Vec::new();

    for j in 0..y {
        for i in 0..x {
            let mut p: usize;
            if j == 0 {
                parent_matrix.push(Vec::new());
                p = (0..=i).rev().find(|&p| b[[p, j]] < b[[i, j]]).unwrap_or(0);
            } else {
                p = (0..=i).rev().find(|&p| b[[p, j]] < b[[i, j]]).unwrap_or(parent_matrix[i][j - 1]);
            }
            parent_matrix[i].push(p);
        }
    }

    let mut a: Vec<usize> = vec![1; x];

    for j in (0..y).rev() {
        for i in 0..x {
            a[i] = if b[[i, j]] == 0 { 1 } else { a[i] + a[parent_matrix[i][j]] };
        }
    }

    a
}

#[test]
fn test() {
    /// (0,0,0)(1,1,1)(2,1,0)(1,1,0)(2,2,1)(3,1,0)(2,2,1)
    let e = vec![0,0,0,1,1,1,2,1,0,1,1,0,2,2,1,3,1,0,2,2,0];
    let ee = vec![0,0,0,0,1,1,1,1];
        let sequence = Array2::from_shape_vec((2, 4), ee).unwrap();
    let bms = bms_to_y(&sequence);
    println!("{:?}", bms);
}