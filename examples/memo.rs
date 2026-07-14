use dioxus::prelude::*;

fn main() {
    launch(Parent);
}

#[component]
fn Parent() -> Element {
    let mut x = use_signal(|| 0);
    let mut y = use_signal(|| 0);
    let _by_memo = use_memo(move || {
        info!("by_memo");
        y()
    });
    let _no_memo = move || {
        info!("no_memo");
        y()
    };
    rsx! {
        button { onclick: move |_| *x.write() += 1, "x = {x}" }
        button { onclick: move |_| *y.write() += 1, "y = {y}" }
        br {}
        // "by_memo = {by_memo()}",
        br {}
        // "no_memo = {no_memo()}"
    }
}