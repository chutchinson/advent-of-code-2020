pub use std::ops::Range;
pub use std::io::prelude::*;
pub use std::io::{Error, ErrorKind};
pub use itertools::Itertools;
pub use crate::context::Context;

pub type Result = std::io::Result<()>;