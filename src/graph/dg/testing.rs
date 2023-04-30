use std::fmt::Display;

use serde::{Serialize, Deserialize};

use super::{defkind::Defkind, edge_label::EdgeLabel, dep_graph::DepGraph};

#[derive(Debug, Clone)]
pub struct TestError {
    msg: String
}

impl TestError {
    fn new(msg: String) -> TestError {
        TestError {
            msg: msg
        }
    }
}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
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

impl Display for NodeTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "| {} | {} |",
            self.name,
            self.kind
        )
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
                    Some(_sink_handle) => {
                        for edge in dep_graph.get_edges(node_handle).unwrap() {
                            let actual_sink = dep_graph.get_node(&edge.get_sink());
                            if actual_sink.unwrap().get_qualified_name() == self.sink {
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

impl Display for EdgeTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "| {} | {} | {} |",
            self.source,
            self.sink,
            self.kind
        )
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
    ) -> Result<String, String> {
        let mut ok: bool = true;
        let mut report: String = String::from("");
        report += "| node | kind | detected |\n| --- | --- | --- |";
        for node in self.nodes.iter() {
            match node.verify(dep_graph) {
                Ok(_) => {
                    report += format!("\n{} OK |", node).as_str();
                },
                Err(_) => {
                    ok = false;
                    report += format!("\n{} NO |", node).as_str();
                }
            }
        }
        report += "\n\n| source | sink | kind | detected |\n| --- | --- | --- | --- |";
        for edge in self.edges.iter() {
            match edge.verify(dep_graph) {
                Ok(_) => {
                    report += format!("\n{} OK |", edge).as_str();
                },
                Err(_) => {
                    ok = false;
                    report += format!("\n{} NO |", edge).as_str();
                }
            }
        }

        if ok {
            return Ok(report);
        } else {
            return Err(report);
        }
    }
}
