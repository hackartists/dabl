#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct NodeService {
    pub id: Signal<String>,
    pub name: Signal<String>,
}

impl NodeService {
    pub fn init() {
        #[cfg(feature = "web")]
        {
            use dioxus_sdk::storage::*;

            let id = use_synced_storage::<LocalStorage, String>("node_id".to_string(), || {
                uuid::Uuid::new_v4().to_string()
            });
            let name = use_signal(|| "Node".to_string());

            use_context_provider(|| Self { name, id });
        }

        #[cfg(not(feature = "web"))]
        use_context_provider(|| Self::default());
    }

    #[allow(dead_code)]
    pub fn use_service() -> Self {
        use_context()
    }
}
