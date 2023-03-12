use std::fmt::Display;

use serde_json::{Value, json};

#[derive(PartialEq)]
pub enum Defkind {
    Class(),
    Interface(),
    Function(),
    Nothing()
}

impl Defkind {
    pub fn from(defkind: String) -> Defkind {
        match defkind.as_str() {
            "class" => Self::Class(),
            "interface" => Self::Interface(),
            "function" => Self::Function(),
            _ => Self::Nothing()
        }
    }

    pub fn cytoscape_style(&self) -> Value {
        json!({
            "shape": match self {
                Self::Class() => "hexagon",
                Self::Function() => "rectangle",
                Self::Interface() => "parallelogram",
                Self::Nothing() => "ellipse"
            },
            "background-color": match self {
                Self::Class() => "blue",
                Self::Function() => "red",
                Self::Interface() => "yellow",
                Self::Nothing() => "green"
            }
        })
    }
}

impl Display for Defkind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Class() => write!(f, "class"),
            Self::Function() => write!(f, "function"),
            Self::Interface() => write!(f, "interface"),
            Self::Nothing() => write!(f, "nothing")
        }
    }
}

impl Clone for Defkind {
    fn clone(&self) -> Self {
        match self {
            Self::Class() => Self::Class(),
            Self::Function() => Self::Function(),
            Self::Interface() => Self::Interface(),
            Self::Nothing() => Self::Nothing(),
        }
    }
}