//! # Sidebar
use dioxus_blocks_components::{Element, ToElement, Wrap};

#[derive(Debug, Default, Clone)]
pub struct Sidebar {}

impl ToElement for Sidebar {
    fn to_element(&self) -> Element {
        Wrap::new().class("t_sidebar").to_element()
    }
}
