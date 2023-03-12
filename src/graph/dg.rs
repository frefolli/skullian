/**
 * TASK
 * 
 * define a module for defining the Dependency Graph
 * 
 * The algorithm should walk through the StackGraph, resolve name bindings and 
 * 
 */
use stack_graphs::arena::Handle;
use stack_graphs::graph::{Node, StackGraph};
use std::collections::HashMap;
use std::fmt::{Display};
use std::ops::Index;

struct DepGraphEdge {
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

struct DepGraph {
    nodes: HashMap<Handle<Node>, DepGraphNode>,
    edges: HashMap<Handle<Node>, Vec<DepGraphEdge>>
}

impl DepGraph {
    pub fn new() -> DepGraph {
        DepGraph {
            nodes: HashMap::<Handle<Node>, DepGraphNode>::new(),
            edges: HashMap::<Handle<Node>, Vec<DepGraphEdge>>::new()
        }
    }

    pub fn from(stack_graph: &StackGraph) -> DepGraph {
        let mut dep_graph = Self::new();
        for node_handle in stack_graph.iter_nodes() {
            if stack_graph.index(node_handle).is_definition() {
                dep_graph.add_node(
                    node_handle,
                    DepGraphNode::new(
                        stack_graph.index(
                            stack_graph.index(node_handle).symbol().unwrap()
                        ).to_string()
                    ));
            }
        }
        dep_graph
    }

    pub fn get_node(&self, node_handle: Handle<Node>) -> Option<&DepGraphNode> {
        self.nodes.get(&node_handle)
    }

    pub fn get_edges(&self, node_handle: Handle<Node>) -> Option<&Vec<DepGraphEdge>> {
        self.edges.get(&node_handle)
    }

    pub fn add_node(&mut self, node_handle: Handle<Node>, data: DepGraphNode) {
        self.nodes.insert(node_handle, data);
        self.edges.insert(node_handle, Vec::<DepGraphEdge>::new());
    }

    pub fn add_edge(&mut self, edge: DepGraphEdge) {
        self.edges.get_mut(&edge.get_source()).unwrap().push(edge);
    }

    pub fn iter_nodes(&self) -> std::collections::hash_map::Iter<Handle<Node>, DepGraphNode> {
        self.nodes.iter()
    }

    pub fn iter_edges(&self) -> std::collections::hash_map::Iter<Handle<Node>, Vec<DepGraphEdge>> {
        self.edges.iter()
    }

    pub fn nodes_to_string(&self) -> String {
        Vec::<String>::from_iter(
            self.iter_nodes().map(
                |(_,data)| data.to_string()
            )
        ).join("\n")
    }

    pub fn edges_to_string(&self) -> String {
        let mut strings = Vec::<String>::from_iter(
            self.iter_edges().map(
                |(_,edges)| {
                    if edges.len() > 0 {
                        return Vec::<String>::from_iter(
                            edges.iter().map(
                                |edge| {
                                    edge.to_string(self)
                                }
                            )
                        ).join("\n")
                    }
                    String::from("")
                }
            )
        );
        strings.retain(|ss| ss.len()> 0);
        strings.join("\n")
    }
}

impl Display for DepGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let nodes_rep = self.nodes_to_string();
        let edges_rep = self.edges_to_string();

        if nodes_rep.len() > 0 {
            write!(f, "{}",
                nodes_rep)?;
            if edges_rep.len() > 0 {
                write!(f, "\n{}",
                edges_rep)?;
            }
        }
        write!(f, "")
    }
}

pub fn todo(stack_graph: &StackGraph) {
    let dep_graph = DepGraph::from(stack_graph);
    println!("{}", dep_graph);
}