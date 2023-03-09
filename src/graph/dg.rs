use stack_graphs::arena::SupplementalArena;
use stack_graphs::arena::Handle;
use stack_graphs::graph::Node;

pub enum NodeKind {
    ClassDefinition(),
    FunctionDefinition(),
    PackageDefinition()
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            NodeKind::ClassDefinition() => return write!(f, "defclass"),
            NodeKind::FunctionDefinition() => return write!(f, "defun"),
            NodeKind::PackageDefinition() => return write!(f, "defpackage")
        }
    }
}

pub enum EdgeKind {
    MemberOf(),
    Contains()
}

impl std::fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            EdgeKind::MemberOf() => return write!(f, "memberOf"),
            EdgeKind::Contains() => return write!(f, "contains"),
        }
    }
}

struct NodeData {
    qualified_name: String,
    kind: NodeKind
}

impl std::fmt::Display for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "(node qualified-name: \"{}\" kind: {})",
               self.qualified_name, self.kind)
    }
}

impl NodeData {
    pub fn from(qualified_name: String,
                kind: NodeKind) -> NodeData {
        NodeData {
            qualified_name: qualified_name,
            kind: kind
        }
    }
}

pub struct EdgeData {
    source: Handle<Node>,
    sink: Handle<Node>,
    kind: EdgeKind
}

impl std::fmt::Display for EdgeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        todo!();
        write!(f, "");
    }
}

impl EdgeData {
    pub fn from() { todo!(); }
}

pub struct Graph {
    nodes: SupplementalArena<Node, NodeData>,
    edges: Vec<EdgeData>,
    node_edges: SupplementalArena<Node, Vec<EdgeData>>
}

pub fn main() {
    let node_data = NodeData::from(
        String::from("java.lang.Object"),
        NodeKind::ClassDefinition()
    );
    println!("{}", node_data);
}
