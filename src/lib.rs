#![allow(non_snake_case)]
use std::collections::HashMap;

use dioxus::prelude::*;
use doson::DataValue;
use fermi::{Atom, use_init_atom_root, use_read, use_set};

pub type Value = DataValue;

#[derive(Props)]
pub struct AppProps<'a> {
    children: Element<'a>,
}

pub static GOLDE_EVENT_LIST: Atom<String> = |_| String::from("{}");

pub fn init_app(cx: &Scope) { use_init_atom_root(cx); }

pub fn trigger(cx: &Scope, func: &str, argument: Vec<DataValue>) {
    let ori_list = use_read(&cx, GOLDE_EVENT_LIST);

    let mut json = serde_json::from_str(&ori_list)
        .unwrap_or(HashMap::new())
    ;

    json.insert(func, argument);

    let call = use_set(cx, GOLDE_EVENT_LIST);
    call(serde_json::to_string(&json).unwrap_or(String::from("{}")));
    
}

pub fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {

    let event_list = use_read(&cx, GOLDE_EVENT_LIST);

    cx.render(rsx!(
        
        div {
            input {
                id: "GoldeEventList",
                value: "{event_list}",
            }
            button {
                id: "GoldeEventUsed",
                onclick: move |_| {
                    let call = use_set(&cx, GOLDE_EVENT_LIST);
                    call("{}".to_string());
                },
                style: "display: none;",
            }
        }
        
        &cx.props.children,
        script { [include_str!("./app.js")] }
    ))
}