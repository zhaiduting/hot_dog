use dioxus::prelude::*;
#[component]
pub fn DogView() -> Element {
    let skip = |_| info!("Skipped!");
    let save = || async {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap();
        let text = response.text().await.unwrap();
        let dogApi: DogApi = serde_json::from_str(&text).unwrap();
        dogApi.message
    };
    let mut img_src = use_resource(save);
    rsx! {
        div { id: "dogview",
            img { src: img_src }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button {
                onclick: move |_| async move {
                    let current = img_src.clone().unwrap();
                    img_src.restart();
                    _ = crate::backend::save_dog(current).await;
                },
                id: "save",
                "save!"
            }
        }
    }
}

use serde::Deserialize;
#[derive(Deserialize)]
struct DogApi {
    message: String,
}
