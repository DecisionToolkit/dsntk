#![feature(test)]

extern crate test;

use dsntk_feel::Name;
use test::Bencher;

#[bench]
fn feel_name_to_string_0001(b: &mut Bencher) {
    let name: Name = "a".into();
    b.iter(|| name.to_string());
}

#[bench]
fn feel_name_to_string_0002(b: &mut Bencher) {
    let name: Name = vec!["a", "b"].into();
    b.iter(|| name.to_string());
}

#[bench]
fn feel_name_to_string_0003(b: &mut Bencher) {
    let name: Name = vec!["a", "b", "c"].into();
    b.iter(|| name.to_string());
}

#[bench]
fn feel_name_to_string_0004(b: &mut Bencher) {
    let name: Name = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"].into();
    b.iter(|| name.to_string());
}
