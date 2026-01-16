//! # Layout
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_blocks_components::{ToElement, View};
use dioxus_blocks_macro::Route as DbmRoute;

use crate::layout::{Body, Footer, Header};

#[derive(Debug, Default, Clone, DbmRoute)]
pub struct Layout {}

impl ToElement for Layout {
    fn to_element(&self) -> Element {
        View::new()
            .class("t_layout")
            .childrens(vec![
                Rc::new(Header::default()),
                Rc::new(Body::default()),
                Rc::new(Footer::default()),
            ])
            .to_element()
    }
}
