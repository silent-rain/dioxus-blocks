//! Text 组件

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Text, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct TextView {}

impl TextView {
    pub fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl TextView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("Text 组件"),
            Text::p("文本组件，用于显示不同类型的文本内容。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![self.text_tag()])
    }

    /// 文本标签示例
    pub fn text_tag(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("标签用法"),
                Text::p("由tag属性来选择文本标签类型，支持H1-H6, P, Span等标签。"),
            ]))
            .childrens2(vec![
                Text::h1("这是 H1 标题"),
                Text::h2("这是 H2 标题"),
                Text::h3("这是 H3 标题"),
                Text::h4("这是 H4 标题"),
                Text::h5("这是 H5 标题"),
                Text::h6("这是 H6 标题"),
                Text::span("这是一个行内文本"),
                Text::p("这是一个段落文本"),
            ])
            .style(|s| s.margin_top("32px"))
    }
}
