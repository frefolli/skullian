use serde::{Serialize, Deserialize};

use super::{defkind::Defkind, edge_label::EdgeLabel, dep_graph::DepGraph};

#[derive(Debug, Clone)]
pub struct TestError {
    msg: String
}

impl TestError {
    fn new(msg: String) -> TestError {
        TestError {
            msg
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NodeTest {
    pub name: String,
    pub kind: Defkind
}

impl NodeTest {
    pub fn new(
        name: String,
        kind: Defkind
    ) -> NodeTest {
        NodeTest {
            name,
            kind,
        }
    }

    pub fn verify(
        &self,
        dep_graph: &DepGraph
    ) -> Result<(), TestError> {
        match dep_graph.get_node_by_name(&self.name) {
            Some(node_handle) => {
                let data = dep_graph.get_node(node_handle).unwrap();
                if data.get_defkind() == self.kind {
                    Ok(())
                } else {
                    Err(
                        TestError::new(
                            format!(
                                "node {} should have kind {}", self.name, self.kind)))
                }
            },
            None => Err(
                TestError::new(
                    format!(
                        "node {} should exist", self.name)))
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EdgeTest {
    pub source: String,
    pub sink: String,
    pub kind: EdgeLabel
}

impl EdgeTest {
    pub fn new(
        source: String,
        sink: String,
        kind: EdgeLabel
    ) -> EdgeTest {
        EdgeTest {
            source,
            sink,
            kind,
        }
    }

    pub fn verify(
        &self,
        dep_graph: &DepGraph
    ) -> Result<(), TestError> {
        match dep_graph.get_node_by_name(&self.source) {
            Some(node_handle) => {
                match dep_graph.get_node_by_name(&self.sink) {
                    Some(sink_handle) => {
                        for edge in dep_graph.get_edges(node_handle).unwrap() {
                            let data = dep_graph.get_node(sink_handle).unwrap();
                            if data.get_qualified_name() == self.sink {
                                if edge.get_label() == self.kind {
                                    return Ok(());
                                }
                            }
                        }
                        Err(
                            TestError::new(
                                format!(
                                    "unable to find an edge of label {} from {} to {}",
                                    self.kind, self.source, self.sink
                                )))
                    },
                    None => Err(
                        TestError::new(
                            format!(
                                "node {} should exist", self.sink)))
                }
            }
            None => Err(
                TestError::new(
                    format!(
                        "node {} should exist", self.source)))
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TestCase {
    pub filepaths: Vec<Box<std::path::Path>>,
    pub nodes: Vec<NodeTest>,
    pub edges: Vec<EdgeTest>
}

impl TestCase {
    pub fn new(
        filepaths: Vec<Box<std::path::Path>>,
        nodes: Vec<NodeTest>,
        edges: Vec<EdgeTest>
    ) -> TestCase {
        TestCase {
            filepaths,
            nodes,
            edges,
        }
    }

    pub fn verify(
        &self,
        dep_graph: &DepGraph
    ) -> Result<(), TestError> {
        for node in self.nodes.iter() {
            node.verify(dep_graph)?;
        }
        for edge in self.edges.iter() {
            edge.verify(dep_graph)?;
        }

        Ok(())
    }
}