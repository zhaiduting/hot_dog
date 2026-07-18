use dioxus::prelude::*;
const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Title {}
        DogView {}
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

#[component]
fn DogView() -> Element {
    let skip = |_| info!("Skipped!");
    let save = || async {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random").await.unwrap();
        let text = response.text().await.unwrap();
        let dogApi: DogApi = serde_json::from_str(&text).unwrap();
        dogApi.message
    };
    let mut img_src = use_resource(save);
    rsx! {
        div { id: "dogview",
            img { src: {img_src} }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }
    }
}

use serde::Deserialize;
#[derive(Deserialize)]
struct DogApi {
    message: String,
}