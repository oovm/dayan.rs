use dayan::{DayanError, DayanPsi};
use std::str::FromStr;

#[test]
fn ready() {
    println!("it works!")
}

fn show(input: &str) -> Result<(), DayanError> {
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let node = DayanPsi::from_str(line)?;
        let expr = node.as_expression()?;
        println!("{}:\n    {}", node, expr);
    }
    Ok(())
}

#[test]
fn single_parameter() -> Result<(), DayanError> {
    show(
        r#"
0
1
w
p(0)
p(1)
p(2)
p(w)
p(p(0))
p(p(1))
    "#,
    )
}

#[test]
fn double_parameter() -> Result<(), DayanError> {
    show(
        r#"
        p(1, 0)
        p(1, 1)
        p(1, w)
        p(1, p(1))
        p(1, p(w))
        p(1, p(1, 0))
        p(1, p(1, w))
        p(2, 0)
        p(3, 0)
        p(3, 1)
        p(3, w)
        p(w, 0)
        p(p(1), 0)
        p(p(w), 0)
        p(p(p(1)), 0)
        p(p(p(w)), 0)
        p(p(1, 0), 0)
        p(p(1, w), 0)
        p(p(w, w), w)
        p(p(p(1, 0), 0), w)
        "#,
    )
}
