use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "Hi" }
            }
            Link { to: Route::Favourites, id: "heart", "heart_emoji" }
        }

        Outlet::<Route> {}
    }
}
