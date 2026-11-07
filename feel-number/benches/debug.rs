#![feature(test)]

extern crate test;

use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_debug_001(b: &mut Bencher) {
    let x = FeelNumber::new(12345, 3);
    b.iter(|| format!("{x:?}"))
}
