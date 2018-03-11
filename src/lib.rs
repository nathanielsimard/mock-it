#[cfg(test)]
#[macro_use]
extern crate table_test;

pub use mock::*;
pub use matcher::*;

mod rule;
mod output;
mod matcher;
mod mock;
