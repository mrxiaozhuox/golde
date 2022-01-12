#![allow(non_snake_case)]

pub mod map;
pub mod event;

use std::collections::HashMap;

use dioxus::prelude::*;
use doson::DataValue;
use fermi::{Atom, use_init_atom_root, use_read, use_set};
// use once_cell::unsync::Lazy;

pub type Value = DataValue;
pub type Collector = HashMap<String, Box<dyn Fn(DataValue) -> ()>>;

#[derive(Props)]
pub struct AppProps<'a> {
    children: Element<'a>,
    collector: Collector,
}

static GOLDE_EVENT_QUEUE: Atom<map::Map<String, event::Event>> = |_| map::Map::new();

pub fn init_app(cx: &Scope) { use_init_atom_root(cx); }


pub fn call(cx: &Scope, name: &str, code: String) {
    
    let mut golde_event_queue = use_read(&cx, GOLDE_EVENT_QUEUE).clone();

    golde_event_queue.set(name.to_string(), event::Event {
        code,
        result: DataValue::None,
    });

    let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
    setter(golde_event_queue);
}

pub fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {

    // this var will 
    let golde_event_queue = use_read(&cx, GOLDE_EVENT_QUEUE);

    if golde_event_queue.len() > 0 {
        let mut new_event_queue: map::Map<String, event::Event> = map::Map::new();

        for (name, data) in &golde_event_queue.inner {
            if data.result != DataValue::None {
                let callback = cx.props.collector.get(name);
                if let Some(fun) = callback {
                    fun(data.result.clone());
                }
                new_event_queue.set(name.clone(), event::Event { code: data.code.clone(), result: DataValue::None });
            }
        }

        let setter = use_set(&cx, GOLDE_EVENT_QUEUE);
        setter(new_event_queue);
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
                    // println!("{:?}", use_read(&cx,GOLDE_EVENT_QUEUE));
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