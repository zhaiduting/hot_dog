use dioxus::prelude::*;
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Unit1 {}
        Unit2 {}
        Unit3 {}
    }
}
#[component]
fn Unit1() -> Element {
    let mut state = use_signal(|| 0);

    rsx! {
        button {
            onclick: move |_| {
                // 使用 clone() 立即释放读保护 ✅
                let cur = state.read().clone();
                *state.write() = cur + 1; // */
            },
            "Click {state}"
        }
    }
}

#[component]
fn Unit2() -> Element {
    let mut state = use_signal(|| 0);

    rsx! {
        button {
            onclick: move |_| {
                // 使用 {...} 立即释放读保护 ✅
                let cur = { *state.read() };
                *state.write() = cur + 1;
            },
            "Click {state}"
        }
    }
}

#[component]
fn Unit3() -> Element {
    let mut state = use_signal(|| 0);

    rsx! {
        button {
            onclick: move |_| {
                // 使用 drop() 立即释放读保护 ✅
let cur = state.read();
let c = *cur;
drop(cur);
*state.write() = c + 1;
            },
            "Click {state}"
        }
    }
}
