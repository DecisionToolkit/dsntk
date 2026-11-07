mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn _0001() {
    let x: FeelNumber = u8::MAX.into();
    eqs!("255", x);
}

#[test]
fn _0002() {
    let x: FeelNumber = i8::MIN.into();
    eqs!("-128", x);
}

#[test]
fn _0003() {
    let x: FeelNumber = u16::MAX.into();
    eqs!("65535", x);
}

#[test]
fn _0004() {
    let x: FeelNumber = i16::MIN.into();
    eqs!("-32768", x);
}

#[test]
fn _0005() {
    let x: FeelNumber = i32::MIN.into();
    eqs!("-2147483648", x);
}

#[test]
fn _0006() {
    let x: FeelNumber = u32::MAX.into();
    eqs!("4294967295", x);
}

#[test]
fn _0007() {
    let x: FeelNumber = u64::MAX.into();
    eqs!("18446744073709551615", x);
}

#[test]
fn _0008() {
    let x: FeelNumber = i64::MIN.into();
    eqs!("-9223372036854775808", x);
}

#[test]
fn _0009() {
    let x: FeelNumber = isize::MIN.into();
    eqs!("-9223372036854775808", x);
}

#[test]
fn _0010() {
    let x: FeelNumber = usize::MAX.into();
    eqs!("18446744073709551615", x);
}

#[test]
fn _0011() {
    let x: i32 = num!(-2147483648).try_into().unwrap();
    assert_eq!(-2147483648_i32, x);
}

#[test]
fn _0012() {
    let x: u32 = num!(4294967295).try_into().unwrap();
    assert_eq!(4294967295_u32, x);
}

#[test]
fn _0013() {
    let x: u8 = num!(255).try_into().unwrap();
    assert_eq!(255_u8, x);
}

#[test]
fn _0014() {
    assert_eq!(27_u8, u8::try_from(num!(27)).unwrap());
    assert!(u8::try_from(num!(0)).is_ok());
    assert_eq!(0, u8::try_from(num!(0)).unwrap());
    assert!(u8::try_from(num!(0.999)).is_ok());
    assert_eq!(0, u8::try_from(num!(0.999)).unwrap());
    assert!(u8::try_from(num!(255)).is_ok());
    assert_eq!(255, u8::try_from(num!(255)).unwrap());
    assert!(u8::try_from(num!(256)).is_err());
    assert!(u8::try_from(num!(4294967296)).is_err());
}

#[test]
fn _0015() {
    assert_eq!(-27_i8, i8::try_from(num!(-27)).unwrap());
    assert!(i8::try_from(num!(0)).is_ok());
    assert!(i8::try_from(num!(0.999)).is_ok());
    assert!(i8::try_from(num!(-0.999)).is_ok());
    assert!(i8::try_from(num!(-128)).is_ok());
    assert!(i8::try_from(num!(127)).is_ok());
    assert!(i8::try_from(num!(-129)).is_err());
    assert!(i8::try_from(num!(128)).is_err());
    assert!(i8::try_from(num!(4294967296)).is_err());
    assert!(i8::try_from(num!(-4294967296)).is_err());
}

#[test]
fn _0016() {
    assert_eq!(65530_u16, u16::try_from(num!(65530)).unwrap());
    assert!(u16::try_from(num!(0)).is_ok());
    assert!(u16::try_from(num!(0.999)).is_ok());
    assert!(u16::try_from(num!(65535)).is_ok());
    assert!(u16::try_from(num!(65536)).is_err());
    assert!(u16::try_from(num!(4294967296)).is_err());
}

