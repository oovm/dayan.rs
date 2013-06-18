use dayan::DayanPsi;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let node = DayanPsi::Psi(vec![
        DayanPsi::Number(1),
        DayanPsi::Omega,
    ]);
    println!("{:?}", node);
}