use crate::backend::list_dogs;
use dioxus::prelude::*;

#[component]
pub fn Favourites() -> Element {
    // create a pending resource that resolves to the list of dogs from the backend
    // wait for the favourites list to resolve with `.suspend()`
    let mut favourites = use_resource(list_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favourites().unwrap() {
                    div { key: "{id}", class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
