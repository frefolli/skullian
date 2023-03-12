/**
 * TASK
 * 
 * define a module for defining the Dependency Graph
 * 
 * The algorithm should walk through the StackGraph, resolve name bindings and 
 * 
 */


pub fn walk_stack_graph(_stack_graph: &stack_graphs::graph::StackGraph) {
    let mut paths = stack_graphs::paths::Paths::new();
    paths.find_all_paths(
        _stack_graph,
        _stack_graph.iter_nodes(),
        &stack_graphs::NoCancellation,
        |sg, ps, p| {
            if p.is_complete(sg) {
                println!("{}", p.display(sg, ps));
            }
        }
    ).unwrap();
}