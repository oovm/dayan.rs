use crate::config::use_dayan;
use dayan::BMSConfig;
use dioxus::prelude::*;
use dioxus_katex::use_katex_display;

pub fn BMSEditor(cx: Scope) -> Element {
    let dayan = use_dayan(cx);
    let mut config = BMSConfig::default();
    config.ellipsis = dayan.ellipsis();
    // initial value
    let place_holder = r#"(0, 0, 0)(1, 1, 1)(2, 1, 0)"#;
    let current_text = use_state(&cx, || place_holder.to_string());
    let mut bms = dayan.get_bms();
    bms.set_expand_steps(dayan.expands());
    let bms = bms.expand();
    let color = dayan.color_toggle();
    let ellipsis = dayan.ellipsis_toggle();
    let expand = dayan.expands_slider();
    let update_bms = dayan.on_bms_input();
    // katex render
    let katex = use_katex_display(&cx);
    config.display = false;
    let bms_inline = config.render(&bms);
    let math_inline = katex.compile(&bms_inline);
    config.display = true;
    let bms_display = config.render(&bms);
    let math_display = katex.compile(&bms_display);
    cx.render(rsx!(
        div {
            class: "form-control flex-1",
            textarea {
                class: "textarea h-96 textarea-bordered textarea-primary",
                id: "editor",
                placeholder: "{place_holder}",
                oninput: move |e| {
                    current_text.set(e.value.to_owned());
                    update_bms(e);
                },
                value: "{current_text}",
            }
        }
        div {
            class: "flex-1 ml-2 mr-2",
            h3 {
                "Inline Math:"
            }
            pre {
                class: "text-sm",
                "{bms_inline}"
            }
            math_inline
        }
        div {
            class: "flex-1 ml-2 mr-2",
            h3 {
                "Display Math:"
            }
            pre {
                class: "text-sm",
                "{bms_display}"
            }
            math_display
        }
        div {
            class: "form-control",
            color
        }
        div {
            class: "form-control",
            ellipsis
        }
        div {
            class: "form-control",
            expand
        }
    ))
}
