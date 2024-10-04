#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::services::polling_service::PollingService;

use super::controller::Controller;

#[component]
pub fn NodeCanvas(onclick: Option<EventHandler<String>>) -> Element {
    let srv = PollingService::use_service();
    let mut hover: Signal<Option<usize>> = use_signal(|| None);
    let nodes = srv.get_nodes();
    let points = Controller::points_by_polygon(srv.get_nodes().len(), (500.0, 300.0), 200.0);

    rsx! {
        svg {
            width: "1000",
            height: "800",
            for (i, point) in points.into_iter().enumerate() {
                rect {
                    x: format!("{}", point.x - 50.0),
                    y: format!("{}", point.y - 15.0),
                    width: "100",
                    height: "30",
                    fill: if hover().is_some() && hover().unwrap() == i {
                        "white"
                    } else {
                        "transparent"
                    },
                    stroke_width: "1px",
                    stroke: "white",
                    onclick: move |_| {
                        if let Some(onclick) = onclick {
                            onclick(srv.get_nodes()[i].clone().id);
                        }
                    },
                    onmouseenter: move |_| {
                        tracing::debug!("node {}", i);
                        hover.set(Some(i));
                    },
                    onmouseleave: move |_| {
                        tracing::debug!("node {}", i);
                        hover.set(None);
                    },
                }
                text {
                    x: format!("{}", point.x),
                    y: format!("{}", point.y),
                    fill: if hover().is_some() && hover().unwrap() == i {
                        "#03AB79"
                    } else {
                        "white"
                    },
                    dominant_baseline: "middle",
                    text_anchor: "middle",
                    onclick: move |_| {
                        if let Some(onclick) = onclick {
                            onclick(srv.get_nodes()[i].clone().id);
                        }
                    },
                    onmouseenter: move |_| {
                        tracing::debug!("node {}", i);
                        hover.set(Some(i));
                    },
                    onmouseleave: move |_| {
                        tracing::debug!("node {}", i);
                        hover.set(None);
                    },
                    "{nodes[i].name}"

                }
            }
        }
    }
}
