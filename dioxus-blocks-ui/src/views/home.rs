//! # 主页

use dioxus::prelude::*;
use dioxus_blocks_components::{
    Card, Element, Grid, GridCols, GridItem, Link, Style, Text, ToElement,
};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct HomeView {}

impl ToElement for HomeView {
    fn to_element(&self) -> Element {
        let card_style = |s: Style| {
            s.text_align("center")
                .border_radius("12px")
                .background_color("#FAFAFA")
        };
        let text_style = |s: Style| s;
        let link_style = |s: Style| {
            s.height("100px")
                .line_height("100px")
                .text_decoration("unset")
        };

        Grid::new(vec![
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Text").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::TextViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Link").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::LinkViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Button").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::ButtonViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Image").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::ImageViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("View").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::ViewExampleRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Grid").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::GridViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Card").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::CardViewRoute {})
                    .style(link_style),
            ),
        ])
        .cols(GridCols::Col4)
        .gap(16)
        .to_element()
    }
}
