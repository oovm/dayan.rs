#![feature(arbitrary_self_types)]
#![allow(non_snake_case)]
#![allow(clippy::mut_from_ref)]
#![allow(clippy::derivable_impls)]

use dioxus::prelude::*;
mod components;
mod config;

pub use self::components::*;
use crate::config::use_dayan;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(AppWeb)
}

// pub fn main_ssr() {
//     let mut vdom = VirtualDom::new(AppWeb);
//     let _ = vdom.rebuild();
//     println!("{}", dioxus::ssr::render_vdom(&vdom));
// }

pub fn AppWeb(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex items-center justify-center",
            div {
                class: "w-full sm:w-4/5 flex flex-col justify-center",
                Header {}
                hr {
                    class: "h-2 bg-transparent border-0"
                }
                BMSEditor {}
            }
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex items-center",
            select {
                class: "select select-primary mr-auto",
                value: "bms40",
                onchange: move |e| log::info!("mode: {}", e.value),
                option {
                    disabled: "true",
                    selected: "true",
                    "Select Notation"
                }
                option {
                    value: "bms40",
                    "BMS 4.0"
                }
                option {
                    value: "y",
                    "Y Sequence"
                }
            }
            a {
                class: "ml-auto",
                href: "https://github.com/oovm/dayan.rs/issues",
                target: "_blank",
                button {
                    class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                    r#type: "button",
                    "Report bug on github"
                }
            }
        }
    })
}
