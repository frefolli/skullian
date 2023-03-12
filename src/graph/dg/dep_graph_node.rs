use std::fmt::Display;

pub struct DepGraphNode {
    qualified_name: String
}

impl Display for DepGraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
            self.to_string()
        )
    }
}

impl DepGraphNode {
    pub fn new(qualified_name: String) -> DepGraphNode {
        DepGraphNode { qualified_name: qualified_name }
    }

    pub fn get_qualified_name(&self) -> String {
        self.qualified_name.clone()
    }

    pub fn to_string(&self) -> String {
        format!("(node \"{}\")",
            self.qualified_name)
    }
}