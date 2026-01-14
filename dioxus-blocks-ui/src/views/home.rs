use dioxus::prelude::*;
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct HomeView {}

impl HomeView {
    pub fn to_element(&self) -> Element {
        rsx! {
            div { class: "grid-section",
                h1 { class: "Grid Cols 基本用法", "" }
                p { class: "指定网格中的列", "" }
            }

            div { class: "grid-section",
                h1 { class: "Grid Rows 基本用法", "" }
                p { class: "指定网格中的行", "" }
            }
        }
    }
}
