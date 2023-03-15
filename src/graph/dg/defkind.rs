use std::fmt::Display;

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Defkind {
    Package,
    Class,
    Interface,
    Function,
    Nothing
}

impl Defkind {
    pub fn from(defkind: String) -> Defkind {
        match defkind.as_str() {
            "package" => Self::Package,
            "class" => Self::Class,
            "interface" => Self::Interface,
            "function" => Self::Function,
            _ => Self::Nothing
        }
    }

    pub fn cytoscape_style(&self) -> Value {
        json!({
            "shape": match self {
                Self::Package => "hexagon",
                Self::Class => "hexagon",
                Self::Function => "rectangle",
                Self::Interface => "parallelogram",
                Self::Nothing => "ellipse"
            },
            "background-color": match self {
                Self::Package => "blue",
                Self::Class => "blue",
                Self::Function => "red",
                Self::Interface => "yellow",
                Self::Nothing => "green"
            }
        })
    }
}

impl Display for Defkind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Package => write!(f, "package"),
            Self::Class => write!(f, "class"),
            Self::Function => write!(f, "function"),
            Self::Interface => write!(f, "interface"),
            Self::Nothing => write!(f, "nothing")
        }
    }
}