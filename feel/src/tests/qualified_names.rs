use crate::{Name, QualifiedName};

#[test]
fn test_qualified_name() {
  let name_a = Name::new(&["a", "+", "b"]);
  let name_b = Name::new(&["b", "-", "c"]);
  let name_c = Name::new(&["c", "/", "d"]);
  let name_d = Name::new(&["d", "*", "e"]);
  let name_e = Name::new(&["e", ".", "f"]);
  let name_f = Name::new(&["f", "'", "g"]);
  let qname = QualifiedName::new(&[]);
  assert_eq!("", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a]);
  assert_eq!("a+b", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b]);
  assert_eq!("a+b.b-c", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert_eq!("a+b.b-c.c/d", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d]);
  assert_eq!("a+b.b-c.c/d.d*e", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d, &name_e]);
  assert_eq!("a+b.b-c.c/d.d*e.e.f", qname.to_string().as_str());
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d, &name_e, &name_f]);
  assert_eq!("a+b.b-c.c/d.d*e.e.f.f'g", qname.to_string().as_str());
}

#[test]
fn test_push() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let mut qname = QualifiedName::new(&[]);
  assert_eq!("", qname.to_string().as_str());
  qname.push(name_a);
  assert_eq!("a", qname.to_string().as_str());
  qname.push(name_b);
  assert_eq!("a.b", qname.to_string().as_str());
  qname.push(name_c);
  assert_eq!("a.b.c", qname.to_string().as_str());
}

#[test]
fn test_insert() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let mut qname = QualifiedName::new(&[]);
  assert_eq!("", qname.to_string().as_str());
  qname.push(name_a.clone());
  assert_eq!("a", qname.to_string().as_str());
  qname.insert(0, name_b);
  assert_eq!("b.a", qname.to_string().as_str());
  qname.push(name_c);
  assert_eq!("b.a.c", qname.to_string().as_str());
  qname.insert(0, name_a);
  assert_eq!("a.b.a.c", qname.to_string().as_str());
}

#[test]
fn test_pop() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let mut qname = QualifiedName::new(&[]);
  qname.push(name_a);
  qname.push(name_b);
  qname.push(name_c);
  assert_eq!("a.b.c", qname.to_string().as_str());
  qname.pop();
  assert_eq!("a.b", qname.to_string().as_str());
  qname.pop();
  assert_eq!("a", qname.to_string().as_str());
  qname.pop();
  assert_eq!("", qname.to_string().as_str());
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert_eq!("a.b.c", qname.to_string().as_str());
  let qname_clone = qname.clone();
  assert_eq!("a.b.c", qname_clone.to_string().as_str());
}

#[test]
fn test_debug() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert_eq!(r#"QualifiedName([Name("a"), Name("b"), Name("c")])"#, format!("{qname:?}"));
}

#[test]
fn test_compare() {
  let name_a = Name::new(&["a"]);
  let name_b = Name::new(&["b"]);
  let name_c = Name::new(&["c"]);
  let qname_a = QualifiedName::new(&[&name_a, &name_b]);
  let qname_b = QualifiedName::new(&[&name_a, &name_b]);
  let qname_c = QualifiedName::new(&[&name_a, &name_b, &name_c]);
  assert!((qname_a == qname_b));
  assert!((qname_a != qname_c));
}
