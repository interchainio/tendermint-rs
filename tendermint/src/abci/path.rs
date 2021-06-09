//! Paths to ABCI data

use crate::error::KindError as Error;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::{
    borrow::ToOwned,
    fmt::{self, Display},
    str::FromStr,
};

/// Path to ABCI data
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Path(String);

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(s.to_owned()))
    }
}
