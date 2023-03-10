use std::fmt::Display;

pub enum Refkind {
    Extends(),
    Implements(),
    Nothing()
}

impl Refkind {
    pub fn from(refkind: String) -> Refkind {
        match refkind.as_str() {
            "extends" => Self::Extends(),
            "implements" => Self::Implements(),
            _ => Self::Nothing()
        }
    }
}

impl Display for Refkind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Extends() => write!(f, "extends"),
            Self::Implements() => write!(f, "implements"),
            Self::Nothing() => write!(f, "nothing")
        }
    }
}

impl Clone for Refkind {
    fn clone(&self) -> Self {
        match self {
            Self::Extends() => Self::Extends(),
            Self::Implements() => Self::Implements(),
            Self::Nothing() => Self::Nothing(),
        }
    }
}