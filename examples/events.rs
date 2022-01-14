use dioxus::prelude::*;
use fermi::*;
use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

static RESULT: Atom<f64> = |_| 0.0;

fn app(cx: Scope) -> Element {
    init_app(&cx);

    let a = use_state(&cx, || 0.0);
    let b = use_state(&cx, || 0.0);

    let res = use_read(&cx, RESULT);

    let setter = use_set(&cx, RESULT).clone();

    cx.render(rsx!(
        App {
            trigger: trigger!(
                test => move |v| {
                    setter(v.as_number().unwrap_or(0.0));
                }
            ),
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
                    let code = format!("{} + {}", &a, &b);
                    execute(&cx, "test", code.to_string());
                },
                "Calc"
            }
            p { "Result: {res}" }
        }
    ))
}
