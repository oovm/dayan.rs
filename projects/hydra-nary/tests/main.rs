// use dayan::{BMSConfig, BashicuMatrixSystem, DayanAlpha, DayanError, NAryHydra};
// use hydra_nary::NAryHydra;
use pex::StopBecause;
use rand::{Rng, SeedableRng};
use std::{io::Write, str::FromStr};

#[test]
fn ready() {
    println!("it works!")
}

// pub fn show(input: &str) -> Result<(), DayanError> {
//     for line in input.lines() {
//         if line.trim().is_empty() {
//             continue;
//         }
//         let node = DayanAlpha::from_str(line)?;
//         let expr = node.as_expression()?;
//         println!("{}:", node);
//         // println!("  {:?}", node);
//         println!("  {}", expr);
//     }
//     Ok(())
// }

#[test]
fn export_beta1() -> Result<(), StopBecause> {
    let beta = NAryHydra::Body { ranks: vec![2], terms: vec![NAryHydra::Head { order: 2 }] };
    println!("{}", beta);
    println!("{:#?}", beta);
    // let beta = NAryHydra::Beta(1, vec![NAryHydra::Number(1)]);
    // println!("{}", beta.as_expression()?);
    // let beta = NAryHydra::Beta(1, vec![NAryHydra::Number(1), NAryHydra::Number(1)]);
    // println!("{}", beta.as_expression()?);
    // let beta = NAryHydra::Beta(1, vec![NAryHydra::Beta(1, vec![NAryHydra::Number(1)]), NAryHydra::Number(1)]);
    // println!("{}", beta.as_expression()?);
    // let beta = NAryHydra::Beta(1, vec![NAryHydra::Beta(2, vec![NAryHydra::Number(1)]), NAryHydra::Number(1)]);
    // println!("{}", beta.as_expression()?);
    Ok(())
}
// #[test]
// fn export_beta2() -> Result<(), DayanError> {
//     let beta = NAryHydra::Beta(2, vec![]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(2, vec![NAryHydra::Number(1)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(2, vec![NAryHydra::Number(1), NAryHydra::Number(1)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(2, vec![NAryHydra::Number(1), NAryHydra::Number(2)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(2, vec![NAryHydra::Number(2), NAryHydra::Number(2)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(4, vec![NAryHydra::Number(3)]);
//     println!("{:#?}", beta.as_dps()?);
//     Ok(())
// }
//
// #[test]
// #[ignore]
// fn export_beta3() -> Result<(), DayanError> {
//     let beta = NAryHydra::Beta(3, vec![]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(3, vec![NAryHydra::Number(1)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(3, vec![NAryHydra::Number(1), NAryHydra::Number(1)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(3, vec![NAryHydra::Number(1), NAryHydra::Number(2)]);
//     println!("{}", beta.as_expression()?);
//     let beta = NAryHydra::Beta(3, vec![NAryHydra::Number(2), NAryHydra::Number(2)]);
//     println!("{}", beta.as_expression()?);
//     Ok(())
// }

#[test]
fn test() {
    let mut fnt = BMSConfig::default();
    fnt.ellipsis = true;
    let sequence = vec![vec![0, 0], vec![1, 1], vec![2, 2]];
    let bms = BashicuMatrixSystem::new(sequence.clone()).unwrap().expand(2);
    println!("{:?}", bms);
    let bms = bms.expand(2);
    println!("{}", fnt.render(&bms));
    fnt.display = false;
    let bms = bms.expand(2);
    println!("{}", fnt.render(&bms));
}
