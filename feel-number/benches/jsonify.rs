#![feature(test)]

extern crate test;

use dsntk_common::Jsonify;
use dsntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_jsonify_001(b: &mut Bencher) {
    let x = FeelNumber::new(123456789, 4);
    b.iter(|| x.jsonify())
}
