//! # Body

use dioxus_blocks_components::{Element, Outlet, ToElement, View};

use crate::Route;

#[derive(Debug, Default, Clone)]
pub struct Body {}

impl ToElement for Body {
    fn to_element(&self) -> Element {
        View::new()
            .class("t_body")
            .children(Outlet::<Route>::default())
            .style(|s| s.padding("20px"))
            .to_element()
    }
}
