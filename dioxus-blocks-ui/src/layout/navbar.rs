//! # Navbar
use crate::Route;
use dioxus::prelude::*;
use dioxus_blocks_components::ToElement;

#[derive(Debug, Default, Clone)]
pub struct Navbar {}

impl ToElement for Navbar {
    fn to_element(&self) -> Element {
        rsx! {
            div { class: "t_navbar",
                Link { to: Route::HomeViewRoute {}, "Home" }
                Link { to: Route::Blog { id: 1 }, "Blog" }
                // Link { to: Route::CardViewRoute {}, "Card" }
                // Link { to: Route::GridViewRoute {}, "Grid" }
                Link { to: Route::TextViewRoute {}, "Text" }
            }

        }
    }
}
