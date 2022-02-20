use dioxus::prelude::*;
use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    init_app(&cx, |_| {});

    cx.render(rsx!(
        App {
            trigger: trigger!(
                jquery_test => |_, v| {
                    println!("{:?}", v);
                }
            ),
            button {
                onclick: move |_| {
                    call(&cx, "jquery_test", "
                        if ($('#hello').text() == 'Hello World!') {
                            $('#hello').text('Hello Dioxus!');
                        } else {
                            $('#hello').text('Hello World!');
                        }
                    ".to_string());
                },
                "Change Value From JQuery"
            }
            p {
                id: "hello",
                "Hello World!"
            }
            script { src: "https://cdn.jsdelivr.net/npm/jquery@3.6.0/dist/jquery.min.js" }
        }
    ))
}
