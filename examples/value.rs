use dioxus::prelude::*;
use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    init_app(&cx);

    cx.render(rsx!(
        App {
            trigger: trigger!(
                val_dict => | _, v | {
                    println!("{:?}", v);
                },
                val_list => | _, v | {
                    println!("{:?}", v);
                }
            ),
            button {
                onclick: move | _ | {
                    execute( & cx, "val_dict", "value_type('dict')".to_string());
                },
                "Javascript::Object => Rust::Dict"
            }
            button {
                onclick: move | _ | {
                    execute( & cx, "val_list", "value_type('list')".to_string());
                },
                "Javascript::Array => Rust::List"
            }
            script { [include_str!("./demo.js")] }
        }
    ))
}
