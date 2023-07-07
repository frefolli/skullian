use std::collections::HashSet;
use std::{collections::HashMap, fmt::Display};
use stack_graphs::{arena::Handle, graph::Node};

use super::defkind::Defkind;
use super::dep_graph_node::DepGraphNode;
use super::dep_graph_edge::DepGraphEdge;
use super::edge_label::EdgeLabel;

pub struct DepGraph {
    names: HashMap<String, Handle<Node>>,
    nodes: HashMap<Handle<Node>, DepGraphNode>,
    edges: HashMap<Handle<Node>, Vec<DepGraphEdge>>,
    parents: HashMap<Handle<Node>, Handle<Node>>,
    childrens: HashMap<Handle<Node>, HashSet<Handle<Node>>>,
    typing: HashMap<Handle<Node>, Handle<Node>>,
    extension: HashMap<Handle<Node>, Handle<Node>>
}

impl DepGraph {
    pub fn new() -> DepGraph {
        DepGraph {
            names: HashMap::<String, Handle<Node>>::new(),
            nodes: HashMap::<Handle<Node>, DepGraphNode>::new(),
            edges: HashMap::<Handle<Node>, Vec<DepGraphEdge>>::new(),
            parents: HashMap::<Handle<Node>, Handle<Node>>::new(),
            childrens: HashMap::<Handle<Node>, HashSet<Handle<Node>>>::new(),
            typing: HashMap::<Handle<Node>, Handle<Node>>::new(),
            extension: HashMap::<Handle<Node>, Handle<Node>>::new()
        }
    }

    pub fn set_god(&mut self, root: Handle<Node>) {
        self.add_node(root, 
            DepGraphNode::new(
                String::from(""),
                String::from(""),
                Defkind::Nothing));
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

    pub fn set_parentship(&mut self, child: Handle<Node>, parent: Handle<Node>) {
        let current_defkind = self.get_node(&child).unwrap().get_defkind();
        let parent_defkind = self.get_node(&parent).unwrap().get_defkind();
        let edge_kind = match current_defkind == parent_defkind {
            true => EdgeLabel::NestedTo,
            false => EdgeLabel::DefinedBy
        };
        self.add_edge(
            DepGraphEdge::new(
                child,
                parent,
                edge_kind
        ));

        // child -> parent
        self.parents.insert(child, parent);
        
        // parent -> child
        if !self.childrens.contains_key(&parent) {
            self.childrens.insert(parent, HashSet::<Handle<Node>>::new());
        }
        self.childrens.get_mut(&parent).unwrap().insert(child);
    }

    pub fn set_inclusion(&mut self, source: Handle<Node>, sink: Handle<Node>) {
        if !self.childrens.contains_key(&source) {
            self.childrens.insert(source, HashSet::<Handle<Node>>::new());
        }
        self.childrens.get_mut(&source).unwrap().insert(sink);
    }

    pub fn set_type(&mut self, source: Handle<Node>, sink: Handle<Node>) {
        self.typing.insert(source, sink);
    }

    pub fn set_extension(&mut self, source: Handle<Node>, sink: Handle<Node>) {
        self.extension.insert(source, sink);
    }

    pub fn iter_children(&self, node: Handle<Node>) -> Option<&HashSet::<Handle<Node>>> {
        self.childrens.get(&node)
    }

    pub fn get_parent(&self, node: Handle<Node>) -> Option<&Handle<Node>> {
        self.parents.get(&node)
    }

    pub fn get_type(&self, node: Handle<Node>) -> Option<&Handle<Node>> {
        self.typing.get(&node)
    }

    pub fn get_extension(&self, node: Handle<Node>) -> Option<&Handle<Node>> {
        self.extension.get(&node)
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

    pub fn to_string(&self) -> String {
        let nodes_rep = self.nodes_to_string();
        let edges_rep = self.edges_to_string();
        let ss = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">
\t<key id=\"qualified_name\" for=\"node\" attr.name=\"qualified_name\" attr.type=\"string\">
\t\t<default>yellow</default>
\t</key>
\t<key id=\"kind\" for=\"node\" attr.name=\"kind\" attr.type=\"string\">
\t\t<default>yellow</default>
\t</key>
\t<key id=\"relationship\" for=\"edge\" attr.name=\"relationship\" attr.type=\"string\"/>
\t<graph id=\"dep-graph\" edgedefault=\"directed\">
{}
{}
\t</graph>
</graphml>
",
        nodes_rep, edges_rep);
        return ss.replace("\t", "  ");
    }
}

impl Display for DepGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "{}", self.to_string())
    }
}
