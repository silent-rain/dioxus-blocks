//! Grid 组件

use dioxus::prelude::*;
use dioxus_blocks_components::{Card, Grid, GridCols, GridItem, GridRows, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct GridView {}

impl ToElement for GridView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .to_element()
    }
}

impl GridView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("Grid 网格布局"),
            Text::p("网格布局组件，用于创建灵活的网格结构。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.cols_section(),
            self.rows_section(),
            self.col_span_section(),
            self.row_span_section(),
        ])
    }

    fn cols_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("列数配置 (Cols)"))
                    .children(Text::p("通过 cols 属性设置网格的列数，支持 1-12 列。")),
            )
            .body(
                Grid::new(
                    (1..=8)
                        .map(|i| {
                            GridItem::new(Text::new(format!("{}", i))).style(|s| {
                                s.display("flex")
                                    .align_items("center")
                                    .custom("justify-content: center")
                                    .padding("24px")
                                    .background_color("#f0f0f0")
                                    .border_radius("8px")
                                    .color("#333")
                                    .font_size("16px")
                                    .font_weight("500")
                            })
                        })
                        .collect(),
                )
                .cols(GridCols::Col4)
                .gap(12),
            )
            .style(|s| s.margin_top("32px"))
    }

    fn rows_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("行数配置 (Rows)"))
                    .children(Text::p("通过 rows 属性设置网格的行数，支持 1-12 行。")),
            )
            .body(
                Grid::new(
                    (1..=8)
                        .map(|i| {
                            GridItem::new(Text::new(format!("{}", i))).style(|s| {
                                s.display("flex")
                                    .align_items("center")
                                    .custom("justify-content: center")
                                    .padding("24px")
                                    .background_color("#f0f0f0")
                                    .border_radius("8px")
                                    .color("#333")
                                    .font_size("16px")
                                    .font_weight("500")
                            })
                        })
                        .collect(),
                )
                .rows(GridRows::Row2)
                .cols(GridCols::Col4)
                .gap(12),
            )
            .style(|s| s.margin_top("32px"))
    }

    fn col_span_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("跨列配置 (Col Span)"))
                    .children(Text::p(
                        "通过 GridItem 的 col_span 方法设置元素跨越的列数。",
                    )),
            )
            .body(
                Grid::new(vec![
                    GridItem::new(Text::new("Span 1")).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#f0f0f0")
                            .border_radius("8px")
                            .color("#333")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                    GridItem::new(Text::new("Span 2")).col_span(2).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#e3f2fd")
                            .border_radius("8px")
                            .color("#fff")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                    GridItem::new(Text::new("Span 1")).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#f0f0f0")
                            .border_radius("8px")
                            .color("#333")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                ])
                .cols(GridCols::Col4)
                .gap(12),
            )
            .style(|s| s.margin_top("32px"))
    }

    fn row_span_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("跨行配置 (Row Span)"))
                    .children(Text::p(
                        "通过 GridItem 的 row_span 方法设置元素跨越的行数。",
                    )),
            )
            .body(
                Grid::new(vec![
                    GridItem::new(Text::new("Normal")).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#f0f0f0")
                            .border_radius("8px")
                            .color("#333")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                    GridItem::new(Text::new("Span 2")).row_span(2).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#e3f2fd")
                            .border_radius("8px")
                            .color("#fff")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                    GridItem::new(Text::new("Normal")).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#f0f0f0")
                            .border_radius("8px")
                            .color("#333")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                    GridItem::new(Text::new("Normal")).style(|s| {
                        s.display("flex")
                            .align_items("center")
                            .custom("justify-content: center")
                            .padding("24px")
                            .background_color("#f0f0f0")
                            .border_radius("8px")
                            .color("#333")
                            .font_size("16px")
                            .font_weight("500")
                    }),
                ])
                .cols(GridCols::Col2)
                .rows(GridRows::Row2)
                .gap(12),
            )
            .style(|s| s.margin_top("32px"))
    }
}
