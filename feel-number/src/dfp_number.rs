//! # FEEL number type
//!
//! Implementation of the FEEL number type
//! based on the **Intel(R) Decimal Floating-Point Math Library**.

use crate::errors::*;
use dfp_number_sys::bid128_000::*;
use dfp_number_sys::*;
use dsntk_common::{DsntkError, Jsonify, Result};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

/// Minimum scale.
const MIN_SCALE: i32 = -6111;

/// Maximum scale.
const MAX_SCALE: i32 = 6144;

/// Flags for the status of the operation.
macro_rules! flags {
  () => {
    &mut 0_u32
  };
}

/// Rounding mode.
macro_rules! round {
  () => {
    RND_NEAREST_EVEN
  };
}

/// FEEL number.
#[derive(Copy, Clone)]
pub struct FeelNumber(BID128, bool);

impl FeelNumber {
  /// Returns [FeelNumber] value infinite.
  pub fn infinite() -> FeelNumber {
    Self(BID128_INF, false)
  }

  /// Returns [FeelNumber] value 0 (zero).
  pub fn zero() -> FeelNumber {
    Self(BID128_ZERO, false)
  }

  /// Returns [FeelNumber] value 1 (one).
  pub fn one() -> FeelNumber {
    Self(BID128_ONE, false)
  }

  /// Returns [FeelNumber] value 2 (two).
  pub fn two() -> FeelNumber {
    Self(BID128_TWO, false)
  }

  /// Returns [FeelNumber] value 1_000_000_000 (billion).
  pub fn billion() -> FeelNumber {
    Self(BID128_BILLION, false)
  }

  /// Creates a new [FeelNumber] from integer value and a scale.
  pub fn new(n: i64, s: i32) -> Self {
    Self(bid128_scalbn(bid128_from_int64(n), -s), false)
  }

  /// Creates a new [FeelNumber] from [isize].
  fn from_isize(n: isize) -> Self {
    Self(bid128_from_string(&format!("{n}"), round!(), flags!()), false)
  }

  /// Returns an absolute value of this [FeelNumber].
  pub fn abs(&self) -> Self {
    Self(bid128_abs(self.0, flags!()), false)
  }

  /// Returns a nearest integer greater or equal to this [FeelNumber].
  pub fn ceiling(&self, scale: i32) -> Result<Self> {
    Ok(Self(
      if scale == 0 {
        let rounded = bid128_round_integral_positive(self.0, flags!());
        if bid128_is_zero(rounded) { BID128_ZERO } else { rounded }
      } else {
        self.validate_scale(scale)?;
        self.unscale(bid128_round_integral_positive(bid128_scalbn(self.0, scale), flags!()), scale)
      },
      true,
    ))
  }

  pub fn even(&self) -> bool {
    bid128_is_zero(bid128_rem(self.0, BID128_TWO, flags!()))
  }

  pub fn exp(&self) -> Option<Self> {
    let n = bid128_exp(self.0, round!(), flags!());
    if bid128_is_finite(n) { Some(Self(n, true)) } else { None }
  }

  /// Returns a nearest integer less or equal to this [FeelNumber].
  pub fn floor(&self, scale: i32) -> Result<Self> {
    Ok(Self(
      if scale == 0 {
        bid128_round_integral_negative(self.0, flags!())
      } else {
        self.validate_scale(scale)?;
        self.unscale(bid128_round_integral_negative(bid128_scalbn(self.0, scale), flags!()), scale)
      },
      true,
    ))
  }

  pub fn frac(&self) -> Self {
    Self(bid128_sub(self.0, bid128_round_integral_zero(self.0, flags!()), round!(), flags!()), true)
  }

  pub fn is_integer(&self) -> bool {
    bid128_quiet_equal(self.0, bid128_round_integral_zero(self.0, flags!()), flags!())
  }

  pub fn is_zero(&self) -> bool {
    bid128_quiet_equal(self.0, BID128_ZERO, flags!())
  }

  pub fn is_one(&self) -> bool {
    bid128_quiet_equal(self.0, BID128_ONE, flags!())
  }

  pub fn is_negative(&self) -> bool {
    bid128_quiet_less(self.0, BID128_ZERO, flags!())
  }

  pub fn is_positive(&self) -> bool {
    bid128_quiet_greater(self.0, BID128_ZERO, flags!())
  }

  pub fn ln(&self) -> Option<Self> {
    let n = bid128_log(self.0, round!(), flags!());
    if bid128_is_finite(n) { Some(Self(n, true)) } else { None }
  }

  pub fn odd(&self) -> bool {
    if self.is_integer() {
      !bid128_is_zero(bid128_rem(self.0, BID128_TWO, flags!()))
    } else {
      false
    }
  }

