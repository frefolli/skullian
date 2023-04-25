use std::fmt::Display;

use stack_graphs::{arena::Handle, graph::Node};

use super::{dep_graph::DepGraph, edge_label::EdgeLabel};

pub struct DepGraphEdge {
    source: Handle<Node>,
    sink: Handle<Node>,
    label: EdgeLabel
}

impl DepGraphEdge {
    pub fn new(
        source: Handle<Node>,
        sink: Handle<Node>,
        label: EdgeLabel
    ) -> DepGraphEdge {
        DepGraphEdge {
            source: source,
            sink: sink,
            label: label
        }
    }

    pub fn get_source(&self) -> Handle<Node> {
        self.source
    }

    pub fn get_sink(&self) -> Handle<Node> {
        self.sink
    }

    pub fn get_label(&self) -> EdgeLabel {
        self.label.clone()
    }

    pub fn to_string(&self, graph: &DepGraph) -> String {
        let label = self.label.to_string();
        let source = graph.get_node(&self.source).unwrap().get_qualified_name();
        let target = graph.get_node(&self.sink).unwrap().get_qualified_name();
        format!("(edge {} {} {})",
            source, label, target)
    }

    pub fn to_json(&self, graph: &DepGraph) -> serde_json::value::Value {
        let label = self.label.to_string();
        let source = graph.get_node(&self.source).unwrap().get_qualified_name();
        let target = graph.get_node(&self.sink).unwrap().get_qualified_name();
        let id = format!("{} -> {}", source, target);
        serde_json::json!({
            "data": {
                "id": id,
                "source": source,
                "target": target,
                "label": label
            }
        })
    }

    pub fn display<'a>(&'a self, graph: &'a DepGraph) -> impl Display + 'a {
        DisplayDepGraphEdge {
            wrapped: self,
            graph,
        }
    }
}

struct DisplayDepGraphEdge<'a> {
    wrapped: &'a DepGraphEdge,
    graph: &'a DepGraph
}

impl <'a> Display for DisplayDepGraphEdge<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
            self.wrapped.to_string(self.graph))
    }
}
