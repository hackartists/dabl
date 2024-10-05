use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,

    // neighbors is a list of node IDs which this node communicates with.
    // And it allows only two neighbors.
    pub neighbors: Vec<String>,
    pub last_updated: u64,
}

#[cfg(feature = "server")]
impl Node {
    pub async fn save(&self) -> Result<(), crate::errors::DablException> {
        use slog::o;

        let cli = easy_dynamodb::Client::new(
            slog::Logger::root(
                slog::Discard,
                o!("service" => "dabl", "model"=>"node", "action"=>"save"),
            ),
            option_env!("AWS_ACCESS_KEY_ID")
                .expect("AWS_ACCESS_KEY_ID is required")
                .to_string(),
            option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required")
                .to_string(),
            option_env!("AWS_REGION")
                .unwrap_or("ap-northeast-2")
                .to_string(),
            option_env!("TABLE_NAME")
                .expect("TABLE_NAME is required")
                .to_string(),
            "id".to_string(),
            None,
            None,
        );

        match cli.create(self.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::errors::DablException::UnknownException(
                e.to_string(),
            )),
        }
    }
}
