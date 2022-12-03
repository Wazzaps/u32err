//! # u32err
//! This crate implements the [`core::ops::Try`] trait with a thin wrapper over [`u32`].
//!
//! You may use it to implement ergonomic error handling for FFI functions that return non-zero values on failure,
//! or as a lightweight [`Result`].
//!
//! ## Example
//!
//! ```no_run
//! use u32err::ErrCode;
//! extern "C" {
//!     /// This is a function that does something (via FFI).
//!     ///
//!     /// It returns either a 0 on success, or a non-zero number on failure.
//!     /// The real FFI signature of this function returns [`u32`], but the types are compatible.
//!     fn returns_zero_on_success() -> ErrCode;
//! }
//!
//! fn foo() -> ErrCode {
//!     unsafe {
//!         returns_zero_on_success()?;
//!     }
//!     ErrCode(0)
//! }
//! ```
#![feature(try_trait_v2)]
#![cfg_attr(not(test), no_std)]

use core::convert::Infallible;
use core::fmt::{Debug, Formatter};
use core::num::NonZeroU32;
use core::ops::{ControlFlow, FromResidual, Try};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
#[must_use]
pub struct ErrCode(pub u32);

impl ErrCode {
    pub fn is_ok(&self) -> bool {
        self.0 == 0
    }

    pub fn is_err(&self) -> bool {
        self.0 != 0
    }

    #[track_caller]
    pub fn unwrap(&self) {
        if self.is_err() {
            panic!("Error: {:?}", self.0);
        }
    }

    #[track_caller]
    pub fn expect(&self, msg: &str) {
        if self.is_err() {
            panic!("[{:?}] {}", self.0, msg);
        }
    }
}

impl Debug for ErrCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "E{}", self.0)
    }
}

pub struct ErrCodeResidual(NonZeroU32);

impl Try for ErrCode {
    type Output = ();
    type Residual = ErrCodeResidual;

    fn from_output(_: Self::Output) -> Self {
        ErrCode(0)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match NonZeroU32::new(self.0) {
            Some(r) => ControlFlow::Break(ErrCodeResidual(r)),
            None => ControlFlow::Continue(()),
        }
    }
}

impl From<u32> for ErrCode {
    fn from(val: u32) -> Self {
        ErrCode(val)
    }
}

impl FromResidual for ErrCode {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        ErrCode(residual.0.into())
    }
}

impl FromResidual<Result<Infallible, ErrCode>> for ErrCode {
    fn from_residual(residual: Result<Infallible, ErrCode>) -> Self {
        match residual {
            Err(err) => err,
            Ok(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn inner() -> ErrCode {
            ErrCode(0)?;
            ErrCode(1)?;
            ErrCode(123)
        }
        assert_eq!(inner(), ErrCode(1));
    }
}
