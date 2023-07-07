use std::{ops::Index, collections::HashSet};
use std::collections::hash_set::Iter;
use stack_graphs::{graph::{StackGraph, Node}, arena::Handle};
use stack_graphs::graph::Node::PushSymbol;
use super::defkind::Defkind;
use super::{synchro::{SynchroExplorer, ReferenceQuery}, refkind::Refkind, find_debug_info, dep_graph::DepGraph};

pub fn unroll_reference(
    explorer: &mut SynchroExplorer,
    stack_graph: &StackGraph,
    source: Handle<Node>,
    reference: Handle<Node>) {
    let mut node_handle = reference;
    let kind = 
    Refkind::from(find_debug_info(
        stack_graph,
        node_handle,
        "refkind".to_string()
    ).unwrap_or_default());
    let mut symbols = Vec::<String>::new();
    symbols.push(stack_graph.index(stack_graph.index(node_handle).symbol().unwrap()).to_string());
    loop {
        let mut found = false;
        for edge in stack_graph.outgoing_edges(node_handle) {
            match stack_graph.index(edge.sink).symbol() {
                Some(obj) => {
                    found = true;
                    symbols.push(stack_graph.index(obj).to_string());
                    node_handle = edge.sink;
                    break;
                },
                None => {}
            }
        }
        if !(found) {
            break;
        }
    }
    match kind {
        Refkind::Includes => {
            explorer.add_reference_includes(source, symbols, kind);
        },
        Refkind::UsesType => {
            explorer.add_reference_uses_type(source, symbols, kind);
        },
        Refkind::Implements|Refkind::Extends => {
            explorer.add_reference_extension(source, symbols, kind);
        },
        _ => {
            explorer.add_reference_query(source, symbols, kind);
        }
    }
    
}

pub fn valid_reference(
    stack_graph: &StackGraph,
    reference: Handle<Node>
) -> bool {
    let mut ok = false;
    match stack_graph.index(reference) {
        PushSymbol(_) => {
            if stack_graph.index(reference).is_reference() {
                let refkind = Refkind::from(find_debug_info(
                    stack_graph,
                    reference,
                    "refkind".to_string()
                ).unwrap_or_default());
                if ! refkind.is_nothing() {
                    ok = true;
                }
            }
        },
        _ => {}
    }
    return ok;
}

fn children(dep_graph: &DepGraph, node: Handle<Node>) -> HashSet<Handle<Node>> {
    match dep_graph.iter_children(node) {
        Some(arr) => arr.clone(),
        None => HashSet::<Handle<Node>>::new()
    }
}

fn name(dep_graph: &DepGraph, node: Handle<Node>) -> String {
    dep_graph.get_node(&node).unwrap().get_proper_name()
}

fn parent(dep_graph: &DepGraph, node: Handle<Node>) -> Option<Handle<Node>> {
    dep_graph.get_parent(node).copied()
}

fn get_type(dep_graph: &DepGraph, node: Handle<Node>) -> Option<Handle<Node>> {
    dep_graph.get_type(node).copied()
}

fn get_extension(dep_graph: &DepGraph, node: Handle<Node>) -> Option<Handle<Node>> {
    dep_graph.get_extension(node).copied()
}

fn identify_going_up(dep_graph: &DepGraph,
                     initial_scope: Handle<Node>,
                     identifier: String) -> Option<Handle<Node>> {
    let mut scope = initial_scope;
    loop {
        //log::info!("query: {:?} x {:?}", name(dep_graph, scope), identifier);
        for child in children(dep_graph, scope) {
            //log::info!("compare: {:?} =?= {:?}", name(dep_graph, child), identifier);
            if name(dep_graph, child) == identifier {
                return Some(child);
            }
        }
        scope = parent(dep_graph, scope)?;
        //log::info!("passaggio {:?}", name(dep_graph, scope));
    }
}

fn identify_simple(dep_graph: &DepGraph,
                   initial_scope: Handle<Node>,
                   identifier: String) -> Option<Handle<Node>> {
    for child in children(dep_graph, initial_scope) {
        if name(dep_graph, child) == identifier {
            return Some(child);
        }
    }
    None
}

