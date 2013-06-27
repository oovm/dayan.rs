use dayan::{DayanError, DayanPsi};
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
        let node = DayanPsi::from_str(line)?;
        let expr = node.as_expression()?;
        println!("{}:", node);
        // println!("  {:?}", node);
        println!("  {}", expr);
    }
    Ok(())
}

pub fn markdown(input: &str, file: &mut File) -> Result<(), DayanError> {
    writeln!(file, "| Node | Psi | Expression |")?;
    writeln!(file, "| ---- | --- | ---------- |")?;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let node = DayanPsi::from_str(line)?;
        let expr = node.as_expression()?;
        writeln!(file, "| ${:?}$ | ${}$ | ${}$ |", node, node, expr)?;
    }
    Ok(())
}

#[test]
fn export_psi() -> Result<(), DayanError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut file = File::create(here.join("tests").join("psi.md"))?;
    markdown(include_str!("psi.txt"), &mut file)
}
