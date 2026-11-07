#![feature(test)]

extern crate test;

use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_abs_001(b: &mut Bencher) {
    let n = FeelNumber::new(0, 0);
    b.iter(|| n.abs())
}

#[bench]
fn bench_abs_002(b: &mut Bencher) {
    let n = FeelNumber::new(-0, 0);
    b.iter(|| n.abs())
}

#[bench]
fn bench_abs_003(b: &mut Bencher) {
    let n = FeelNumber::new(1, 0);
    b.iter(|| n.abs())
}

#[bench]
fn bench_abs_004(b: &mut Bencher) {
    let n = FeelNumber::new(-1, 0);
    b.iter(|| n.abs())
}

#[bench]
fn bench_abs_005(b: &mut Bencher) {
    let n = FeelNumber::new(123456, 6);
    b.iter(|| n.abs())
}

#[bench]
fn bench_abs_006(b: &mut Bencher) {
    let n = FeelNumber::new(-123456, 6);
    b.iter(|| n.abs())
}
