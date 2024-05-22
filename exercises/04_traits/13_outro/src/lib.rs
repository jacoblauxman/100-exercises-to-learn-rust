// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u8> for SaturatingU16 {
    fn from(val: u8) -> Self {
        Self { value: val.into() }
    }
}

// impl From<&u8> for SaturatingU16 {
//     fn from(val: &u8) -> Self {
//         Self {
//             value: (*val).into(),
//         }
//     }
// }
impl From<&u8> for SaturatingU16 {
    fn from(val: &u8) -> Self {
        (*val).into() // can be simplified due to `u8` impl above (already defined to return `SaturatingU16 { val }` so only deref needed)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(val: u16) -> Self {
        Self { value: val }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(val: &u16) -> Self {
        (*val).into()
    }
}

impl Add for SaturatingU16 {
    // type Output = SaturatingU16;
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    // type Output = SaturatingU16;
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        // self + rhs.value
        self + *rhs
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        let value = self.value.saturating_add(rhs);
        Self { value }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &u16) -> Self::Output {
        let value = self.value.saturating_add(*rhs);
        Self { value }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
