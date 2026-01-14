//! # Header
use std::sync::Arc;

use dioxus_blocks_components::{Element, ToElement, Wrap};

use crate::layout::Navbar;

#[derive(Debug, Default, Clone)]
pub struct Header {}

impl ToElement for Header {
    fn to_element(&self) -> Element {
        Wrap::new()
            .class("t_header")
            .children(Wrap::new().class("logo"))
            .childrens(vec![Arc::new(Navbar::default())])
            .to_element()
    }
}
