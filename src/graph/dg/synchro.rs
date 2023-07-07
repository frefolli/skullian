use std::collections::{HashSet, HashMap};

use stack_graphs::{arena::Handle, graph::{Node, StackGraph}};

use super::{refkind::Refkind, dep_graph::DepGraph};

#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceQuery {
    source: Handle<Node>,
    sink: Vec<String>,
    kind: Refkind
}

impl ReferenceQuery {
    pub fn get_source(&self) -> Handle<Node> {
        return self.source;
    }
    pub fn get_sink(&self) -> Vec<String> {
        return self.sink.clone();
    }
    pub fn get_kind(&self) -> Refkind {
        return self.kind.clone();
    }
}

pub struct SynchroExplorer {
    reference_extension: HashSet<ReferenceQuery>,
    reference_includes: HashSet<ReferenceQuery>,
    reference_uses_type: HashSet<ReferenceQuery>,
    reference_queries: HashSet<ReferenceQuery>,
    name_bindings: HashMap<Handle<Node>, Handle<Node>>,
    visited_modes: HashMap<Handle<Node>, bool>,
    current_node: Option<Handle<Node>>,
    parent_node: Option<Handle<Node>>,
    scope_prefix: String
}

impl SynchroExplorer {
    pub fn new() -> SynchroExplorer {
        SynchroExplorer {
            reference_extension: HashSet::<ReferenceQuery>::new(),
            reference_includes: HashSet::<ReferenceQuery>::new(),
            reference_uses_type: HashSet::<ReferenceQuery>::new(),
            reference_queries: HashSet::<ReferenceQuery>::new(),
            name_bindings: HashMap::<Handle<Node>, Handle<Node>>::new(),
            visited_modes: HashMap::<Handle<Node>, bool>::new(),
            current_node: None,
            parent_node: None,
            scope_prefix: String::from("")
        }
    }

    pub fn get_current_node(&self) -> Option<Handle<Node>> {
        self.current_node
    }

    pub fn set_current_node(&mut self, node_handle: Option<Handle<Node>>) {
        self.current_node = node_handle
    }

    pub fn get_parent_node(&self) -> Option<Handle<Node>> {
        self.parent_node
    }

    pub fn set_parent_node(&mut self, node_handle: Option<Handle<Node>>) {
        self.parent_node = node_handle
    }

    pub fn get_scope_prefix(&self) -> String {
        self.scope_prefix.clone()
    }

    pub fn set_scope_prefix(&mut self, scope_prefix: String) {
        self.scope_prefix = scope_prefix;
    }

    pub fn is_not_visited(&self, node_handle: Handle<Node>) -> bool {
        self.visited_modes.get(&node_handle).is_none()
    }

    pub fn visit(&mut self, node_handle: Handle<Node>) {
        self.visited_modes.insert(node_handle, true);
    }

    pub fn set_name_binding(&mut self, source: Handle<Node>, sink: Handle<Node>) {
        self.name_bindings.insert(source, sink);
    }

    pub fn get_name_binding(&mut self, source: Handle<Node>) -> Option<&Handle<Node>> {
        self.name_bindings.get(&source)
    }

    pub fn add_reference_query(&mut self, source: Handle<Node>, sink: Vec<String>, kind: Refkind) {
        self.reference_queries.insert(ReferenceQuery { source: source, sink: sink, kind: kind });
    }

    pub fn iter_reference_queries(&self) -> std::collections::hash_set::Iter<'_, ReferenceQuery> {
        self.reference_queries.iter()
    }

    pub fn number_of_reference_queries(&self) -> usize {
        self.reference_queries.len()
    }

    pub fn add_reference_uses_type(&mut self, source: Handle<Node>, sink: Vec<String>, kind: Refkind) {
        self.reference_uses_type.insert(ReferenceQuery { source: source, sink: sink, kind: kind });
    }

    pub fn iter_reference_uses_type(&self) -> std::collections::hash_set::Iter<'_, ReferenceQuery> {
        self.reference_uses_type.iter()
    }

    pub fn number_of_reference_uses_type(&self) -> usize {
        self.reference_uses_type.len()
    }

    pub fn add_reference_includes(&mut self, source: Handle<Node>, sink: Vec<String>, kind: Refkind) {
        self.reference_includes.insert(ReferenceQuery { source: source, sink: sink, kind: kind });
    }

    pub fn iter_reference_includes(&self) -> std::collections::hash_set::Iter<'_, ReferenceQuery> {
        self.reference_includes.iter()
    }

    pub fn number_of_reference_includes(&self) -> usize {
        self.reference_includes.len()
    }

    pub fn add_reference_extension(&mut self, source: Handle<Node>, sink: Vec<String>, kind: Refkind) {
        self.reference_extension.insert(ReferenceQuery { source: source, sink: sink, kind: kind });
    }

    pub fn iter_reference_extension(&self) -> std::collections::hash_set::Iter<'_, ReferenceQuery> {
        self.reference_extension.iter()
    }

    pub fn number_of_reference_extension(&self) -> usize {
        self.reference_extension.len()
    }
}