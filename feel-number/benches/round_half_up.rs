#![feature(test)]

extern crate test;

use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_round_half_up_0001(b: &mut Bencher) {
    let x = FeelNumber::new(54, 1);
    assert_eq!("5", x.round_half_up(0).unwrap().to_string());
    b.iter(|| x.round_half_up(0).unwrap())
}

#[bench]
fn bench_round_half_up_0002(b: &mut Bencher) {
    let x = FeelNumber::new(55, 1);
    assert_eq!("6", x.round_half_up(0).unwrap().to_string());
    b.iter(|| x.round_half_up(0).unwrap())
}

#[bench]
fn bench_round_half_up_0003(b: &mut Bencher) {
    let x = FeelNumber::new(56, 1);
    assert_eq!("6", x.round_half_up(0).unwrap().to_string());
    b.iter(|| x.round_half_up(0).unwrap())
}
