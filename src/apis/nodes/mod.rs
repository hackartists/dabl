#![allow(unused_imports)]

use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::models::node::Node;

#[server(endpoint = "/nodes/:id", input = GetUrl, output = Json)]
pub async fn get_my_node(id: String) -> Result<Node, ServerFnError> {
    tracing::debug!("/nodes/:id: {:?}", id);

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}

#[server(endpoint = "/nodes/:id", input = PostUrl, output = Json)]
pub async fn set_node_info(req: Node) -> Result<(), ServerFnError> {
    tracing::debug!("/nodes/:id: {:?}", req);

    req.save().await?;

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blockchain {
    pub nodes: Vec<Node>,
}

#[server(endpoint = "/nodes", input=GetUrl, output=Json)]
pub async fn get_polling(id: String) -> Result<Blockchain, ServerFnError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // TODO: implement list nodes from dynamodb.

    // TODO: update last_updated of node by id.

    Ok(Blockchain {
        nodes: (0..10)
            .into_iter()
            .map(|e| Node {
                id: format!("{:?}", e),
                name: format!("node {:?}", e),
                neighbors: vec![],
                last_updated: timestamp,
            })
            .collect(),
    })
}
