use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Refkind {
    Extends,
    Implements,
    Includes,
    UsesType,
    AccessField,
    Nothing
}

impl Refkind {
    pub fn from(refkind: String) -> Refkind {
        match refkind.as_str() {
            "extends" => Self::Extends,
            "implements" => Self::Implements,
            "includes" => Self::Includes,
            "usesType" => Self::UsesType,
            "accessField" => Self::AccessField,
            _ => Self::Nothing
        }
    }
}

impl Display for Refkind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Extends => write!(f, "extends"),
            Self::Implements => write!(f, "implements"),
            Self::Includes => write!(f, "includes"),
            Self::UsesType => write!(f, "usesType"),
            Self::AccessField => write!(f, "accessField"),
            Self::Nothing => write!(f, "nothing")
        }
    }
}