//! # Body

use dioxus_blocks_components::{Element, ToElement, View};

use crate::Route;

#[derive(Debug, Default, Clone)]
pub struct Body {}

impl ToElement for Body {
    fn to_element(&self) -> Element {
        View::new()
            .class("t_body")
            .children(dioxus_blocks_components::Outlet::<Route>::default())
            .to_element()
    }
}
