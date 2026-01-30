//! Blog 组件

use dioxus::prelude::*;
use dioxus_blocks_components::{Card, Link, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Clone, Route)]
pub struct Blog {
    id: i32,
}

impl ToElement for Blog {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .to_element()
    }
}

impl Blog {
    fn title(&self) -> View {
        View::new().childrens(vec![
            Text::h1(format!("Blog #{}", self.id)),
            Text::p("博客组件，展示 Dioxus 路由系统的参数传递。"),
        ])
    }

    fn content(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h2("博客详情"))
                    .children(Text::p(format!("当前博客 ID: {}", self.id))),
            )
            .body(
                View::new()
                    .children(Text::p(format!(
                        "在博客 #{} 中，我们展示 Dioxus 路由如何工作，以及 URL 参数如何作为属性传递给路由组件。",
                        self.id
                    )))
                    .children(
                        View::new()
                            .style(|s| s.display("flex").gap("16px").custom("padding-top: 16px"))
                            .children(
                                Link::new(crate::Route::BlogRoute { id: self.id - 1 })
                                    .text("上一页"),
                            )
                            .children(Text::span(" <---> "))
                            .children(
                                Link::new(crate::Route::BlogRoute { id: self.id + 1 })
                                    .text("下一页"),
                            ),
                    ),
            )
    }
}
