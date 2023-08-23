use dayan::{BashicuMatrixSystem, OrdinalNotation};
use hydra_nary::NAryHydraTeX;

#[test]
fn test() {
    // (0,0,0)(1,1,1)(2,1,0)(1,1,0)(2,2,1)(3,1,0)(2,2,1)
    // (0,0)(1,1)(2,2)(2,1)(2,1)
    let sequence =
        vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 1, 0], vec![1, 1, 0], vec![2, 2, 1], vec![3, 1, 0], vec![2, 2, 1]];
    let sequence2 = vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![2, 1], vec![2, 1]];
    let bms = BashicuMatrixSystem::new(sequence2.clone()).unwrap();
    let y = bms.as_y_sequence();
    println!("{:?}", y);
    let render = NAryHydraTeX::default();
    let hydra = bms.as_nary_hydra();
    println!("{}", render.render(&hydra));
}
