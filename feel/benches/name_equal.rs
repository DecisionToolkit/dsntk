#![feature(test)]

extern crate test;

use dsntk_feel::Name;
use test::Bencher;

#[bench]
fn feel_name_equal_0001(b: &mut Bencher) {
  let name: Name = "a".into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0002(b: &mut Bencher) {
  let name: Name = vec!["a", "b"].into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0003(b: &mut Bencher) {
  let name: Name = vec!["a", "b", "c"].into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0004(b: &mut Bencher) {
  let name: Name = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"].into();
  b.iter(|| name == name);
}

#[bench]
fn feel_name_equal_0005(b: &mut Bencher) {
  let name_a: Name = vec!["a", "b"].into();
  let name_b: Name = vec!["a b"].into();
  b.iter(|| name_a == name_b);
}

#[bench]
fn feel_name_equal_0006(b: &mut Bencher) {
  let name_a: Name = vec!["a", "b", "c"].into();
  let name_b: Name = vec!["a b c"].into();
  b.iter(|| name_a == name_b);
}

#[bench]
fn feel_name_equal_0007(b: &mut Bencher) {
  let name_a: Name = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"].into();
  let name_b: Name = vec!["a b c d e f g h i j"].into();
  b.iter(|| name_a == name_b);
}