  pub fn pow(&self, rhs: &FeelNumber) -> Option<Self> {
    let n = bid128_pow(self.0, rhs.0, round!(), flags!());
    if bid128_is_finite(n) { Some(Self(n, true)) } else { None }
  }

  pub fn round(&self, rhs: &FeelNumber) -> Self {
    let scale = bid128_to_int32_int(rhs.0, flags!());
    let quantifier = bid128_scalbn(BID128_ONE, -scale);
    Self(bid128_quantize(self.0, quantifier, round!(), flags!()), false)
  }

  pub fn round_down(&self, scale: i32) -> Result<Self> {
    Ok(Self(
      if scale == 0 {
        bid128_round_integral_zero(self.0, flags!())
      } else {
        self.validate_scale(scale)?;
        self.unscale(bid128_round_integral_zero(bid128_scalbn(self.0, scale), flags!()), scale)
      },
      true,
    ))
  }

  pub fn round_half_down(&self, scale: i32) -> Result<Self> {
    let positive = |n| {
      let b = bid128_sub(n, BID128_ONE_TENTH, round!(), flags!());
      bid128_round_integral_nearest_away(b, flags!())
    };
    let negative = |n| {
      let b = bid128_add(n, BID128_ONE_TENTH, round!(), flags!());
      bid128_round_integral_nearest_away(b, flags!())
    };
    Ok(Self(
      if scale == 0 {
        if bid128_is_signed(self.0) { negative(self.0) } else { positive(self.0) }
      } else {
        self.validate_scale(scale)?;
        let scaled = bid128_scalbn(self.0, scale);
        self.unscale(if bid128_is_signed(self.0) { negative(scaled) } else { positive(scaled) }, scale)
      },
      true,
    ))
  }

  pub fn round_half_up(&self, scale: i32) -> Result<Self> {
    Ok(Self(
      if scale == 0 {
        bid128_round_integral_nearest_away(self.0, flags!())
      } else {
        self.validate_scale(scale)?;
        self.unscale(bid128_round_integral_nearest_away(bid128_scalbn(self.0, scale), flags!()), scale)
      },
      true,
    ))
  }

  pub fn round_up(&self, scale: i32) -> Result<Self> {
    Ok(Self(
      if scale == 0 {
        if bid128_is_signed(self.0) {
          bid128_round_integral_negative(self.0, flags!())
        } else {
          bid128_round_integral_positive(self.0, flags!())
        }
      } else {
        self.validate_scale(scale)?;
        self.unscale(
          if bid128_is_signed(self.0) {
            bid128_round_integral_negative(bid128_scalbn(self.0, scale), flags!())
          } else {
            bid128_round_integral_positive(bid128_scalbn(self.0, scale), flags!())
          },
          scale,
        )
      },
      true,
    ))
  }

  pub fn sqrt(&self) -> Option<Self> {
    let n = bid128_sqrt(self.0, round!(), flags!());
    if bid128_is_finite(n) { Some(Self(n, true)) } else { None }
  }

  pub fn square(&self) -> Option<Self> {
    let n = bid128_pow(self.0, BID128_TWO, round!(), flags!());
    if bid128_is_finite(n) { Some(Self(n, true)) } else { None }
  }

  pub fn trunc(&self) -> Self {
    Self(bid128_round_integral_zero(self.0, flags!()), false)
  }

  /// Calculates the remainder of the division.
  fn remainder(&self, rhs: BID128) -> BID128 {
    let mut n = bid128_div(self.0, rhs, round!(), flags!());
    n = bid128_round_integral_negative(n, flags!());
    n = bid128_mul(rhs, n, round!(), flags!());
    bid128_sub(self.0, n, round!(), flags!())
  }

  /// Validates effective scale after changing the scale to the required one.
  /// When the final scale is out of allowed range (-6176..6144) then returns an error.
  fn validate_scale(&self, scale: i32) -> Result<()> {
    let scale = bid128_ilogb(self.0, flags!()) + scale;
    if (MIN_SCALE..=MAX_SCALE).contains(&scale) {
      Ok(())
    } else {
      Err(err_invalid_scale(MIN_SCALE, MAX_SCALE, scale))
    }
  }

  fn unscale(&self, n: BID128, scale: i32) -> BID128 {
    if bid128_is_zero(n) {
      BID128_ZERO
    } else if bid128_quiet_equal(n, BID128_ONE, flags!()) {
      BID128_ONE
    } else if bid128_quiet_equal(n, BID128_MINUS_ONE, flags!()) {
      BID128_MINUS_ONE
    } else {
      bid128_scalbn(n, -scale)
    }
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    bid128_quiet_equal(self.0, rhs.0, flags!())
  }
}

