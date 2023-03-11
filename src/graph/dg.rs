use stack_graphs::graph::{StackGraph, Node};
use stack_graphs::arena::{Handle, SupplementalArena};
use std::ops::Index;

struct EdgeData {
    source: Handle<Node>,
    sink: Handle<Node>,
    label: String
}

struct VisitContext {
    qualified_names: SupplementalArena::<Node, String>,
    edges: SupplementalArena<Node, Vec<EdgeData>>,
    current_parent: Option<Handle<Node>>
}

impl VisitContext {
    pub fn new() -> VisitContext {
        VisitContext {
            qualified_names: SupplementalArena::<Node, String>::new(),
            edges: SupplementalArena::<Node, Vec<EdgeData>>::new(),
            current_parent: None
        }
    }

    pub fn is_not_defined(&self, node_handle: Handle<Node>) -> bool {
        self.qualified_names.get(node_handle).is_none()
    }

    pub fn is_defined(&self, node_handle: Handle<Node>) -> bool {
        self.qualified_names.get(node_handle).is_some()
    }

    pub fn get_qualified_name(&self, node_handle: Handle<Node>) -> Option<&String> {
        self.qualified_names.get(node_handle)
    }

    pub fn set_qualified_name(&mut self, node_handle: Handle<Node>, qualified_name: &String) {
        self.qualified_names[node_handle] = qualified_name.clone();
    }

    pub fn get_current_parent(&self) -> Option<Handle<Node>> {
        self.current_parent
    }

    pub fn set_current_parent(&mut self, parent: Option<Handle<Node>>) {
        self.current_parent = parent;
    }

    pub fn add_edge(&mut self, edge: EdgeData) {
        self.edges[edge.source].push(edge);
    }

    pub fn edge_to_string(&self, edge: &EdgeData) -> String {
        format!("(edge \"{}\" \"{}\" \"{}\")",
            self.get_qualified_name(edge.source).unwrap(),
            edge.label,
            self.get_qualified_name(edge.sink).unwrap()
        )
    }

    pub fn node_to_string(&self, node: Handle<Node>) -> String {
        format!("(node \"{}\")",
            self.get_qualified_name(node).unwrap()
        )
    }
}

fn walk_stack_graph_impl(
        visit_context: &mut VisitContext,
        stack_graph: &StackGraph,
        current_node: Handle<Node>,
        scope_prefix: &String
) {
    if visit_context.is_defined(current_node) {
        return;
    }

    let mut current_scope_prefix = scope_prefix.clone();
    if stack_graph.index(current_node).is_definition() {
        current_scope_prefix += stack_graph.index(stack_graph.index(current_node).symbol().unwrap());
        visit_context.set_qualified_name(current_node, &current_scope_prefix);
        current_scope_prefix += ".";
        println!("{}", visit_context.node_to_string(current_node));
    } else {
        visit_context.set_qualified_name(current_node, &String::from(""));
    }
    
    let current_parent = visit_context.get_current_parent();
    if stack_graph.index(current_node).is_definition() {
        if current_parent.is_some() {
            let contains_edge = EdgeData { source: current_parent.unwrap(), sink: current_node, label: String::from("contains") };
            let member_of_edge = EdgeData { source: current_node, sink: current_parent.unwrap(), label: String::from("memberOf") };
            println!("{}", visit_context.edge_to_string(&contains_edge));
            println!("{}", visit_context.edge_to_string(&member_of_edge));
            visit_context.add_edge(contains_edge);
            visit_context.add_edge(member_of_edge);
        }

        visit_context.set_current_parent(Some(current_node));
    }
    for edge in stack_graph.outgoing_edges(current_node) {
        if visit_context.is_not_defined(edge.sink) {
            walk_stack_graph_impl(visit_context, stack_graph, edge.sink, &current_scope_prefix);
        }
    }
    if stack_graph.index(current_node).is_definition() {
        visit_context.set_current_parent(current_parent);
    }
}
pub fn walk_stack_graph(stack_graph: &StackGraph) {
    let mut visit_context = VisitContext::new();
    let root_node = StackGraph::root_node();
    let scope_prefix = String::from("");
    walk_stack_graph_impl(&mut visit_context, stack_graph, root_node, &scope_prefix);
}