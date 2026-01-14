//! Grid 组件

use dioxus::prelude::*;

use dioxus_blocks_components::{Grid, GridCols, GridItem, GridRows, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct GridView {}

impl ToElement for GridView {
    fn to_element(&self) -> Element {
        View::new()
            .childrens2(vec![self.basic_grid_cols(), self.basic_grid_rows()])
            .to_element()
    }
}

impl GridView {
    /// Cols 基本用法
    pub fn basic_grid_cols(&self) -> Grid {
        Grid::new(
            (1..=12)
                .map(|i| GridItem::new(Text::new(format!("Item {i}")).class("grid-item")))
                .collect(),
        )
        .cols(GridCols::Col4)
    }
    /// Rows 基本用法
    pub fn basic_grid_rows(&self) -> Grid {
        Grid::new(
            (1..=12)
                .map(|i| GridItem::new(Text::new(format!("Item {i}")).class("grid-item")))
                .collect(),
        )
        .rows(GridRows::Row4)
        .gap_xy(4, 8)
    }
}
