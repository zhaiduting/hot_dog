use crate::components::*;
use dioxus::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    DogView {},

    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
