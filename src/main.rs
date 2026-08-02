use dioxus::prelude::*;
mod backend;
mod components;
pub mod route;

use components::*;
const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Title {}
        Router::<route::Route>{}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}
