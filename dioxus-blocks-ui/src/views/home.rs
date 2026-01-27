//! # 主页

use dioxus::prelude::*;
use dioxus_blocks_components::{Grid, GridCols, GridItem, Link, Style, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct HomeView {}

impl ToElement for HomeView {
    fn to_element(&self) -> Element {
        View::new()
            .style(|s| s.padding("40px 24px"))
            .children(self.header())
            .children(self.component_grid())
            .into()
    }
}

impl HomeView {
    /// 页面头部
    fn header(&self) -> View {
        View::new()
            .style(|s| s.text_align("center").margin_bottom("48px"))
            .childrens2(vec![
                Text::h1("Dioxus Blocks 组件库").style(|s| {
                    s.font_size("36px")
                        .font_weight("700")
                        .color("#303133")
                        .margin_bottom("16px")
                }),
                Text::p("基于 Dioxus 的现代化 UI 组件库")
                    .style(|s| s.font_size("16px").color("#909399")),
            ])
    }

    /// 组件网格
    fn component_grid(&self) -> Grid {
        let card_style = |s: Style| {
            s.border_radius("16px")
                .background_color("white")
                .box_shadow("0 2px 12px rgba(0, 0, 0, 0.08)")
                .padding("32px 24px")
                .height("160px")
                .min_width("200px")
                .min_height("160px")
                .display("flex")
                .flex_direction("column")
                .align_items("center")
                .justify_content("center")
                .cursor("pointer")
                .transition("all 0.3s ease")
        };

        let icon_style = |s: Style| s.font_size("48px").margin_bottom("16px");

        let title_style = |s: Style| s.font_size("18px").font_weight("600").color("#303133");

        let components = vec![
            ("📝", Text::new("Text"), crate::Route::TextViewRoute {}),
            ("🔗", Text::new("Link"), crate::Route::LinkViewRoute {}),
            ("🔘", Text::new("Button"), crate::Route::ButtonViewRoute {}),
            ("🖼️", Text::new("Image"), crate::Route::ImageViewRoute {}),
            ("📦", Text::new("Card"), crate::Route::CardViewRoute {}),
            ("📐", Text::new("Grid"), crate::Route::GridViewRoute {}),
            ("📏", Text::new("Layout"), crate::Route::LayoutViewRoute {}),
            ("👁️", Text::new("View"), crate::Route::ViewExampleRoute {}),
            (
                "🔢",
                Text::new("InputNumber"),
                crate::Route::InputNumberViewRoute {},
            ),
        ];

        Grid::new(
            components
                .into_iter()
                .map(|(icon, title, route)| {
                    GridItem::new(
                        Link::default()
                            .to(route)
                            .children(View::new().style(icon_style).children(Text::new(icon)))
                            .children(title.style(title_style))
                            .style(card_style),
                    )
                })
                .collect(),
        )
        .cols(GridCols::Col4)
        .gap(24)
    }
}
