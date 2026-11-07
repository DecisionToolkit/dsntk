#![feature(test)]

extern crate test;

use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_is_one_001(b: &mut Bencher) {
    let x = FeelNumber::new(1, 0);
    b.iter(|| x.is_one())
}

#[bench]
fn bench_is_one_002(b: &mut Bencher) {
    let x = FeelNumber::new(11, 1);
    b.iter(|| x.is_one())
}
