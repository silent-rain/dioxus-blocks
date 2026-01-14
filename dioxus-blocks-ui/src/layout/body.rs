//! # Body

use dioxus_blocks_components::{Element, ToElement, Wrap};

use crate::Route;

#[derive(Debug, Default, Clone)]
pub struct Body {}

impl ToElement for Body {
    fn to_element(&self) -> Element {
        Wrap::new()
            .class("t_body")
            .children(dioxus_blocks_components::Outlet::<Route>::default())
            .to_element()
    }
}