impl PartialEq<FeelNumber> for isize {
  fn eq(&self, rhs: &FeelNumber) -> bool {
    bid128_quiet_equal(FeelNumber::from_isize(*self).0, rhs.0, flags!())
  }
}

impl PartialEq<isize> for FeelNumber {
  fn eq(&self, rhs: &isize) -> bool {
    bid128_quiet_equal(self.0, FeelNumber::from_isize(*rhs).0, flags!())
  }
}

impl PartialOrd<FeelNumber> for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    if bid128_quiet_equal(self.0, rhs.0, flags!()) {
      return Some(Ordering::Equal);
    }
    if bid128_quiet_greater(self.0, rhs.0, flags!()) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }

  fn lt(&self, rhs: &FeelNumber) -> bool {
    bid128_quiet_less(self.0, rhs.0, flags!())
  }

  fn le(&self, rhs: &FeelNumber) -> bool {
    bid128_quiet_less_equal(self.0, rhs.0, flags!())
  }

  fn gt(&self, rhs: &FeelNumber) -> bool {
    bid128_quiet_greater(self.0, rhs.0, flags!())
  }

  fn ge(&self, rhs: &FeelNumber) -> bool {
    bid128_quiet_greater_equal(self.0, rhs.0, flags!())
  }
}

impl PartialOrd<FeelNumber> for isize {
  fn partial_cmp(&self, rhs: &FeelNumber) -> Option<Ordering> {
    let n = FeelNumber::from_isize(*self).0;
    if bid128_quiet_equal(n, rhs.0, flags!()) {
      return Some(Ordering::Equal);
    }
    if bid128_quiet_greater(n, rhs.0, flags!()) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<isize> for FeelNumber {
  fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
    let n = FeelNumber::from_isize(*rhs).0;
    if bid128_quiet_equal(self.0, n, flags!()) {
      return Some(Ordering::Equal);
    }
    if bid128_quiet_greater(self.0, n, flags!()) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl Add<FeelNumber> for FeelNumber {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self(bid128_add(self.0, rhs.0, round!(), flags!()), true)
  }
}

impl AddAssign<FeelNumber> for FeelNumber {
  fn add_assign(&mut self, rhs: Self) {
    self.0 = bid128_add(self.0, rhs.0, round!(), flags!());
    self.1 = true;
  }
}

impl Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Self(bid128_sub(self.0, rhs.0, round!(), flags!()), true)
  }
}

impl SubAssign<FeelNumber> for FeelNumber {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = bid128_sub(self.0, rhs.0, round!(), flags!());
    self.1 = true;
  }
}

impl Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Self(bid128_mul(self.0, rhs.0, round!(), flags!()), true)
  }
}

impl MulAssign<FeelNumber> for FeelNumber {
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = bid128_mul(self.0, rhs.0, round!(), flags!());
    self.1 = true;
  }
}

impl Div<FeelNumber> for FeelNumber {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    Self(bid128_div(self.0, rhs.0, round!(), flags!()), true)
  }
}

impl DivAssign<FeelNumber> for FeelNumber {
  fn div_assign(&mut self, rhs: Self) {
    self.0 = bid128_div(self.0, rhs.0, round!(), flags!());
    self.1 = true;
  }
}

impl Rem<FeelNumber> for FeelNumber {
  type Output = Self;
  fn rem(self, rhs: Self) -> Self::Output {
    Self(self.remainder(rhs.0), true)
  }
}

impl RemAssign<FeelNumber> for FeelNumber {
  fn rem_assign(&mut self, rhs: Self) {
    self.0 = self.remainder(rhs.0);
    self.1 = true;
  }
}

impl Neg for FeelNumber {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self(bid128_negate(self.0), true)
  }
}

impl Debug for FeelNumber {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", bid128_to_string(self.0, flags!()))
  }
}

impl Display for FeelNumber {
  /// Converts [FeelNumber] to human readable string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = bid128_to_string(self.0, flags!());
    let negative = s.starts_with('-');
    let mut split = s[1..].split('E');
    let (sb, sa) = split.next().zip(split.next()).unwrap(); // unwrap is ok, there is always E present
    let exponent = sa.parse::<isize>().unwrap(); // unwrap is ok, there is always correct exponent present
    let decimal_points = exponent.unsigned_abs();
    let (mut before, mut after) = if exponent < 0 {
      let digit_count = sb.len();
      if digit_count <= decimal_points {
        let before = "0".to_string();
        let mut after = "0".repeat(decimal_points - digit_count);
        if self.1 {
          after.push_str(sb.trim_end_matches('0'));
        } else {
          after.push_str(sb);
        }
        (before, after)
      } else {
        let before = sb[..digit_count - decimal_points].to_string();
        let after = if self.1 {
          sb[digit_count - decimal_points..].trim_end_matches('0').to_string()
        } else {
          sb[digit_count - decimal_points..].to_string()
        };
        (before, after)
      }
    } else {
      let mut before = sb.to_string();
      before.push_str(&"0".repeat(decimal_points));
      let after = "".to_string();
      (before, after)
    };
    if let Some(precision) = f.precision() {
      if after.len() < precision {
        after.push_str(&"0".repeat(precision - after.len()));
      } else {
        after = after[0..precision].to_string();
      }
    }
    if !after.is_empty() {
      before.push('.');
      before.push_str(&after);
    }
    f.pad_integral(!negative, "", &before)
  }
}

