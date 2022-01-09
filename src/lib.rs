#![allow(non_snake_case)]
use std::collections::HashMap;

use dioxus::prelude::*;
use doson::DataValue;
use fermi::{Atom, use_init_atom_root, use_read, use_set};
use once_cell::unsync::Lazy;

pub type Value = DataValue;

#[derive(Props)]
pub struct AppProps<'a> {
    children: Element<'a>,
}

static GOLDE_EVENT_LIST: Atom<String> = |_| String::from("{}");
static CURRENT_ID: Atom<String> = |_| rand_id_dict();
static mut GOLDE_EVENT_RESULTS: Lazy<HashMap<String, DataValue>> = Lazy::new(|| HashMap::new());

pub fn init_app(cx: &Scope) { use_init_atom_root(cx); }

fn rand_id_dict() -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
    .map(|()| rand::Rng::sample(&mut rng, rand::distributions::Alphanumeric))
    .map(char::from)
    .take(15)
    .collect()
}

pub fn trigger(cx: &Scope, func: &str, argument: Vec<DataValue>) -> DataValue {

    let ori_list = use_read(&cx, GOLDE_EVENT_LIST);

    let ori_id = use_read(&cx, CURRENT_ID).clone();

    let mut json = serde_json::from_str(&ori_list)
        .unwrap_or(HashMap::new())
    ;

    json.insert(func, argument);

    let call = use_set(cx, GOLDE_EVENT_LIST);
    call(serde_json::to_string(&json).unwrap_or(String::from("{}")));

    std::thread::sleep(core::time::Duration::from_millis(500));

    unsafe {
        let res = &GOLDE_EVENT_RESULTS;
        return res.get(&ori_id).unwrap_or(&DataValue::None).clone();
    }
}

pub fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {

    let event_list = use_read(&cx, GOLDE_EVENT_LIST);

    let event_results = format!("{{\"_id\":\"{}\"}}", use_read(&cx, CURRENT_ID));

    cx.render(rsx!(
        
        div {

            input {
                id: "GoldeEventList",
                value: "{event_list}",
                style: "display: none;"
            }
            button {
                id: "GoldeEventUsed",
                onclick: move |_| {
                    let call = use_set(&cx, GOLDE_EVENT_LIST);
                    call("{}".to_string());
                },
                style: "display: none;",
            }

            form {
                id: "GoldeEventResultForm",
                style: "display: none;",
                onsubmit: move |data| {
                    
                    println!("DATA: {:?}", data);

                   let v = doson::DataValue::from(&data.value);
                   let temp_dict = v.as_dict().unwrap_or(HashMap::new());

                    unsafe {
                        GOLDE_EVENT_RESULTS.insert(
                            temp_dict.get("_id").unwrap().as_string().unwrap(),
                            v.clone()
                        );
                    }
            
                    let call = use_set(&cx, CURRENT_ID);
                    call(rand_id_dict());
                    
                },
                "value": "{event_results}",
                button {
                    id: "GoldeEventResultSubmit",
                    r#type: "submit",
                }  
            }

        }
        
        &cx.props.children,
        script { [include_str!("./app.js")] }
    ))
}