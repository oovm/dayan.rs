use std::str::FromStr;
use dayan::{DayanError, DayanPsi};

#[test]
fn ready() {
    println!("it works!")
}


fn show(input: &str) -> Result<(), DayanError> {
    let node = DayanPsi::from_str(input)?;
    let expr = node.as_expression()?;
    println!("{}:\n    {}", node, expr);
    Ok(())
}

#[test]
fn single_parameter() {
    show("0").unwrap();
    show("1").unwrap();
    show("w").unwrap();
    show("p(0)").unwrap();
    show("p(1)").unwrap();
    show("p(2)").unwrap();
    show("p(w)").unwrap();
    show("p(p(0))").unwrap();
    show("p(p(1))").unwrap();
}

#[test]
fn double_parameter() {
    show("p(1, 0)").unwrap();
    show("p(1, 1)").unwrap();
    show("p(1, w)").unwrap();
    show("p(1, p(1))").unwrap();
    show("p(1, p(w))").unwrap();
    show("p(1, p(1, 0))").unwrap();
    show("p(1, p(1, w))").unwrap();
    show("p(2, 0)").unwrap();
    show("p(3, 0)").unwrap();
}

#[test]
fn double_parameter2() {
    show("p(w, 0)").unwrap();
    show("p(p(0), 0)").unwrap();
    show("p(p(1), 0)").unwrap();
}