impl Jsonify for FeelNumber {
  /// Converts [FeelNumber] to JSON string.
  fn jsonify(&self) -> String {
    format!("{self}")
  }
}

impl FromStr for FeelNumber {
  type Err = DsntkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let n = bid128_from_string(s, round!(), flags!());
    if bid128_is_finite(n) { Ok(Self(n, false)) } else { Err(err_invalid_number_literal(s)) }
  }
}

impl From<u8> for FeelNumber {
  fn from(value: u8) -> Self {
    Self(bid128_from_uint32(value as u32), false)
  }
}

impl From<i8> for FeelNumber {
  fn from(value: i8) -> Self {
    Self(bid128_from_int32(value as i32), false)
  }
}

impl From<u16> for FeelNumber {
  fn from(value: u16) -> Self {
    Self(bid128_from_uint32(value as u32), false)
  }
}

impl From<i16> for FeelNumber {
  fn from(value: i16) -> Self {
    Self(bid128_from_int32(value as i32), false)
  }
}

impl From<u32> for FeelNumber {
  fn from(value: u32) -> Self {
    Self(bid128_from_uint32(value), false)
  }
}

impl From<i32> for FeelNumber {
  fn from(value: i32) -> Self {
    Self(bid128_from_int32(value), false)
  }
}

impl From<u64> for FeelNumber {
  fn from(value: u64) -> Self {
    Self(bid128_from_uint64(value), false)
  }
}

impl From<i64> for FeelNumber {
  fn from(value: i64) -> Self {
    Self(bid128_from_int64(value), false)
  }
}

impl From<isize> for FeelNumber {
  #[cfg(target_pointer_width = "64")]
  fn from(value: isize) -> Self {
    Self(bid128_from_int64(value.try_into().unwrap()), false)
  }

  #[cfg(not(target_pointer_width = "64"))]
  fn from(value: isize) -> Self {
    compile_error!("Implement conversion from isize for other architectures as 64-bit")
  }
}

impl From<usize> for FeelNumber {
  #[cfg(target_pointer_width = "64")]
  fn from(value: usize) -> Self {
    Self(bid128_from_uint64(value.try_into().unwrap()), false)
  }

  #[cfg(not(target_pointer_width = "64"))]
  fn from(value: usize) -> Self {
    compile_error!("Implement conversion from isize for other architectures as 64-bit")
  }
}

impl TryFrom<FeelNumber> for u8 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u8::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u8 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_uint32_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    n.try_into().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for i8 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i8::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i8 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_int32_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    n.try_into().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for u16 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u16::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u16 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_uint32_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    n.try_into().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for i16 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i16::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i16 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_int32_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    n.try_into().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for u32 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u32::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u32 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_uint32_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    Ok(n)
  }
}

impl TryFrom<FeelNumber> for i32 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i32::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i32 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_int32_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    Ok(n)
  }
}

impl TryFrom<FeelNumber> for u64 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u64::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u64 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_uint64_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    Ok(n)
  }
}

impl TryFrom<FeelNumber> for i64 {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i64::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i64 {
  type Error = DsntkError;

  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_int64_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    Ok(n)
  }
}

impl TryFrom<FeelNumber> for usize {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    usize::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for usize {
  type Error = DsntkError;

  #[cfg(target_pointer_width = "64")]
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_uint64_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    Ok(n.try_into().unwrap())
  }

  #[cfg(not(target_pointer_width = "64"))]
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    compile_error!("Implement conversion from usize for other architectures as 64-bit")
  }
}

impl TryFrom<FeelNumber> for isize {
  type Error = DsntkError;

  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    isize::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for isize {
  type Error = DsntkError;

  #[cfg(target_pointer_width = "64")]
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let mut flags = EXE_CLEAR;
    let n = bid128_to_int64_int(value.0, &mut flags);
    if flags != EXE_CLEAR {
      return Err(err_number_conversion_failed());
    }
    Ok(n.try_into().unwrap())
  }

  #[cfg(not(target_pointer_width = "64"))]
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    compile_error!("Implement conversion from isize for other architectures as 64-bit")
  }
}
