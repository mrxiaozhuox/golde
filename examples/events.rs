use dioxus::prelude::*;
use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {

    init_app(&cx);

    cx.render(rsx!(
        App {
            button {
                onclick: move |_| {
                    let res = trigger(&cx, "add", vec![
                        Value::Number(1.0),
                        Value::Number(2.0),
                    ]);
                    println!("{:?}", res);
                }
            }
            script { [include_str!("./demo.js")] }
        }
    ))
}