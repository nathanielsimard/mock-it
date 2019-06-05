#[cfg(test)]
#[macro_use]
extern crate table_test;

pub use mock_it_codegen::mock_it;

pub use crate::matcher::*;
pub use crate::mock::*;
pub use crate::validator::verify;

mod given;
mod matcher;
mod mock;
mod rule;
mod validator;
