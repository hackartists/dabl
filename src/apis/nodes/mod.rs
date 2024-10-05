#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use serde::{Deserialize, Serialize};

use crate::models::node::Node;

#[server(endpoint = "/nodes/:id", input = GetUrl, output = Json)]
pub async fn get_my_node(id: String) -> Result<Node, ServerFnError> {
    tracing::debug!("/nodes/:id: {:?}", id);

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}
