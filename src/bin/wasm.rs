use dioxus::prelude::*;
use golde::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    init_app(&cx);

    cx.render(rsx!{
        App {
            trigger: trigger!(),
            h1 { "hello world" }
        }
    })
}