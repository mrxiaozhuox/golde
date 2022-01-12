#![allow(non_snake_case)]

pub mod map;
pub mod event;

use std::collections::HashMap;

use dioxus::prelude::*;
use doson::DataValue;
use fermi::{Atom, use_init_atom_root, use_read, use_set};
// use once_cell::unsync::Lazy;

pub type Value = DataValue;
pub type Trigger = HashMap<String, Box<dyn Fn(&Scope<AppProps>, DataValue) -> ()>>;


#[macro_export]
macro_rules! makec {
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

pub fn init_app(cx: &Scope) { use_init_atom_root(cx); }


pub fn execute(cx: &Scope, name: &str, code: String) {
    
    let mut golde_event_queue = use_read(&cx, GOLDE_EVENT_QUEUE).clone();
    golde_event_queue.set(name.to_string(), event::Event {
        code,
        result: DataValue::None,
    });

    let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
    setter(golde_event_queue.clone());
}

pub fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {

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

    cx.render(rsx!(
        div {
            id: "GoldeAppStatus",
            style: "display: none;",
            form {
                id: "GoldeEventQueue",
                "value": "{golde_event_queue}",
                onsubmit: move |data| {
                    let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
                    setter(map::Map {
                        inner: serde_json::from_str::
                        <HashMap<String, event::Event>>
                        (&data.value).unwrap()
                    });
                },
                button {
                    id: "GoldeEventQueueSubmit",
                    r#type: "submit",
                }
            }
        }
        &cx.props.children,
        script { [include_str!("./script/app.js")] }
    ))
}