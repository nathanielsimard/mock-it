#[cfg(test)]
#[macro_use]
extern crate table_test;

pub use matcher::*;
pub use mock::*;
pub use validator::verify;

mod matcher;
mod mock;
mod output;
mod rule;
mod validator;
