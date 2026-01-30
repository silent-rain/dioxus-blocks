//! Card 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, CardShadow, Text, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct CardView {}

impl CardView {
    pub fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl CardView {
    fn title(&self) -> View {
        View::new().childrens(vec![
            Text::h1("Card 组件"),
            Text::p("卡片组件，用于展示相关内容，支持多种布局和样式配置。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens(vec![
            self.basic_card(),
            self.header_and_body_card(),
            self.border_divider_card(),
            self.footer_and_body_card(),
            self.header_body_footer_card(),
            self.children_card(),
            self.shadow_card(),
        ])
    }

    /// 基础卡片
    fn basic_card(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens(vec![Text::h2("极简卡片"), Text::p("仅有内容的卡片形式。")]),
            )
            .childrens(vec![Text::p(
                "This is a basic card with only body content.",
            )])
            .style(|s| s.margin_top("32px"))
    }

    /// 带标题栏的卡片
    fn header_and_body_card(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("带 header 的卡片"),
                Text::p(
                    "由极简卡片上方的标题栏组成，标题栏中可包含标题、图片、操作区、状态等内容。",
                ),
            ]))
            .childrens(vec![
                Text::p("List item 1"),
                Text::p("List item 2"),
                Text::p("List item 3"),
                Text::p("List item 4"),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 边框与分割线控制
    fn border_divider_card(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("边框与分割线控制"),
                Text::p("通过 border 属性控制卡片是否显示边框，通过 header_divider 属性控制标题与内容之间是否显示分割线。"),
            ]))
            .header_divider(false)
            .childrens(vec![
                Text::p("List item 1"),
                Text::p("List item 2"),
                Text::p("List item 3"),
                Text::p("List item 4"),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 带底部栏的卡片
    fn footer_and_body_card(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("带 footer 的卡片"),
                Text::p("由极简卡片下方的操作栏组成，操作栏中可包含按钮、链接、操作区等内容。"),
            ]))
            .childrens(vec![
                Text::p("List item 1"),
                Text::p("List item 2"),
                Text::p("List item 3"),
                Text::p("List item 4"),
            ])
            .footer(View::new().childrens(vec![Text::p("Footer content")]))
            .style(|s| s.margin_top("32px"))
    }

    /// 完整卡片（header + body + footer）
    fn header_body_footer_card(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("同时带 header 和 footer 的卡片"),
                Text::p("由顶部栏、底部栏和极简卡片组成的复杂卡片，三个区域内容可根据需要对内容进行配置。"),
            ]))
            .childrens(vec![
                Text::p("List item 1"),
                Text::p("List item 2"),
                Text::p("List item 3"),
                Text::p("List item 4"),
            ])
            .footer(View::new().childrens(vec![Text::p("Footer content")]))
            .style(|s| s.margin_top("32px"))
    }

    /// 使用 children 的卡片
    fn children_card(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("带 children 的卡片"),
                Text::p("使用 children 属性替代 body 属性，可以使用任意元素作为卡片内容。"),
            ]))
            .childrens(vec![
                Text::p("This card uses children instead of body prop."),
                Text::p("You can add multiple children elements."),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 阴影效果控制
    fn shadow_card(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("阴影效果控制"),
                Text::p("通过 shadow 属性设置卡片阴影出现的时机。该属性的值可以是：always、hover 或 never。"),
            ]))
            .childrens(vec![
                View::new()
                    .style(|s| s.display("flex").gap("20px").flex_wrap("wrap"))
                    .childrens(vec![
                        Card::new()
                            .shadow(CardShadow::Always)
                            .childrens(vec![Text::p("Always Shadow")])
                            .style(|s| s.width("200px")),
                        Card::new()
                            .shadow(CardShadow::Hover)
                            .childrens(vec![Text::p("Hover Shadow")])
                            .style(|s| s.width("200px")),
                        Card::new()
                            .shadow(CardShadow::Never)
                            .childrens(vec![Text::p("No Shadow")])
                            .style(|s| s.width("200px")),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }
}
