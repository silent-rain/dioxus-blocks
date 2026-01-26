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
            Text::p("链接组件，用于页面导航和跳转，支持多种类型和下划线样式。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_links(),
            self.type_links(),
            self.underline_links(),
            self.disabled_links(),
        ])
    }

    /// 基础链接
    fn basic_links(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础链接"),
                Text::p("使用 .to() 方法设置链接目标。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.padding("20px"))
                    .childrens2(vec![Link::default().to("/home").text("返回首页")]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 类型链接
    fn type_links(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![Text::h2("类型链接"), Text::p("不同类型的链接。")]))
            .childrens2(vec![
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Link::default().to("/home").text("Default"),
                        Link::default().to("/home").text("Primary").as_primary(),
                        Link::default().to("/home").text("Success").as_success(),
                        Link::default().to("/home").text("Info").as_info(),
                        Link::default().to("/home").text("Warning").as_warning(),
                        Link::default().to("/home").text("Danger").as_danger(),
                    ]),
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Link::default()
                            .to("/home")
                            .text("Primary Link")
                            .as_primary(),
                        Link::default()
                            .to("/about")
                            .text("Success Link")
                            .as_success(),
                        Link::default()
                            .to("/settings")
                            .text("Warning Link")
                            .as_warning(),
                        Link::default()
                            .to("/danger")
                            .text("Danger Link")
                            .as_danger(),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 下划线样式
    fn underline_links(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("下划线样式"),
                Text::p("不同的下划线显示方式。"),
            ]))
            .childrens2(vec![
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Link::default().to("/home").text("Default"),
                        Link::default()
                            .to("/home")
                            .text("Always")
                            .as_underline_always(),
                        Link::default()
                            .to("/home")
                            .text("Hover")
                            .as_underline_hover(),
                        Link::default()
                            .to("/home")
                            .text("Never")
                            .as_underline_never(),
                    ]),
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Link::default()
                            .to("/home")
                            .text("Primary Always")
                            .as_primary()
                            .as_underline_always(),
                        Link::default()
                            .to("/home")
                            .text("Success Hover")
                            .as_success()
                            .as_underline_hover(),
                        Link::default()
                            .to("/home")
                            .text("Info Never")
                            .as_info()
                            .as_underline_never(),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用链接
    fn disabled_links(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![Text::h2("禁用链接"), Text::p("禁用状态的链接。")]))
            .childrens2(vec![
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Link::default().to("/home").text("Default").disabled(),
                        Link::default()
                            .to("/home")
                            .text("Primary")
                            .as_primary()
                            .disabled(),
                        Link::default()
                            .to("/home")
                            .text("Success")
                            .as_success()
                            .disabled(),
                        Link::default()
                            .to("/home")
                            .text("Info")
                            .as_info()
                            .disabled(),
                        Link::default()
                            .to("/home")
                            .text("Warning")
                            .as_warning()
                            .disabled(),
                        Link::default()
                            .to("/home")
                            .text("Danger")
                            .as_danger()
                            .disabled(),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }
}
