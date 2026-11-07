#![feature(test)]

extern crate test;

use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_add_001(b: &mut Bencher) {
    let x = FeelNumber::new(1, 1);
    let y = FeelNumber::new(3, 1);
    b.iter(|| x + y)
}

#[bench]
fn bench_add_002(b: &mut Bencher) {
    let x = FeelNumber::new(12345, 2);
    let y = FeelNumber::new(3847847, 7);
    b.iter(|| x + y)
}

#[bench]
fn bench_add_003(b: &mut Bencher) {
    let x = FeelNumber::new(123, 2);
    let y = FeelNumber::new(77, 1);
    b.iter(|| x + y)
}

#[bench]
fn bench_add_004(b: &mut Bencher) {
    let mut x = FeelNumber::new(1, 1);
    let y = FeelNumber::new(3, 1);
    b.iter(|| x += y)
}

#[bench]
fn bench_add_005(b: &mut Bencher) {
    let mut x = FeelNumber::new(12345, 2);
    let y = FeelNumber::new(3847847, 7);
    b.iter(|| x += y)
}

#[bench]
fn bench_add_006(b: &mut Bencher) {
    let mut x = FeelNumber::new(123, 2);
    let y = FeelNumber::new(77, 1);
    b.iter(|| x += y)
}
