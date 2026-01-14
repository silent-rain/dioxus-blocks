//! # Header
use std::sync::Arc;

use dioxus_blocks_components::{Element, ToElement, View};

use crate::layout::Navbar;

#[derive(Debug, Default, Clone)]
pub struct Header {}

impl ToElement for Header {
    fn to_element(&self) -> Element {
        View::new()
            .class("t_header")
            .children(View::new().class("logo"))
            .childrens(vec![Arc::new(Navbar::default())])
            .to_element()
    }
}
