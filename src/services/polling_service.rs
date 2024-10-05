#![allow(non_snake_case)]

use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_logger::tracing;
use web_sys::wasm_bindgen::JsValue;

use crate::{
    apis::nodes::{get_polling, Blockchain},
    models::node::Node,
};

use super::node_service::NodeService;

#[derive(Debug, Clone, Copy, Default)]
pub struct PollingService {
    pub blockchain: Signal<Blockchain>,
}

impl PollingService {
    pub fn init() {
        let srv = Self::default();
        use_context_provider(|| srv);
        let id = Arc::new(NodeService::use_service().get_id());

        use_coroutine(|_: UnboundedReceiver<Self>| async move {
            web_sys::console::log_1(&JsValue::from_str("starting coroutine"));
            let mut blockchain = srv.blockchain.to_owned();

            loop {
                let id = id.clone();
                match get_polling(id.to_string()).await {
                    Ok(bc) => {
                        tracing::debug!("fetched");
                        blockchain.set(bc);
                    }
                    Err(_) => {}
                }

                let _ = wasm_bindgen_futures::JsFuture::from(sleep(1000)).await;
            }
        });
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        (self.blockchain)().nodes
    }

    pub fn use_service() -> Self {
        use_context()
    }
}

pub fn sleep(ms: i32) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}
