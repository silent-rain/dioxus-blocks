//! # 主页

use dioxus::prelude::*;
use dioxus_blocks_components::{Element, Grid, GridCols, GridItem, Text, ToElement};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct HomeView {}

impl ToElement for HomeView {
    fn to_element(&self) -> Element {
        Grid::new(vec![
            GridItem::new(Text::new("11")),
            GridItem::new(Text::new("22")),
            GridItem::new(Text::new("33")),
            GridItem::new(Text::new("44")),
            GridItem::new(Text::new("55")),
        ])
        .cols(GridCols::Col4)
        .to_element()
    }
}
