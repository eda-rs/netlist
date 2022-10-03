#![allow(dead_code)]

use std::result::Result;
mod error;
pub mod model;
mod ops;
mod parser;
mod saver;

pub use error::NetListError;
pub use model::{NetList, PinDirection};

pub(crate) type NResult<T> = Result<T, NetListError>;