#[test]
fn _0017() {
    assert_eq!(-32760_i16, i16::try_from(num!(-32760)).unwrap());
    assert!(i16::try_from(num!(0)).is_ok());
    assert!(i16::try_from(num!(0.999)).is_ok());
    assert!(i16::try_from(num!(-0.999)).is_ok());
    assert!(i16::try_from(num!(32767)).is_ok());
    assert!(i16::try_from(num!(-32768)).is_ok());
    assert!(i16::try_from(num!(32768)).is_err());
    assert!(i16::try_from(num!(-32769)).is_err());
    assert!(i16::try_from(num!(4294967296)).is_err());
    assert!(i16::try_from(num!(-4294967296)).is_err());
}

#[test]
fn _0018() {
    assert_eq!(27443494_u32, u32::try_from(num!(27443494)).unwrap());
    assert!(u32::try_from(num!(0)).is_ok());
    assert!(u32::try_from(num!(0.999)).is_ok());
    assert!(u32::try_from(num!(4294967295)).is_ok());
    assert!(u32::try_from(num!(4294967296)).is_err());
    assert!(u32::try_from(num!(-1)).is_err());
}

#[test]
fn _0019() {
    assert_eq!(-27443494_i32, i32::try_from(num!(-27443494)).unwrap());
    assert!(i32::try_from(num!(0)).is_ok());
    assert!(i32::try_from(num!(0.999)).is_ok());
    assert!(i32::try_from(num!(-0.999)).is_ok());
    assert!(i32::try_from(num!(2147483647)).is_ok());
    assert!(i32::try_from(num!(-2147483648)).is_ok());
    assert!(i32::try_from(num!(2147483648)).is_err());
    assert!(i32::try_from(num!(-2147483649)).is_err());
}

#[test]
fn _0020() {
    assert_eq!(300_000_000_u64, u64::try_from(num!(300000000)).unwrap());
    assert!(u64::try_from(num!(0)).is_ok());
    assert!(u64::try_from(num!(0.999)).is_ok());
    assert!(u64::try_from(num!(18446744073709551615)).is_ok());
    assert!(u64::try_from(num!(18446744073709551615.999)).is_ok());
    assert!(u64::try_from(num!(18446744073709551616)).is_err());
    assert!(u64::try_from(num!(-1)).is_err());
}

#[test]
fn _0021() {
    assert_eq!(-300_000_000_i64, i64::try_from(num!(-300000000)).unwrap());
    assert!(i64::try_from(num!(0)).is_ok());
    assert!(i64::try_from(num!(0.999)).is_ok());
    assert!(i64::try_from(num!(-0.999)).is_ok());
    assert!(i64::try_from(num!(9223372036854775807)).is_ok());
    assert!(i64::try_from(num!(-9223372036854775808)).is_ok());
    assert!(i64::try_from(num!(9223372036854775808)).is_err());
    assert!(i64::try_from(num!(-9223372036854775809)).is_err());
}

#[test]
#[cfg(target_pointer_width = "64")]
fn _0022() {
    assert_eq!(
        -300_000_000_isize,
        isize::try_from(num!(-300000000)).unwrap()
    );
    assert!(isize::try_from(num!(0)).is_ok());
    assert!(isize::try_from(num!(0.999)).is_ok());
    assert!(isize::try_from(num!(-0.999)).is_ok());
    assert!(isize::try_from(num!(9223372036854775807)).is_ok());
    assert!(isize::try_from(num!(-9223372036854775808)).is_ok());
    assert!(isize::try_from(num!(9223372036854775808)).is_err());
    assert!(isize::try_from(num!(-9223372036854775809)).is_err());
}

#[test]
#[cfg(target_pointer_width = "64")]
fn _0023() {
    assert_eq!(300_000_000_usize, usize::try_from(num!(300000000)).unwrap());
    assert!(usize::try_from(num!(0)).is_ok());
    assert!(usize::try_from(num!(0.999)).is_ok());
    assert!(usize::try_from(num!(18446744073709551615)).is_ok());
    assert!(usize::try_from(num!(18446744073709551615.999)).is_ok());
    assert!(usize::try_from(num!(18446744073709551616)).is_err());
    assert!(usize::try_from(num!(-1)).is_err());
}
