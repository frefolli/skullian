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
use stack_graphs::graph::Node::PushSymbol;
use std::{collections::{HashMap}, ops::Index};
use stack_graphs::{
    graph::{StackGraph, Node, File},
    arena::Handle,
    NoCancellation, CancellationFlag, CancellationError,
    stitching::{Database, ForwardPartialPathStitcher},
    partial::{PartialPath, PartialPaths}
};
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
    let debug_infos = stack_graph.node_debug_info(node_handle)?;
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
                    Refkind::CastsType => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::CastsType
                                ));
                        }
                    },
                    Refkind::ThrowsType => {
                        let sink = explorer.get_name_binding(current_node);
                        if sink.is_some() {
                            dep_graph.add_edge(
                                DepGraphEdge::new(
                                    parent,
                                    *sink.unwrap(),
                                    EdgeLabel::ThrowsType
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

pub fn save_to_data_string(output_file: &std::path::Path, dep_graph: &DepGraph) {
    std::fs::write(
        output_file,
        dep_graph.to_string()
    ).unwrap();
}

pub fn save_to_data_json(output_file: &std::path::Path, dep_graph: &DepGraph) {
    std::fs::write(
        output_file,
        dep_graph.to_json().to_string()
    ).unwrap();
}

pub fn scrape_all_partial_paths_in_file(
    stack_graph: &StackGraph,
    database: &mut Database,
    file: Handle<File>,
    total_paths: &mut usize,
    paths: &mut PartialPaths
) {
    match paths.find_minimal_partial_path_set_in_file(
        stack_graph, file,
        &NoCancellation, |sg, ps, p| {
            database.add_partial_path(sg, ps, p);
            *total_paths += 1;
    }) {
        Err(e) => panic!("{}",e),
        _ => ()
    }
}

pub fn scrape_all_partial_paths_in_files(
    stack_graph: &StackGraph,
    database: &mut Database,
    paths: &mut PartialPaths
) {
    let mut total_paths = 0;
    log::info!("finding partial paths");
    for file in stack_graph.iter_files() {
        scrape_all_partial_paths_in_file(stack_graph,
                                         database,
                                         file, &mut total_paths, paths)
    }
    log::info!("found {} partial paths", total_paths);
}

pub fn find_all_valid_references(
    stack_graph: &StackGraph,
    references: &mut Vec::<Handle<Node>>
) {
    log::info!("finding references");
    for node_handle in stack_graph.iter_nodes() {
        match stack_graph.index(node_handle) {
            PushSymbol(_) => {
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
            },
            _ => {}
        }
    }
    log::info!("found {} references", references.len());
}

pub fn find_all_complete_partial_paths<I, F>(
    graph: &StackGraph,
    partials: &mut PartialPaths,
    db: &mut Database,
    starting_nodes: I,
    cancellation_flag: &dyn CancellationFlag,
    mut visit: F,
) -> Result<(), CancellationError>
where
    I: IntoIterator<Item = Handle<Node>>,
    F: FnMut(&StackGraph, &mut PartialPaths, &PartialPath),
{
    log::info!("INIT");
    let starting_nodes = starting_nodes
        .into_iter()
        .filter(|n| graph[*n].is_reference());
    log::info!("FILTERED");
    let mut stitcher = ForwardPartialPathStitcher::from_nodes(graph, partials, starting_nodes);
    log::info!("STITCHED");
    while !stitcher.is_complete() {
        log::info!("WHILED");
        cancellation_flag.check("finding complete partial paths")?;
        log::info!("CHECKED");
        stitcher.process_next_phase(graph, partials, db);
        log::info!("PHASED");
        for path in stitcher.previous_phase_partial_paths() {
            log::info!("FORED");
            if path.is_complete(graph) {
                log::info!("IFED");
                visit(graph, partials, path);
                log::info!("VISITED");
            }
        }
    }
    Ok(())
}

pub fn resolve_references_with_database(
    stack_graph: &StackGraph,
    database: &mut Database,
    explorer: &mut SynchroExplorer,
    paths: &mut PartialPaths
) {
    let mut references = Vec::<Handle<Node>>::new();
    find_all_valid_references(stack_graph, &mut references);
    let progress_bar = indicatif::ProgressBar::new(references.len().try_into().unwrap());
    let mut bindings = 0;
    log::info!("resolving references");
    match find_all_complete_partial_paths(
        stack_graph, paths,
        database, references,
        &NoCancellation, |_sg, _ps, path| {
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
                    }
                }
            }
        }
    }) {
        Err(e) => panic!("{}",e),
        _ => ()
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
    let mut files = 0;
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
            Defkind::AnnotationElement => annotation_elements += 1,
            Defkind::File => files += 1
        }
    }

    let total = packages + classes +
                     interfaces + functions +
                     parameters + attributes +
                     enums + enum_variants +
                     constants + annotations +
                     annotation_elements + files + others;
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
    log::info!("found {} files", files);
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
    let mut casts_type = 0;
    let mut throws_type = 0;

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
                EdgeLabel::Calls => calls += 1,
                EdgeLabel::CastsType => casts_type += 1,
                EdgeLabel::ThrowsType => throws_type += 1
            }
        }
    }

    let total = defined_by + is_implementation_of +
                     is_child_of + nested_to +
                     includes + uses_type +
                     access_field + calls +
                     casts_type + throws_type;
    log::info!("found {} definedBy", defined_by);
    log::info!("found {} isImplementationOf", is_implementation_of);
    log::info!("found {} isChildOf", is_child_of);
    log::info!("found {} nestedTo", nested_to);
    log::info!("found: {} includes", includes);
    log::info!("found: {} uses_type", uses_type);
    log::info!("found: {} access_field", access_field);
    log::info!("found: {} calls", calls);
    log::info!("found: {} casts_type", casts_type);
    log::info!("found: {} throws_type", throws_type);
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
    let mut database = Database::new();
    let mut paths = stack_graphs::partial::PartialPaths::new();
    log::info!("Explorer is_done_with init");
    scrape_all_partial_paths_in_files(stack_graph, &mut database, &mut paths);
    log::info!("Explorer is_done_with partial_paths_paths");
    resolve_references_with_database(stack_graph, &mut database, &mut explorer, &mut paths);
    log::info!("Explorer is_done_with resolving_paths");
    walk_step(&mut explorer, dep_graph, stack_graph);
    log::info!("Explorer is_done_with exploring graph");
    if output_file.as_os_str() != "" {
        save_to_data_string(output_file, dep_graph);
        log::info!("Explorer is_done_with saving_graph_to_json");
    }
    fun_facts(&dep_graph);
}
