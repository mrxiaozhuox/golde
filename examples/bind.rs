use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {

    let val = use_state(&cx, || String::from("hello world"));

    cx.render(rsx!(
        input {
            value: "{val}",
            oninput: move |new| {
                val.set(new.value.to_string());
            },
        },
        span { "Value: {val}" }
    ))
}