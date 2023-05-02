use std::fmt::Display;
use super::defkind::Defkind;
pub struct DepGraphNode {
    qualified_name: String,
    defkind: Defkind
}

impl Display for DepGraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
            self.to_string()
        )
    }
}

impl DepGraphNode {
    pub fn new(
        qualified_name: String,
        defkind: Defkind
    ) -> DepGraphNode {
        DepGraphNode {
            qualified_name: qualified_name,
            defkind: defkind
        }
    }

    pub fn get_qualified_name(&self) -> String {
        self.qualified_name.clone()
    }

    pub fn get_defkind(&self) -> Defkind {
        self.defkind.clone()
    }

    pub fn to_string(&self) -> String {
        format!("
\t\t<node id=\"{}\">
\t\t\t<data id=\"qualified_name\">{}</data>
\t\t\t<data id=\"kind\">{}</data>
\t\t</node>",
            self.qualified_name,
            self.qualified_name,
            self.defkind
        )
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "data": {
                "id": self.qualified_name
            },
            "style": self.defkind.cytoscape_style()
        })
    }
}
