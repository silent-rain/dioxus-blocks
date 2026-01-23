//! Link 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Link, Text, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct LinkView {}

impl LinkView {
    pub fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl LinkView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("Link 组件"),
            Text::p("链接组件，用于页面导航和跳转。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_link(),
            self.styled_link(),
            self.icon_link(),
        ])
    }

    /// 基础链接
    fn basic_link(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础链接"),
                Text::p("使用 .to() 方法设置链接目标。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.padding("20px"))
                    .children(Text::p("访问首页："))
                    .childrens2(vec![
                        Link::default()
                            .to(crate::Route::HomeViewRoute {})
                            .class("t-link")
                            .children(Text::new("返回首页")),
                    ]),
            )
    }

    /// 样式链接
    fn styled_link(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens2(vec![Text::h2("样式链接"), Text::p("带有悬停效果的链接。")]),
            )
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Link::default()
                            .to(crate::Route::HomeViewRoute {})
                            .class("t-link-button")
                            .children(Text::new("悬停查看效果")),
                    ]),
            )
    }

    /// 带图标的链接
    fn icon_link(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("图标链接"),
                Text::p("包含图标和文本的链接。"),
            ]))
            .children(
                View::new()
                    .style(|s| {
                        s.padding("20px")
                            .display("flex")
                            .gap("16px")
                            .flex_wrap("wrap")
                    })
                    .childrens2(vec![
                        Link::default()
                            .to(crate::Route::HomeViewRoute {})
                            .class("t-link-icon")
                            .childrens2(vec![
                                Text::new("🏠").style(|s| s.font_size("20px")),
                                Text::new("返回首页"),
                            ]),
                    ]),
            )
    }
}
