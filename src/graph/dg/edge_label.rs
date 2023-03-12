use std::fmt::Display;

pub enum EdgeLabel {
    DefinedBy(),
    IsImplementationOf(),
    IsChildOf(),
    NestedTo()
}

impl EdgeLabel {
    pub fn to_string(&self) -> String {
        match self {
            Self::DefinedBy() => "definedBy".to_string(),
            Self::IsImplementationOf() => "isImplementationOf".to_string(),
            Self::IsChildOf() => "isChildOf".to_string(),
            Self::NestedTo() => "nestedTo".to_string()
        }
    }
}

impl Display for EdgeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Clone for EdgeLabel {
    fn clone(&self) -> Self {
        match self {
            Self::DefinedBy() => Self::DefinedBy(),
            Self::IsImplementationOf() => Self::IsImplementationOf(),
            Self::IsChildOf() => Self::IsChildOf(),
            Self::NestedTo() => Self::NestedTo(),
        }
    }
}