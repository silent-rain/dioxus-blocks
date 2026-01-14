//! # Footer

use dioxus_blocks_components::{Element, ToElement, View};

#[derive(Debug, Default, Clone)]
pub struct Footer {}

impl ToElement for Footer {
    fn to_element(&self) -> Element {
        View::new().class("t_footer").to_element()
    }
}
