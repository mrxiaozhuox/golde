use std::collections::HashMap;

use dioxus::prelude::*;
use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {

    // let mut app = GoldeApp::init_app(&cx);

    init_app(&cx);

    let a = use_state(&cx, || 0.0);
    let b = use_state(&cx, || 0.0);

    let res = use_state(&cx, || 0.0);

    let mut collector: Collector = HashMap::new();
    collector.insert("golde@init".into(), Box::new(|v| {
        println!("{:?}",v);
    }));

    cx.render(rsx!(
        App {
            collector: collector,
            input {
                value: "{a}",
                onchange: move |data| a.set(
                    data.value.parse::<f64>().unwrap_or(0.0)
                )
            }
            input {
                value: "{b}",
                onchange: move |data| b.set(
                    data.value.parse::<f64>().unwrap_or(0.0)
                )
            }
            button {  
                onclick: move |_| {
                    call(&cx, "test", "1 + 1".to_string());
                },
                "计算结果"
            }
            span { "结果为: {res}" }
            script { [include_str!("./demo.js")] }
        }
    ))
}