#![allow(non_snake_case)]

pub mod event;
pub mod map;

use std::collections::HashMap;

use dioxus::prelude::*;
use doson::DataValue;
use fermi::{use_init_atom_root, use_read, use_set, Atom};
// use once_cell::unsync::Lazy;

pub type Value = DataValue;
pub type Trigger = HashMap<String, Box<dyn Fn(String, DataValue) -> ()>>;

#[macro_export]
macro_rules! trigger {
    ($( $key: ident => $val: expr ),*) => {{
         let mut map: Trigger = std::collections::HashMap::new();
         $( map.insert(String::from(stringify!($key)), std::boxed::Box::new($val)); )*
         map
    }}
}

pub fn use_once(cx: &ScopeState, f: impl FnOnce()) {
    let init = cx.use_hook(|_| true);
    if *init {
        f();
        *init = false;
    }
}

#[derive(Props)]
pub struct AppProps<'a> {
    
    children: Element<'a>,
    trigger: Trigger,

}

static GOLDE_EVENT_QUEUE: Atom<map::Map<String, event::Event>> = |_| map::Map::new();

pub fn init_app(cx: &Scope, f: impl FnOnce()) {
    use_init_atom_root(cx);
    let init = cx.use_hook(|_| true);
    if *init {
        f();
        *init = false;
    }
}

pub fn execute(cx: &ScopeState, name: &str, code: String) {
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

pub fn just_call(cx: &ScopeState, code: String) {
    let mut golde_event_queue = use_read(&cx, GOLDE_EVENT_QUEUE).clone();
    golde_event_queue.set(
        "_JUST_CALL_".to_string(),
        event::Event {
            code,
            result: DataValue::String("<Just-Call>".to_string()),
        },
    );
    let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
    setter(golde_event_queue.clone());
}

pub fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {

    // check the runtime platform, now the `golde` just support WASM and Desktop
    let wasm_runtime = cfg!(any(target_arch = "wasm32", target_arch = "wasm64"));

    let platform = format!("{}", if wasm_runtime { "WASM" } else { "Desktop" });

    let initialized = use_state(&cx, || false);
    if !*initialized.get() {
        log::info!("Dioxus [Golde] Runtime Platform: {}", platform);
        initialized.set(true);
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
                    fun(data.code.clone(), data.result.clone());
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


    cx.render(rsx!(
        div {
            id: "GoldeAppStatus",
            style: "display: none;",
            "platform": "{platform}",
            form {
                id: "GoldeEventQueue",
                "value": "{golde_event_queue}",
                onsubmit: move |data| {

                    let queue;
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
                        log::info!("NEW_: {}", queue);
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