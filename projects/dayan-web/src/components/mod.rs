use crate::config::{use_dayan, UseDayan};
use dayan::{BMSConfig, BashicuMatrixSystem};
use dioxus::{events::FormEvent, prelude::*};
use dioxus_katex::{use_katex_display, UseKatex};
use std::{
    convert::TryFrom,
    num::{NonZeroUsize, TryFromIntError},
    str::FromStr,
};

pub fn BMSEditor(cx: Scope) -> Element {
    let dayan = use_dayan(cx);
    let mut config = BMSConfig::default();
    config.ellipsis = dayan.ellipsis();
    /// initial value
    let mut bms = BashicuMatrixSystem::new(vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 1, 0]]);
    bms.set_expand_steps(dayan.expands());
    let bms = bms.expand();
    let place_holder = r#"(0, 0, 0)(1, 1, 1)(2, 1, 0)"#;
    let current_text = use_state(&cx, || place_holder.to_string());
    let color = dayan.color_toggle();
    let ellipsis = dayan.ellipsis_toggle();
    let expand = dayan.expands_slider();
    // katex render
    let katex = use_katex_display(&cx);
    config.display = false;
    let bms_inline = config.render(&bms);
    let math_inline = katex.compile(&bms_inline);
    config.display = true;
    let bms_display = config.render(&bms);
    let math_display = katex.compile(&bms_display);
    /// <div class="flex items-center justify-center h-screen">
    //   <div class="w-full sm:w-4/5">
    //     <!-- 内容 -->
    //   </div>
    // </div>
    cx.render(rsx!(
        div {
            class: "form-control flex-1",
            textarea {
                class: "textarea h-96 textarea-bordered textarea-primary",
                id: "editor",
                placeholder: "{place_holder}",
                oninput: move |e| current_text.set(e.value.to_owned()),
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
