#![feature(test)]

extern crate test;

use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_frac_001(b: &mut Bencher) {
    let x = FeelNumber::new(123456789, 4);
    b.iter(|| x.frac())
}
