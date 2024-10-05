use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,

    // neighbors is a list of node IDs which this node communicates with.
    // And it allows only two neighbors.
    pub neighbors: Vec<String>,
}
