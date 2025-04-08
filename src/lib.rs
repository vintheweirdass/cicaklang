#![forbid(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![no_std]
extern crate alloc;

pub mod error;
pub mod lex;
pub mod util;
pub mod consts;