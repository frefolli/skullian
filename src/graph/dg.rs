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
use std::{collections::{HashMap, VecDeque}, ops::Index};
use stack_graphs::{graph::{StackGraph, Node}, arena::Handle, NoCancellation, cycles::CycleDetector, CancellationFlag, paths::{Path, Paths}};
use dep_graph::DepGraph;

use self::{dep_graph_node::DepGraphNode, dep_graph_edge::DepGraphEdge, defkind::Defkind, refkind::Refkind, edge_label::EdgeLabel};

pub struct SynchroExplorer {
    name_bindings: HashMap<Handle<Node>, Handle<Node>>,
    visited_modes: HashMap<Handle<Node>, bool>,
    current_node: Option<Handle<Node>>,
    parent_node: Option<Handle<Node>>,
    scope_prefix: String
}

impl SynchroExplorer {
    pub fn new() -> SynchroExplorer {
        SynchroExplorer {
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
}

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
                let symbol = stack_graph.index(concrete_node.symbol().unwrap()).to_string();
                let qualified_name = format!("{}{}", scope_prefix, symbol);

                // check if definition is a justified duplicated (ex: package declaration at top level)
                let mut defnode = dep_graph.get_node_by_name(&qualified_name);
                let node_data : &DepGraphNode;
                if defnode.is_some() {
                    next_parent = defnode.copied();
                    node_data = dep_graph.get_node(&defnode.unwrap()).unwrap();
                } else {
                    dep_graph.add_node(
                        current_node,
                        DepGraphNode::new(
                            qualified_name.clone(),
                            defkind
                        ));
                    dep_graph.add_name(current_node, qualified_name);
                    node_data = dep_graph.get_node(&current_node).unwrap();
                    next_parent = Some(current_node);
                    defnode = Some(&current_node);
                }

                if current_parent.is_some() {
                    let current_defkind = node_data.get_defkind();
                    let parent_defkind = dep_graph.get_node(&current_parent.unwrap()).unwrap().get_defkind();
                    if current_defkind == parent_defkind {
                        dep_graph.add_edge(
                            DepGraphEdge::new(
                                *defnode.unwrap(),
                                current_parent.unwrap(),
                                EdgeLabel::NestedTo
                            ));
                    } else {
                        dep_graph.add_edge(
                            DepGraphEdge::new(
                                *defnode.unwrap(),
                                current_parent.unwrap(),
                                EdgeLabel::DefinedBy
                            ));
                    }
                }
            }
        } else if concrete_node.is_reference() {
            match current_parent {
                Some(parent) => {
                let refkind = Refkind::from(find_debug_info(
                    stack_graph,
                    current_node,
                    "refkind".to_string()
                ).unwrap_or_default());
                match refkind {
                    Refkind::Extends => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::IsChildOf
                                ));
                        }
                    },
                    Refkind::Implements => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::IsImplementationOf
                                ));
                        }
                    },
                    Refkind::Includes => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::Includes
                                ));
                        }
                    },
                    Refkind::UsesType => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::UsesType
                                ));
                        }
                    },
                    Refkind::AccessField => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::AccessField
                                ));
                        }
                    },
                    Refkind::Calls => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::Calls
                                ));
                        }
                    },
                    Refkind::Nothing => ()
                }},
                None => ()
            };
        }

        // PREPARE PHASE
        if concrete_node.is_definition() {
            let symbol = stack_graph.index(concrete_node.symbol().unwrap()).to_string();
            explorer.set_scope_prefix(format!("{}{}.", scope_prefix, symbol));
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

pub fn save_to_data_json(output_file: &std::path::Path, dep_graph: &DepGraph) {
    std::fs::write(
        output_file,
        dep_graph.to_json().to_string()
    ).unwrap();
}

pub fn resolve_all_paths(
    explorer: &mut SynchroExplorer,
    stack_graph: &StackGraph
) {
    let mut paths = stack_graphs::paths::Paths::new();
    paths.find_all_paths(
        stack_graph,
        stack_graph.iter_nodes(),
        &NoCancellation,
        |sg,_ps,p| {
            if p.is_complete(sg) {
                explorer.set_name_binding(p.start_node, p.end_node);
            }
        }
    ).unwrap();
}

pub fn resolve_all_paths_only_of_references(
    explorer: &mut SynchroExplorer,
    stack_graph: &StackGraph
) {
    let mut paths = stack_graphs::paths::Paths::new();
    let mut references = Vec::<Handle<Node>>::new();
    log::info!("finding references");
    for node_handle in stack_graph.iter_nodes() {
        if stack_graph.index(node_handle).is_reference() {
            let refkind = Refkind::from(find_debug_info(
                stack_graph,
                node_handle,
                "refkind".to_string()
            ).unwrap_or_default());
            match refkind.is_nothing() {
                false => {
                    references.push(node_handle);
                },
                true => (),
            }
        }
    }
    log::info!("found {} references", references.len());
    let mut bindings = 0;
    let progress_bar = indicatif::ProgressBar::new(references.len().try_into().unwrap());
    paths.find_all_paths(
        stack_graph,
        references.into_iter(),
        &NoCancellation,
        |sg,_ps,p| {
            if p.is_complete(sg) {
                match Defkind::from(
                    find_debug_info(
                        stack_graph,
                        p.end_node,
                        "defkind".to_string()
                    ).unwrap_or_default()
                ).is_nothing() {
                    true => {},
                    false => {
                        if explorer.name_bindings.get(&p.start_node).is_none() {
                            bindings += 1;
                            progress_bar.inc(1);
                            explorer.set_name_binding(p.start_node, p.end_node);
                        }
                    }
                }
            }
        }
    ).unwrap();
    progress_bar.finish();
    log::info!("found {} bindings", bindings);
}

pub fn resolve_all_paths_manual_extension(
    explorer: &mut SynchroExplorer,
    stack_graph: &StackGraph
) {
    let mut references = Vec::<Handle<Node>>::new();
    log::info!("finding references");
    for node_handle in stack_graph.iter_nodes() {
        if stack_graph.index(node_handle).is_reference() {
            let refkind = Refkind::from(find_debug_info(
                stack_graph,
                node_handle,
                "refkind".to_string()
            ).unwrap_or_default());
            match refkind.is_nothing() {
                false => {
                    references.push(node_handle);
                },
                true => (),
            }
        }
    }
    log::info!("found {} references", references.len());
    
    let mut bindings = 0;
    let progress_bar = indicatif::ProgressBar::new(references.len().try_into().unwrap());
    for node_handle in references {
        let mut paths = Paths::new();
        let mut cycle_detector = CycleDetector::new();
        let mut queue = [node_handle].iter()
            .into_iter()
            .filter_map(|node| Path::from_node(stack_graph, &mut paths, *node))
            .collect::<VecDeque<_>>();
        while let Some(path) = queue.pop_front() {
            NoCancellation.check("finding paths").unwrap();
            if !cycle_detector.should_process_path(&path, |probe| probe.cmp(stack_graph, &mut paths, &path)) {
               continue;
            }
            path.extend(stack_graph, &mut paths, &mut queue);
            if path.is_complete(stack_graph) {
                match Defkind::from(
                    find_debug_info(
                        stack_graph,
                        path.end_node,
                        "defkind".to_string()
                    ).unwrap_or_default()
                ).is_nothing() {
                    true => {},
                    false => {
                        if explorer.name_bindings.get(&path.start_node).is_none() {
                            bindings += 1;
                            progress_bar.inc(1);
                            explorer.set_name_binding(path.start_node, path.end_node);
                            break;
                        }
                    }
                }
            }
        }
    }
    progress_bar.finish();
    log::info!("found {} bindings", bindings);
}

fn fun_facts_about_nodes(dep_graph: &DepGraph) {
    let mut packages = 0;
    let mut classes = 0;
    let mut interfaces = 0;
    let mut functions = 0;
    let mut parameters = 0;
    let mut attributes = 0;
    let mut enums = 0;
    let mut enum_variants = 0;
    let mut constants = 0;
    let mut annotations = 0;
    let mut annotation_elements = 0;
    let mut others = 0;

    for (_node, _data) in dep_graph.iter_nodes() {
        match _data.get_defkind() {
            Defkind::Package => packages += 1,
            Defkind::Class => classes += 1,
            Defkind::Interface => interfaces += 1,
            Defkind::Function => functions += 1,
            Defkind::Parameter => parameters += 1,
            Defkind::Attribute => attributes += 1,
            Defkind::Enum => enums += 1,
            Defkind::EnumVariant => enum_variants += 1,
            Defkind::Nothing => others += 1,
            Defkind::Constant => constants += 1,
            Defkind::Annotation => annotations += 1,
            Defkind::AnnotationElement => annotation_elements += 1
        }
    }

    let total = packages + classes +
                     interfaces + functions +
                     parameters + attributes +
                     enums + enum_variants +
                     constants + annotations +
                     annotation_elements + others;
    log::info!("found {} packages", packages);
    log::info!("found {} classes", classes);
    log::info!("found {} interfaces", interfaces);
    log::info!("found {} functions", functions);
    log::info!("found {} parameters", parameters);
    log::info!("found {} attributes", attributes);
    log::info!("found {} enums", enums);
    log::info!("found {} enum_variants", enum_variants);
    log::info!("found {} constants", constants);
    log::info!("found {} annotations", annotations);
    log::info!("found {} annotation_elements", annotation_elements);
    log::info!("found {} other nodes", others);
    log::info!("total: {} nodes", total);
}

pub fn fun_facts_about_edges(dep_graph: &DepGraph) {
    let mut defined_by = 0;
    let mut is_implementation_of = 0;
    let mut is_child_of = 0;
    let mut nested_to = 0;
    let mut includes = 0;
    let mut uses_type = 0;
    let mut access_field = 0;
    let mut calls = 0;

    for (_node, _edges) in dep_graph.iter_edges() {
        for edge in _edges.iter() {
            match edge.get_label() {
                EdgeLabel::DefinedBy => defined_by += 1,
                EdgeLabel::IsImplementationOf => is_implementation_of += 1,
                EdgeLabel::IsChildOf => is_child_of += 1,
                EdgeLabel::NestedTo => nested_to += 1,
                EdgeLabel::Includes => includes += 1,
                EdgeLabel::UsesType => uses_type += 1,
                EdgeLabel::AccessField => access_field += 1,
                EdgeLabel::Calls => calls += 1
            }
        }
    }

    let total = defined_by + is_implementation_of +
                     is_child_of + nested_to +
                     includes + uses_type +
                     access_field + calls;
    log::info!("found {} definedBy", defined_by);
    log::info!("found {} isImplementationOf", is_implementation_of);
    log::info!("found {} isChildOf", is_child_of);
    log::info!("found {} nestedTo", nested_to);
    log::info!("found: {} includes", includes);
    log::info!("found: {} uses_type", uses_type);
    log::info!("found: {} access_field", access_field);
    log::info!("found: {} calls", calls);
    log::info!("total: {} edges", total);
}

fn fun_facts(dep_graph: &DepGraph) {
    fun_facts_about_nodes(dep_graph);
    fun_facts_about_edges(dep_graph);
}

pub fn build_dep_graph(
    dep_graph: &mut DepGraph,
    output_file: &std::path::Path,
    stack_graph: &StackGraph
) {
    let mut explorer = SynchroExplorer::new();
    explorer.set_current_node(Some(stack_graphs::graph::StackGraph::root_node()));
    // resolve_all_paths_only_of_references(&mut explorer, stack_graph);
    resolve_all_paths_manual_extension(&mut explorer, stack_graph);
    log::info!("Explorer is_done_with resolving_paths");
    walk_step(&mut explorer, dep_graph, stack_graph);
    log::info!("Explorer is_done_with exploring graph");
    save_to_data_json(output_file, dep_graph);
    log::info!("Explorer is_done_with saving_graph_to_json");
    fun_facts(&dep_graph);
    log::info!("{}", dep_graph);
}
