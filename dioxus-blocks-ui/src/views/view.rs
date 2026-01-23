//! View 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Text, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct ViewExample {}

impl ViewExample {
    pub fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl ViewExample {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("View 组件"),
            Text::p("通用容器组件，用于包装其他元素，支持丰富的样式配置。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_example(),
            self.style_example(),
            self.layout_example(),
        ])
    }

    /// 基础容器示例
    fn basic_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础用法"),
                Text::p("View 组件的基本使用方式。"),
            ]))
            .children(
                View::new()
                    .style(|s| {
                        s.padding("24px")
                            .background_color("#f9fafb")
                            .border_radius("8px")
                    })
                    .children(Text::p("这是一个基础容器。")),
            )
    }

    /// 样式示例
    fn style_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("样式配置"),
                Text::p("通过链式调用配置各种样式属性。"),
            ]))
            .children(
                View::new()
                    .style(|s| {
                        s.padding("20px")
                            .background_color("white")
                            .border("1px solid #e5e7eb")
                            .border_radius("8px")
                            .box_shadow("0 2px 4px rgba(0,0,0,0.1)")
                    })
                    .children(Text::p("支持多种样式配置："))
                    .childrens2(vec![
                        View::new()
                            .style(|s| s.margin_top("12px").padding_left("20px"))
                            .childrens2(vec![
                                Text::p("- 内边距"),
                                Text::p("- 外边距"),
                                Text::p("- 背景颜色"),
                                Text::p("- 边框"),
                                Text::p("- 阴影"),
                            ]),
                    ]),
            )
    }

    /// 布局示例
    fn layout_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("布局功能"),
                Text::p("使用 Flexbox 布局实现元素排列。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.display("flex").gap("16px").padding("20px"))
                    .childrens2(vec![
                        View::new()
                            .style(|s| {
                                s.flex("1")
                                    .padding("16px")
                                    .background_color("#dbeafe")
                                    .border_radius("8px")
                            })
                            .children(Text::p("项目 1")),
                        View::new()
                            .style(|s| {
                                s.flex("1")
                                    .padding("16px")
                                    .background_color("#bfdbfe")
                                    .border_radius("8px")
                            })
                            .children(Text::p("项目 2")),
                        View::new()
                            .style(|s| {
                                s.flex("1")
                                    .padding("16px")
                                    .background_color("#93c5fd")
                                    .border_radius("8px")
                            })
                            .children(Text::p("项目 3")),
                    ]),
            )
    }
}
