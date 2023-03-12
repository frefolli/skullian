use std::fmt::Display;

use stack_graphs::{arena::Handle, graph::Node};

use super::dep_graph::DepGraph;

pub struct DepGraphEdge {
    source: Handle<Node>,
    sink: Handle<Node>
}

impl DepGraphEdge {
    pub fn new(
        source: Handle<Node>,
        sink: Handle<Node>
    ) -> DepGraphEdge {
        DepGraphEdge {
            source: source,
            sink: sink
        }
    }

    pub fn get_source(&self) -> Handle<Node> {
        self.source
    }

    pub fn get_sink(&self) -> Handle<Node> {
        self.sink
    }

    pub fn to_string(&self, graph: &DepGraph) -> String {
        format!("(edge\n\t\"{}\"\n\t\"{}\")",
            graph.get_node(self.source).unwrap(),
            graph.get_node(self.sink).unwrap())
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