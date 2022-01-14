#![allow(non_snake_case)]

pub mod event;
pub mod map;

use std::collections::HashMap;

use dioxus::prelude::*;
use doson::DataValue;
use fermi::{use_init_atom_root, use_read, use_set, Atom};
// use once_cell::unsync::Lazy;

pub type Value = DataValue;
pub type Trigger = HashMap<String, Box<dyn Fn(&Scope<AppProps>, DataValue) -> ()>>;

#[macro_export]
macro_rules! trigger {
    ($( $key: ident => $val: expr ),*) => {{
         let mut map: Trigger = std::collections::HashMap::new();
         $( map.insert(String::from(stringify!($key)), std::boxed::Box::new($val)); )*
         map
    }}
}

#[derive(Props)]
pub struct AppProps<'a> {
    children: Element<'a>,
    trigger: Trigger,
}

static GOLDE_EVENT_QUEUE: Atom<map::Map<String, event::Event>> = |_| map::Map::new();

pub fn init_app(cx: &Scope) {
    use_init_atom_root(cx);
}

pub fn execute(cx: &Scope, name: &str, code: String) {
    let mut golde_event_queue = use_read(&cx, GOLDE_EVENT_QUEUE).clone();
    golde_event_queue.set(
        name.to_string(),
        event::Event {
            code,
            result: DataValue::None,
        },
    );

    let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
    setter(golde_event_queue.clone());
}

pub fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {

    // check the runtime platform, now the `golde` just support WASM and Desktop
    let wasm_runtime: bool;
    if cfg!(any(target_arch = "wasm32", target_arch = "wasm64")) {
        // runtime for wasm
        wasm_runtime = true;
    } else {
        // default runtime
        wasm_runtime = false;
    }

    let golde_event_queue = use_read(&cx, GOLDE_EVENT_QUEUE);
    
    if golde_event_queue.len() > 0 {
        // here will call the callback function and return the result.
        let mut new_event_queue: map::Map<String, event::Event> = golde_event_queue.clone();
        let mut need_reload_queue: bool = false;

        for (name, data) in &golde_event_queue.inner {
            if data.result != DataValue::None {
                let callback = cx.props.trigger.get(name);
                if let Some(fun) = callback {
                    fun(&cx, data.result.clone());
                }
                need_reload_queue = true;
                new_event_queue.inner.remove(name);
            }
        }
        if need_reload_queue {
            let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
            setter(new_event_queue);
        }
    }

    let platform = format!("{}", if wasm_runtime { "WASM" } else { "Desktop" });
    log::info!("Dioxus [Golde] Runtime Platform: {}", platform);

    cx.render(rsx!(
        div {
            id: "GoldeAppStatus",
            style: "display: none;",
            "platform": "{platform}",
            form {
                id: "GoldeEventQueue",
                "value": "{golde_event_queue}",
                onsubmit: move |data| {

                    let mut queue = map::Map {
                        inner: HashMap::new(),
                    };

                    if !wasm_runtime {
                        queue = map::Map {
                            inner: serde_json::from_str::
                            <HashMap<String, event::Event>>
                            (&data.value).unwrap()
                        };
                    } else {
                        let r = WebAssemblyGetResult();
                        queue = map::Map {
                            inner: serde_json::from_str::
                            <HashMap<String, event::Event>>
                            (&r).unwrap()
                        };
                    }

                    let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
                    setter(queue);

                },
                button {
                    id: "GoldeEventQueueSubmit",
                    r#type: "submit",
                }
            }
        }
        &cx.props.children,
        script { "var platform = \"{platform}\";" }
        script { [include_str!("./script/app.js")] }
    ))
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    fn WebAssemblyGetResult() -> String;
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
fn WebAssemblyGetResult() -> String {
    return String::new();
}