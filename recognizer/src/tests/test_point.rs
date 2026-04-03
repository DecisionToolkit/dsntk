use crate::point::*;

#[test]
fn test_point_zero() {
  assert_eq!((0, 0), Point::zero().into());
}

#[test]
fn test_point_new() {
  assert_eq!((1, 2), Point::new(1, 2).into());
}

#[test]
fn test_point_display() {
  assert_eq!("(10,20)", format!("{}", Point::new(10, 20)));
  assert_eq!("(0,0)", format!("{}", Point::zero()));
}

#[test]
fn test_point_debug() {
  assert_eq!("(10,20)", format!("{:?}", Point::new(10, 20)));
  assert_eq!("(0,0)", format!("{:?}", Point::zero()));
}

#[test]
fn test_point_compare() {
  let p1 = Point::new(1, 2);
  let p2 = Point::new(1, 2);
  let p3 = Point::new(2, 1);
  assert!(p1.eq(&p2));
  assert!(p1.ne(&p3));
}

#[test]
fn test_point_conversion_into_tuple() {
  assert_eq!((21, 22), Point::new(21, 22).into());
}

#[test]
fn test_point_clone() {
  let p = Point::new(1, 2);
  assert!(p.eq(&p.clone()));
}
