/**
 * TASK
 * 
 * define a module for defining the Dependency Graph
 * 
 * The algorithm should walk through the StackGraph, resolve name bindings and 
 * 
 */
pub mod dep_graph;
pub mod dep_graph_node;
pub mod dep_graph_edge;

use stack_graphs::graph::StackGraph;
use crate::graph::dg::dep_graph::DepGraph;

pub fn todo(stack_graph: &StackGraph) {
    let dep_graph = DepGraph::from(stack_graph);
    println!("{}", dep_graph);
}