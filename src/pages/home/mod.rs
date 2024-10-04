#![allow(non_snake_case)]
use controller::Controller;
use dioxus::prelude::*;

pub mod controller;
pub mod node_canvas;

use node_canvas::NodeCanvas;

#[component]
pub fn Home() -> Element {
    let _ctrl = Controller::init();

    rsx! {
        div {
            class: "flex w-full flex-row items-start justify-start my-[100px]",
            NodeCanvas { }
        }
    }
}
