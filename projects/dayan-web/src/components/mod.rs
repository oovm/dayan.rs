use dioxus::{events::FormEvent, prelude::*};

use dayan::{BMSConfig, BashicuMatrixSystem};
use dioxus_katex::{use_katex_display, UseKatex};
use log::log;

pub fn Editor(cx: Scope) -> Element {
    let mut config = BMSConfig::default();
    config.display = true;
    let bms = BashicuMatrixSystem::new(vec![vec![0, 0], vec![1, 1], vec![2, 1]]).expand();

    let place_holder = r#"K_{0}-K_{1}=\frac{E}{c^2}\frac{v^2}{2}"#;
    let text = use_state(&cx, || place_holder.to_string());
    let katex = use_katex_display(&cx);
    let is_display = DisplayToggle(katex, "Display Mode");
    let is_display2 = DisplayToggle(katex, "Ellipsis");
    let math = katex.compile(text);
    // <select class="select select-info w-full max-w-xs">
    //   <option disabled selected>Select language</option>
    //   <option>English</option>
    //   <option>Japanese</option>
    //   <option>Italian</option>
    // </select>
    cx.render(rsx!(
        select {
            class: "select select-primary w-full max-w-xs",
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
            href: "https://github.com/oovm/dayan.rs/issues",
            target: "_blank",
            button {
                class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                r#type: "button",
                "Report bug on github"
            }
        }
        div {
            class: "flex flex-column",
            div {
                class: "form-control flex-1",
                textarea {
                    class: "textarea h-96 textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "{place_holder}",
                    oninput: move |e| text.set(e.value.to_owned()),
                    value: "{text}",
                }
            }
            div {
                class: "flex-1 ml-2 mr-2",
                math
            }
        }
        div {
            class: "form-control",
            is_display
        }
        div {
            class: "form-control",
            is_display2
        }
        div {
            class: "form-control",
            "a"
        }
    ))
}

fn DisplayToggle<'a>(ctx: &'a UseKatex, text: &'static str) -> LazyNodes<'a, 'a> {
    let v = ctx.get_config().display_mode;
    let click = move |e: FormEvent| match e.value.as_str() {
        "true" => ctx.set_display_mode(),
        "false" => ctx.set_inline_mode(),
        _ => {}
    };
    rsx!(
        label {
            class: "cursor-pointer label",
            span {
                class: "label-text",
                "{text}"
            }
            input {
                r#type: "checkbox",
                class: "toggle",
                checked: "{v}",
                oninput: click
            }
        }
    )
}
// fn ModeSelect(vtx: &UseKatex) -> LazyNodes {
//     let v = vtx.get_mode();
//     rsx!(
//         label {
//             class: "cursor-pointer label",
//             span {
//                 class: "label-text",
//                 "Compile Mode"
//             }
//             select {
//                 class: "select select-primary w-full max-w-xs",
//                 value: "{v}",
//                 onchange: move |e| vtx.set_mode(e),
//                 option {
//                     value: "m",
//                     "Normal"
//                 }
//                 option {
//                     value: "i",
//                     "Inline"
//                 }
//                 option {
//                     value: "s",
//                     "Scoped"
//                 }
//                 option {
//                     value: "k",
//                     "DataKey"
//                 }
//                 option {
//                     value: "v",
//                     "DataValue"
//                 }
//             }
//         }
//     )
// }
