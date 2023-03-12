use std::{collections::HashMap, fmt::Display};
use stack_graphs::{arena::Handle, graph::Node};

use super::dep_graph_node::DepGraphNode;
use super::dep_graph_edge::DepGraphEdge;

pub struct DepGraph {
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

    pub fn nodes_to_json(&self) -> Vec<serde_json::value::Value> {
        Vec::<serde_json::value::Value>::from_iter(
            self.iter_nodes().map(
                |(_,data)| data.to_json()
            )
        )
    }

    pub fn edges_to_json(&self) -> Vec<serde_json::value::Value> {
        let mut edges_data = Vec::<serde_json::value::Value>::new();
        for (_, edges) in self.iter_edges() {
            for edge in edges.iter() {
                edges_data.push(edge.to_json(self));
            }
        }
        edges_data
    }

    pub fn to_json(&self) -> serde_json::value::Value {
        let mut nodes_data = self.nodes_to_json();
        let mut edges_data = self.edges_to_json();
        nodes_data.append(&mut edges_data);
        serde_json::json!(nodes_data)
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