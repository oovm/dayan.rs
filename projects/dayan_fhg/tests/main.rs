use dayan::DayanPsi;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let node = DayanPsi::Psi(vec![
        DayanPsi::Number(0),
    ]);
    println!("{}: {}", node, node.as_expression().unwrap());
    let node = DayanPsi::Psi(vec![
        DayanPsi::Number(1),
    ]);
    println!("{}: {}", node, node.as_expression().unwrap());
    let node = DayanPsi::Psi(vec![
        DayanPsi::Omega,
    ]);
    println!("{}: {}", node, node.as_expression().unwrap());



    let node = DayanPsi::Psi(vec![
        DayanPsi::Number(1),
        DayanPsi::Omega,
    ]);
    println!("{}: {}", node, node.as_expression().unwrap());
}