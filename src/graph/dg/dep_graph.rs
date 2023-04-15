use std::{collections::HashMap, fmt::Display};
use stack_graphs::{arena::Handle, graph::Node};

use super::defkind::Defkind;
use super::dep_graph_node::DepGraphNode;
use super::dep_graph_edge::DepGraphEdge;

pub struct DepGraph {
    names: HashMap<String, Handle<Node>>,
    nodes: HashMap<Handle<Node>, DepGraphNode>,
    edges: HashMap<Handle<Node>, Vec<DepGraphEdge>>
}

impl DepGraph {
    pub fn new() -> DepGraph {
        DepGraph {
            names: HashMap::<String, Handle<Node>>::new(),
            nodes: HashMap::<Handle<Node>, DepGraphNode>::new(),
            edges: HashMap::<Handle<Node>, Vec<DepGraphEdge>>::new()
        }
    }

    pub fn get_node(&self, node_handle: &Handle<Node>) -> Option<&DepGraphNode> {
        self.nodes.get(node_handle)
    }

    pub fn get_edges(&self, node_handle: &Handle<Node>) -> Option<&Vec<DepGraphEdge>> {
        self.edges.get(node_handle)
    }

    pub fn add_node(&mut self, node_handle: Handle<Node>, data: DepGraphNode) {
        self.nodes.insert(node_handle, data);
        self.edges.insert(node_handle, Vec::<DepGraphEdge>::new());
    }

    pub fn add_edge(&mut self, edge: DepGraphEdge) {
        self.edges.get_mut(&edge.get_source()).unwrap().push(edge);
    }

    pub fn add_name(&mut self, node_handle: Handle<Node>, qualified_name: String) {
        self.names.insert(qualified_name, node_handle);
    }

    pub fn iter_nodes(&self) -> std::collections::hash_map::Iter<Handle<Node>, DepGraphNode> {
        self.nodes.iter()
    }

    pub fn iter_edges(&self) -> std::collections::hash_map::Iter<Handle<Node>, Vec<DepGraphEdge>> {
        self.edges.iter()
    }

    pub fn nodes_to_string(&self) -> String {
        let mut strings = Vec::<String>::new();
        for (_, data) in self.iter_nodes() {
                match data.get_defkind() == Defkind::Nothing {
                    true => (),
                    false => {
                        strings.push(data.to_string());
                    }
                }
        }
        strings.join("\n")
    }

    pub fn edges_to_string(&self) -> String {
        let mut strings = Vec::<String>::new();
        for (_, edges) in self.iter_edges() {
            for edge in edges {
                match self.get_node(&edge.get_source()) {
                    Some(source_node) => {
                        match self.get_node(&edge.get_sink()) {
                            Some(sink_node) => {
                                if ! source_node.get_defkind().is_nothing() {
                                    if ! sink_node.get_defkind().is_nothing() {
                                        strings.push(edge.to_string(self));
                                    }
                                }
                            },
                            None => log::warn!("edge with no sink")
                        }
                    }
                    None => log::warn!("edge with no source")
                }
            }
        }
        strings.join("\n")
    }

    pub fn nodes_to_json(&self) -> Vec<serde_json::value::Value> {
        let mut jsons = Vec::<serde_json::value::Value>::new();
        for (_, data) in self.iter_nodes() {
            match data.get_defkind() == Defkind::Nothing {
                true => (),
                false => {
                    jsons.push(data.to_json());
                }
            }
        }
        jsons
    }

    pub fn edges_to_json(&self) -> Vec<serde_json::value::Value> {
        let mut jsons = Vec::<serde_json::value::Value>::new();
        for (_, edges) in self.iter_edges() {
            for edge in edges {
                match self.get_node(&edge.get_source()) {
                    Some(source_node) => {
                        match self.get_node(&edge.get_sink()) {
                            Some(sink_node) => {
                                if ! source_node.get_defkind().is_nothing() {
                                    if ! sink_node.get_defkind().is_nothing() {
                                        jsons.push(edge.to_json(self));
                                    }
                                }
                            },
                            None => log::warn!("edge with no sink")
                        }
                    }
                    None => log::warn!("edge with no source")
                }
            }
        }
        jsons
    }

    pub fn to_json(&self) -> serde_json::value::Value {
        let mut nodes_data = self.nodes_to_json();
        let mut edges_data = self.edges_to_json();
        nodes_data.append(&mut edges_data);
        serde_json::json!(nodes_data)
    }

    pub fn get_node_by_name(&self, name: &String) -> Option<&Handle<Node>> {
        self.names.get(name)
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