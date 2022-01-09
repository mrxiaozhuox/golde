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
                    trigger(&cx, "add", vec![
                        Value::Number(1),
                        Value::Number(2),
                    ]);
                }
            }
            script { [include_str!("./scripts/calc.js")] }
        }
    ))
}