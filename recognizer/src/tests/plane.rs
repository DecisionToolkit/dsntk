use crate::canvas::Canvas;
use crate::tests::{EX_04, EX_07};

#[test]
fn plane_pivot_horizontal() {
    let mut canvas = Canvas::scan(&String::from(EX_04)).unwrap();
    let mut plane = canvas.plane().unwrap();
    let before = format!("{plane}");
    plane.pivot();
    plane.pivot();
    let after = format!("{plane}");
    assert_eq!(before, after);
}

#[test]
fn plane_pivot_vertical() {
    let mut canvas = Canvas::scan(&String::from(EX_07)).unwrap();
    let mut plane = canvas.plane().unwrap();
    let before = format!("{plane}");
    plane.pivot();
    plane.pivot();
    let after = format!("{plane}");
    assert_eq!(before, after);
}
