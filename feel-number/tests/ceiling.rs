mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn _0001() {
    eqs!("2", num!(1.5).ceiling(0).unwrap());
}

#[test]
fn _0002() {
    eqs!("-1", num!(-1.5).ceiling(0).unwrap());
}

#[test]
fn _0003() {
    eqs!("-1.5", num!(-1.56).ceiling(1).unwrap());
}

#[test]
fn _0004() {
    eqs!("1", num!(0.3333).ceiling(0).unwrap());
}

#[test]
fn _0005() {
    eqs!("0", num!(-0.3333).ceiling(0).unwrap());
}

#[test]
fn _0006() {
    println!("{}", num!(-0.3333).ceiling(0).unwrap());
}
