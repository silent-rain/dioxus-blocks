//! # Layout
use std::sync::Arc;

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
                Arc::new(Header::default()),
                Arc::new(Body::default()),
                Arc::new(Footer::default()),
            ])
            .to_element()
    }
}
