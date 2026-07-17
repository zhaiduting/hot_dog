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
    let mut img_src = use_signal(|| "".to_string());
    // 改用 use_signal(|| "") 会导致生命周期问题
    
    let skip = move |_| info!("Skipped!");
    let save = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random").await.unwrap();
        let text = response.text().await.unwrap();
        let dogApi: DogApi = serde_json::from_str(&text).unwrap();
        img_src.set(dogApi.message);
    };
    rsx! {
        div { id: "dogview",
            img { src: {img_src} }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}

use serde::Deserialize;
#[derive(Deserialize)]
struct DogApi {
    message: String,
}