fn resolve_reference(dep_graph: &DepGraph,
                     initial_scope: Handle<Node>,
                     identifiers: &mut Vec::<String>) -> Option<Handle<Node>> {
    //log::info!("chase: {:?} x {:?}", dep_graph.get_node(&initial_scope).unwrap().get_qualified_name(), identifiers);
    let identifier = identifiers.last().unwrap().clone(); 
    let mut scope = identify_going_up(dep_graph, initial_scope, identifier)?;
    identifiers.pop();
    while identifiers.len() > 0 {
        match identify_simple(dep_graph, scope, identifiers.pop()?) {
            Some(other) => {
                scope = other;
            },
            None => {
                match dep_graph.get_node(&scope).unwrap().get_defkind() {
                    Defkind::Attribute|Defkind::Parameter => {
                        match get_type(dep_graph, scope) {
                            Some(other) => {scope = other;},
                            None => ()
                        }
                    },
                    Defkind::Class|Defkind::Interface => {
                        match get_extension(dep_graph, scope) {
                            Some(other) => {scope = other;},
                            None => ()
                        }
                    },
                    _ => {
                        return None;
                    }
                }
            }
        }
    }
    Some(scope)
}

fn resolve_reference_smart(dep_graph: &DepGraph,
                           initial_scope: Handle<Node>,
                           identifiers: &Vec::<String>) -> Option<Handle<Node>> {
    let mut trial = identifiers.clone();
    loop {
        match resolve_reference(dep_graph, initial_scope, &mut trial) {
            Some(obj) => {
                return Some(obj)
            },
            None => {
                let length = trial.len();
                if length < 2 {
                    return None;
                }
                trial[length-1] = format!("{}.{}", trial[length-1], trial[length-2]).to_string();
                trial.remove(trial.len()-2);
            }
        }
    }
}

pub fn resolve_references_unit(references: Iter<'_, ReferenceQuery>, total: u64, dep_graph: &mut DepGraph) {
    let progress_bar = indicatif::ProgressBar::new(total);
    let mut found = 0;
    for reference in references {
        log::info!("##########################################################");
        let target = resolve_reference_smart(dep_graph,
                                                                   reference.get_source(),
                                                                   &reference.get_sink());
        
        if target.is_some() {
            let kind = reference.get_kind();
            let source = dep_graph.get_node(&reference.get_source()).unwrap().get_qualified_name();
            let sink = dep_graph.get_node(&target.unwrap()).unwrap().get_qualified_name();
            log::info!("{}: {} -> {}", kind, source, sink);
            if kind == Refkind::Includes {
                dep_graph.set_inclusion(reference.get_source(), target.unwrap());
            } else if kind == Refkind::UsesType {
                dep_graph.set_type(reference.get_source(), target.unwrap());
            } else if kind == Refkind::Extends || kind == Refkind::Implements {
                dep_graph.set_extension(reference.get_source(), target.unwrap());
            }
            found += 1;
        } else {
            log::info!("{:?} x {:?}", dep_graph.get_node(&reference.get_source()).unwrap().get_qualified_name(), reference.get_sink());
        }
        progress_bar.inc(1);
    }
    println!("found {}/{} references", found, total);
}

pub fn resolve_references(explorer: &SynchroExplorer, dep_graph: &mut DepGraph) {
    {
        let total = explorer.number_of_reference_includes().try_into().unwrap();
        let references = explorer.iter_reference_includes();
        resolve_references_unit(references, total, dep_graph);
    }
    {
        let total = explorer.number_of_reference_extension().try_into().unwrap();
        let references = explorer.iter_reference_extension();
        resolve_references_unit(references, total, dep_graph);
    }
    {
        let total = explorer.number_of_reference_uses_type().try_into().unwrap();
        let references = explorer.iter_reference_uses_type();
        resolve_references_unit(references, total, dep_graph);
    }
    {
        let total = explorer.number_of_reference_queries().try_into().unwrap();
        let references = explorer.iter_reference_queries();
        resolve_references_unit(references, total, dep_graph);
    }
}