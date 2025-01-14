#![allow(non_snake_case)]
use crate::{
    apis::users::{keep_updates, KeepUpdatesRequest},
    components::filled_button::FilledButton,
    services::popup_service::PopupService,
};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn DownloadPopup() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut popup = PopupService::use_popup_service();

    rsx! {
        div {
            class: "grid grid-rows-5 w-[370px] h-[500px] drop-shadow-[0px_0px_20px_rgba(0,0,0,0.4)] rounded-[20px] overflow-hidden z-[1000]",
            onclick: move |evt| {
                tracing::debug!("close popup");
                evt.stop_propagation();
            },
            div {
                style: format!("background-image: url('{}')",asset!(image("public/images/popup.png"))),
                class: "row-span-2 bg-[#21344C] flex justify-center items-center bg-center bg-no-repeat",
            }
            div {
                class: "row-span-3 bg-white px-[20px] py-[30px] flex flex-col justify-center items-center gap-[30px]",
                div {
                    class: "flex flex-col gap-[10px]",
                    div {
                        class: "text-[32px] text-center leading-[35px] font-bold text-[#21344C]",
                        "Select below to download!"
                    }
                    div {
                        class: "text-[18px] text-center font-regular text-[#21344C]",
                        "Leave your email to keep updated"
                    }
                }
                div {
                    class: "flex flex-col w-full gap-[10px]",
                    input {
                        class: "w-full h-[52px] bg-transparent rounded-[10px] px-[24px] py-[14px] text-[16px] leading-[24px] font-regular border-[1px] border-[#21344C] focus:outline-none focus:border-[#03AB79] transition-all duration-300 ease-in-out text-[#21344C]",
                        placeholder: "Email (optional)",
                        onchange: move |e| {
                            email.set(e.value());
                            e.stop_propagation();
                        },

                    }
                    div {
                        class: "flex flex-row justify-between items-center grid grid-cols-2 gap-[10px]",
                        FilledButton {
                            background_color: "bg-[#21344C]",
                            text_color: "text-white col-span-1",
                            onclick: move |_| {
                                spawn(async move {
                                    let email = email();
                                    if !email.is_empty() && email.contains('@') {
                                        match keep_updates(KeepUpdatesRequest { email }).await {
                                            Ok(_) => {},
                                            Err(_) => {
                                                tracing::error!("Failed to subscribe!");
                                            }
                                        };
                                    }

                                    popup.close();
                                });

                                #[cfg(feature = "web")]
                                let _ = web_sys::window().unwrap().open_with_url_and_target("https://metadata.biyard.co/decks/dagit.pdf", "_blank");
                            },
                            "BROCHURE"
                        }
                        FilledButton {
                            background_color: "bg-[#21344C]",
                            text_color: "text-white col-span-1",
                            onclick: move |_| {
                                spawn(async move {
                                    let email = email();
                                    if !email.is_empty() && email.contains('@') {
                                        match keep_updates(KeepUpdatesRequest { email }).await {
                                            Ok(_) => {},
                                            Err(_) => {
                                                tracing::error!("Failed to subscribe!");
                                            }
                                        };
                                    }

                                    popup.close();
                                });

                                #[cfg(feature = "web")]
                                let _ = web_sys::window().unwrap().open_with_url_and_target("https://metadata.biyard.co/decks/deck.pdf", "_blank");

                            },
                            "COMPANY DECK"
                        }
                    }
                }
            }
        }
    }
}
