#![allow(dead_code)]
use std::error::Error;
use std::result::Result;
mod error;
pub mod model;
mod ops;
mod parser;
mod saver;

pub use model::NetList;

pub type NResult<T> = Result<T, Box<dyn Error>>;
