//! Grid 组件

use dioxus::prelude::*;

use dioxus_blocks_components::{Grid, GridCols, GridRows};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct GridView {}

impl GridView {
    pub fn element(&self) -> Element {
        rsx! {
            div { class: "grid-section",
                h1 { class: "Grid Cols 基本用法", "" }
                p { class: "指定网格中的列", "" }
            }
            {self.basic_grid_cols()}

            div { class: "grid-section",
                h1 { class: "Grid Rows 基本用法", "" }
                p { class: "指定网格中的行", "" }
            }
            {self.basic_grid_rows()}
        }
    }
}

impl GridView {
    /// Cols 基本用法
    pub fn basic_grid_cols(&self) -> Element {
        Grid::new()
            .cols(GridCols::Col4)
            .childrens(
                (1..=12)
                    .map(|i| {
                        rsx! {
                            div { class: "grid-item", "Item {i}" }
                        }
                    })
                    .collect(),
            )
            .into()
    }
    /// Rows 基本用法
    pub fn basic_grid_rows(&self) -> Element {
        Grid::new()
            .rows(GridRows::Row4)
            .gap_xy(4, 8)
            .childrens(
                (1..=12)
                    .map(|i| {
                        rsx! {
                            div { class: "grid-item", "Item {i}" }
                        }
                    })
                    .collect(),
            )
            .into()
    }
}
