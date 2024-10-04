pub mod home;
pub mod users;

use dioxus::fullstack::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::{GetUrl, Json};

#[server(endpoint = "/version", input=GetUrl, output=Json)]
pub async fn version() -> Result<String, ServerFnError> {
    Ok(match option_env!("VERSION") {
        Some(version) => match option_env!("COMMIT") {
            Some(commit) => format!("{}-{}", version, commit),
            None => format!("{}", version),
        },
        None => match option_env!("DATE") {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        },
    }
    .to_string())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blockchain {
    pub nodes: Vec<Node>,
}

#[server(endpoint = "/nodes", input=GetUrl, output=Json)]
pub async fn get_polling() -> Result<Blockchain, ServerFnError> {
    Ok(Blockchain {
        nodes: [0..10]
            .iter()
            .map(|e| Node {
                id: format!("{}", e),
                name: format!("node {}", e),
            })
            .collect(),
    })
}
