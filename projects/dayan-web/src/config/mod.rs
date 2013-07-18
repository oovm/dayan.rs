use dayan::BashicuMatrixSystem;
use dioxus::{core_macro::rsx, events::FormEvent, prelude::*};
use std::{cell::RefCell, rc::Rc, str::FromStr, sync::Arc};

#[derive(Clone)]
pub struct UseDayan {
    dayan: Rc<RefCell<DayanOptions>>,
    updater: Arc<dyn Fn() + 'static>,
}

pub struct DayanOptions {
    color: bool,
    ellipsis: bool,
    expands: usize,
    bms: BashicuMatrixSystem,
}

pub fn use_dayan(cx: &ScopeState) -> &mut UseDayan {
    let options = DayanOptions {
        color: false,
        ellipsis: false,
        expands: 2,
        bms: BashicuMatrixSystem::new(vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 1, 0]]).expect("BMS"),
    };
    let katex = UseDayan { dayan: Rc::new(RefCell::new(options)), updater: cx.schedule_update() };
    cx.use_hook(|| katex)
}

impl UseDayan {
    pub fn get_bms(&self) -> BashicuMatrixSystem {
        self.dayan.borrow().bms.clone()
    }
    pub fn on_bms_input(&self) -> impl Fn(FormEvent) {
        let copy = self.clone();
        move |e| {
            let mut v = copy.dayan.borrow_mut();
            match BashicuMatrixSystem::from_str(e.value.as_str()) {
                Ok(o) => {
                    v.bms = o;
                    copy.needs_update();
                }
                Err(_) => {
                    // log::error!("{:?}", e)
                }
            };
        }
    }
}

impl UseDayan {
    /// Get the current value of the ellipsis option.
    pub fn ellipsis(&self) -> bool {
        self.dayan.borrow().ellipsis
    }
    /// Add a toggle button for the ellipsis option.
    pub fn ellipsis_toggle(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            let mut v = self.dayan.borrow_mut();
            match e.value.as_str() {
                "true" => v.ellipsis = true,
                "false" => v.ellipsis = false,
                _ => {}
            }
            self.needs_update();
        };
        let v = self.dayan.borrow().ellipsis;
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Ellipsis"
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
    /// Get the current value of the color option.
    pub fn color_toggle(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            let mut v = self.dayan.borrow_mut();
            match e.value.as_str() {
                "true" => v.color = true,
                "false" => v.color = false,
                _ => {}
            }
            self.needs_update();
        };
        let v = self.dayan.borrow().color;
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Colored"
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
    /// Get the current value of the color option.
    pub fn expands(&self) -> usize {
        self.dayan.borrow().expands
    }
    /// Add a toggle button for the ellipsis option.
    pub fn expands_slider(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            match usize::from_str(e.value.as_str()) {
                Ok(v) => {
                    self.dayan.borrow_mut().expands = v;
                }
                Err(e) => {
                    log::warn!("range {:?}", e);
                }
            }
            self.needs_update();
        };
        let v = self.dayan.borrow().expands;
        rsx!(
            label {
                class: "label flex",
                span {
                    class: "mr-auto label-text",
                    "Expands"
                }
                input {
                    class: "range range-sm",
                    style: "width: 200px;",
                    r#type: "range",
                    min: "1",
                    max: "10",
                    step: "1",
                    value: "{v}",
                    onchange: click,
                }
            }
        )
    }
    /// Force a re-render of the component.
    pub fn needs_update(&self) {
        (self.updater)()
    }
}
