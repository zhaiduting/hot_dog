use dioxus::prelude::*;
#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        p { "nav-page-not-found {segments:?}" }
    }
}
