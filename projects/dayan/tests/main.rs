mod y_sequence;

use dayan::{BMSConfig, BashicuMatrixSystem, DayanAlpha, DayanBeta, DayanError};
use std::{fs::File, io::Write, path::Path, str::FromStr};
#[test]
fn ready() {
    println!("it works!")
}

pub fn show(input: &str) -> Result<(), DayanError> {
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let node = DayanAlpha::from_str(line)?;
        let expr = node.as_expression()?;
        println!("{}:", node);
        // println!("  {:?}", node);
        println!("  {}", expr);
    }
    Ok(())
}

#[test]
fn export_beta1() -> Result<(), DayanError> {
    let beta = DayanBeta::Beta(1, vec![]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(1, vec![DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(1, vec![DayanBeta::Number(1), DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(1, vec![DayanBeta::Beta(1, vec![DayanBeta::Number(1)]), DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(1, vec![DayanBeta::Beta(2, vec![DayanBeta::Number(1)]), DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    Ok(())
}
#[test]
fn export_beta2() -> Result<(), DayanError> {
    let beta = DayanBeta::Beta(2, vec![]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(2, vec![DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(2, vec![DayanBeta::Number(1), DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(2, vec![DayanBeta::Number(1), DayanBeta::Number(2)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(2, vec![DayanBeta::Number(2), DayanBeta::Number(2)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(4, vec![DayanBeta::Number(3)]);
    println!("{:#?}", beta.as_dps()?);
    Ok(())
}

#[test]
#[ignore]
fn export_beta3() -> Result<(), DayanError> {
    let beta = DayanBeta::Beta(3, vec![]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(3, vec![DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(3, vec![DayanBeta::Number(1), DayanBeta::Number(1)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(3, vec![DayanBeta::Number(1), DayanBeta::Number(2)]);
    println!("{}", beta.as_expression()?);
    let beta = DayanBeta::Beta(3, vec![DayanBeta::Number(2), DayanBeta::Number(2)]);
    println!("{}", beta.as_expression()?);
    Ok(())
}

#[test]
fn test() {
    let mut fnt = BMSConfig::default();
    fnt.ellipsis = true;
    let sequence = vec![vec![0, 0], vec![1, 1], vec![2, 2]];
    let bms = BashicuMatrixSystem::new(sequence.clone()).unwrap().expand();
    println!("{:?}", bms);
    let bms = bms.expand();
    println!("{}", fnt.render(&bms));
    fnt.display = false;
    let bms = bms.expand();
    println!("{}", fnt.render(&bms));
}
