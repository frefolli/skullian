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
    Parameter,
    Attribute,
    Enum,
    Variant,
    Constant,
    Annotation,
    Nothing
}

impl Defkind {
    pub fn is_nothing(&self) -> bool {
        self == &Defkind::Nothing
    }

    pub fn from(defkind: String) -> Defkind {
        match defkind.as_str() {
            "package" => Self::Package,
            "class" => Self::Class,
            "interface" => Self::Interface,
            "function" => Self::Function,
            "parameter" => Self::Parameter,
            "attribute" => Self::Attribute,
            "enum" => Self::Enum,
            "variant" => Self::Variant,
            "constant" => Self::Constant,
            "annotation" => Self::Annotation,
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
                Self::Parameter => "parameter",
                Self::Attribute => "attribute",
                Self::Enum => "enum",
                Self::Variant => "variant",
                Self::Constant => "constant",
                Self::Annotation => "annotation",
                Self::Nothing => "ellipse"
            },
            "background-color": match self {
                Self::Package => "blue",
                Self::Class => "blue",
                Self::Function => "red",
                Self::Interface => "yellow",
                Self::Parameter => "red",
                Self::Attribute => "yellow",
                Self::Enum => "blue",
                Self::Variant => "variant",
                Self::Constant => "constant",
                Self::Annotation => "annotation",
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
            Self::Parameter => write!(f, "parameter"),
            Self::Attribute => write!(f, "attribute"),
            Self::Enum => write!(f, "enum"),
            Self::Variant => write!(f, "variant"),
            Self::Constant => write!(f, "constant"),
            Self::Annotation => write!(f, "annotation"),
            Self::Nothing => write!(f, "nothing")
        }
    }
}
