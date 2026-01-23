//! Button 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Button, Card, Text, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct ButtonView {}

impl ButtonView {
    pub fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl ButtonView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("Button 组件"),
            Text::p("按钮组件，支持多种样式和交互状态。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_button(),
            self.variant_buttons(),
            self.size_buttons(),
            self.style_buttons(),
        ])
    }

    /// 基础按钮
    fn basic_button(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础按钮"),
                Text::p("最简单的按钮使用方式。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .children(
                        Button::default()
                            .class("t-button t-button-primary")
                            .onclick(move |_| {
                                println!("按钮被点击了！");
                            })
                            .children(Text::new("点击我")),
                    ),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 变体按钮
    fn variant_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("按钮变体"),
                Text::p("不同颜色和风格的按钮。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::default()
                            .class("t-button t-button-primary")
                            .onclick(move |_| println!("主要按钮"))
                            .children(Text::new("主要按钮")),
                        Button::default()
                            .class("t-button t-button-secondary")
                            .onclick(move |_| println!("次要按钮"))
                            .children(Text::new("次要按钮")),
                        Button::default()
                            .class("t-button t-button-danger")
                            .onclick(move |_| println!("危险按钮"))
                            .children(Text::new("危险按钮")),
                        Button::default()
                            .class("t-button t-button-success")
                            .onclick(move |_| println!("成功按钮"))
                            .children(Text::new("成功按钮")),
                        Button::default()
                            .class("t-button t-button-warning")
                            .onclick(move |_| println!("警告按钮"))
                            .children(Text::new("警告按钮")),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 尺寸按钮
    fn size_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![Text::h2("按钮尺寸"), Text::p("不同大小的按钮。")]))
            .children(
                View::new()
                    .style(|s| {
                        s.padding("20px")
                            .display("flex")
                            .gap("12px")
                            .align_items("center")
                    })
                    .childrens2(vec![
                        Button::default()
                            .class("t-button t-button-primary t-button-small")
                            .onclick(move |_| println!("小按钮"))
                            .children(Text::new("小")),
                        Button::default()
                            .class("t-button t-button-primary t-button-medium")
                            .onclick(move |_| println!("中按钮"))
                            .children(Text::new("中")),
                        Button::default()
                            .class("t-button t-button-primary t-button-large")
                            .onclick(move |_| println!("大按钮"))
                            .children(Text::new("大")),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 特殊样式按钮
    fn style_buttons(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens2(vec![Text::h2("特殊样式"), Text::p("各种特殊效果的按钮。")]),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.padding("20px")
                            .display("flex")
                            .gap("12px")
                            .flex_wrap("wrap")
                    })
                    .childrens2(vec![
                        Button::default()
                            .class("t-button t-button-ghost")
                            .onclick(move |_| println!("幽灵按钮"))
                            .children(Text::new("幽灵")),
                        Button::default()
                            .class("t-button t-button-round")
                            .onclick(move |_| println!("圆形按钮"))
                            .children(Text::new("圆形")),
                        Button::default()
                            .class("t-button t-button-gradient")
                            .onclick(move |_| println!("渐变按钮"))
                            .children(Text::new("渐变")),
                        Button::default()
                            .class("t-button t-button-3d")
                            .onclick(move |_| println!("3D按钮"))
                            .children(Text::new("3D")),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }
}
