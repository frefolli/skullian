/**
 * TASK
 * 
 * define a module for defining the Dependency Graph
 * 
 * The algorithm should walk through the StackGraph, resolve name bindings and 
 * 
 */
pub mod edge_label;
pub mod defkind;
pub mod refkind;
pub mod dep_graph;
pub mod dep_graph_node;
pub mod dep_graph_edge;
pub mod testing;
pub mod synchro;
pub mod fun;
pub mod references;
use std::ops::Index;

use stack_graphs::{graph::{StackGraph, Node}, arena::Handle};
use dep_graph::DepGraph;

use self::{dep_graph_node::DepGraphNode, defkind::Defkind, synchro::{SynchroExplorer}};

fn find_debug_info(
    stack_graph: &StackGraph,
    node_handle: Handle<Node>,
    key: String
) -> Option<String> {
    let debug_infos = stack_graph.debug_info(node_handle)?;
    for entry in debug_infos.iter() {
        if key == stack_graph.index(entry.key) {
            return Some(stack_graph.index(entry.value).to_string());
        }
    }
    None
}

fn walk_step(
    explorer: &mut SynchroExplorer,
    dep_graph: &mut DepGraph,
    stack_graph: &StackGraph
) {
    let mut delimiter: &str = ".";
    let current_node = explorer.get_current_node().unwrap();
    let current_parent = explorer.get_parent_node();
    let mut next_parent = explorer.get_parent_node();
    let scope_prefix = explorer.get_scope_prefix();
    if explorer.is_not_visited(current_node) {
        // MARK PHASE
        explorer.visit(current_node);
        let concrete_node = stack_graph.index(current_node);
        if concrete_node.is_definition() {
            let defkind = Defkind::from(
                find_debug_info(
                    stack_graph,
                    current_node,
                    "defkind".to_string()
                ).unwrap_or_default()
            );
            if ! defkind.is_nothing() {
                if defkind == Defkind::File {
                    delimiter = "::";
                }
                let mut symbol = stack_graph.index(concrete_node.symbol().unwrap()).to_string();
                let qualified_name = format!("{}{}", scope_prefix, symbol);
                
                if defkind == Defkind::Package {
                    symbol = qualified_name.clone();
                }

                // check if definition is a justified duplicated (ex: package declaration at top level)
                let defnode = dep_graph.get_node_by_name(&qualified_name);
                let actual_defnode : &Handle<Node>;
                if defnode.is_some() {
                    next_parent = defnode.copied();
                    actual_defnode = defnode.unwrap();
                } else {
                    dep_graph.add_node(
                        current_node,
                        DepGraphNode::new(
                            symbol,
                            qualified_name.clone(),
                            defkind
                        ));
                    dep_graph.add_name(current_node, qualified_name);
                    actual_defnode = &current_node;
                    next_parent = Some(current_node);
                }

                if current_parent.is_some() {
                    dep_graph.set_parentship(*actual_defnode, current_parent.unwrap());
                }
            }
        } else if concrete_node.is_reference() {
            match current_parent {
                Some(parent) => {
                    if references::valid_reference(stack_graph, current_node) {
                        references::unroll_reference(explorer, stack_graph, parent, current_node);
                    }
                },
                None => ()
            };
        }

        // PREPARE PHASE
        if concrete_node.is_definition() {
            let symbol = stack_graph.index(concrete_node.symbol().unwrap()).to_string();
            explorer.set_scope_prefix(format!("{}{}{}", scope_prefix, symbol, delimiter));
            explorer.set_parent_node(next_parent);
        }
        
        // RECURSIVE PHASE
        for edge in stack_graph.outgoing_edges(current_node) {
            let sink = edge.sink;
            if explorer.is_not_visited(sink) {
                explorer.set_current_node(Some(sink));
                walk_step(explorer, dep_graph, stack_graph);
            }
        }

        // END PHASE
        if concrete_node.is_definition() {
            explorer.set_parent_node(current_parent);
            explorer.set_scope_prefix(scope_prefix);
        }
    }
}

fn save_to_data_string(output_file: &std::path::Path, dep_graph: &DepGraph) {
    std::fs::write(
        output_file,
        dep_graph.to_string()
    ).unwrap();
}

fn save_to_data_json(output_file: &std::path::Path, dep_graph: &DepGraph) {
    std::fs::write(
        output_file,
        dep_graph.to_json().to_string()
    ).unwrap();
}

pub fn build_dep_graph(
    dep_graph: &mut DepGraph,
    output_file: &std::path::Path,
    stack_graph: &StackGraph
) {
    let mut explorer = SynchroExplorer::new();
    dep_graph.set_god(stack_graphs::graph::StackGraph::root_node());
    explorer.set_current_node(Some(stack_graphs::graph::StackGraph::root_node()));
    explorer.set_parent_node(Some(stack_graphs::graph::StackGraph::root_node()));
    walk_step(&mut explorer, dep_graph, stack_graph);
    log::info!("Explorer is_done_with exploring graph");
    references::resolve_references(&explorer, dep_graph);
    log::info!("Explorer is_done_with resolving_references");
    if output_file.as_os_str() != "" {
        save_to_data_string(output_file, dep_graph);
        log::info!("Explorer is_done_with saving_graph_to_json");
    }
    fun::fun_facts(&dep_graph);
}