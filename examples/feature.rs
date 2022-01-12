use dioxus::prelude::*;
// use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    
    let v = use_state(&cx,|| 0.0);

    println!("OnReload");

    cx.render(rsx!(
        button {
            onclick: move |_| v.set(v.get() + 1.0),
        }
        span { "Num: {v}" }
    ))
}