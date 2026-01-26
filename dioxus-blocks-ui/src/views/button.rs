//! Button 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{
    Button, ButtonShape, ButtonSize, ButtonType, Card, Text, ToElement, View,
};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct ButtonView {}

impl ToElement for ButtonView {
    fn to_element(&self) -> Element {
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
            Text::p("按钮组件，支持多种类型、变体、形状和尺寸。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_buttons(),
            self.plain_buttons(),
            self.round_buttons(),
            self.circle_buttons(),
            self.size_buttons(),
            self.state_buttons(),
            self.link_buttons(),
            self.text_buttons(),
            self.counter_example(),
        ])
    }

    /// 基础按钮（实心）
    fn basic_buttons(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens2(vec![Text::h2("基础按钮"), Text::p("不同类型的实心按钮。")]),
            )
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Default").onclick(|_| {
                            println!("Button clicked again!");
                        }),
                        Button::new().text("Primary").btn_type(ButtonType::Primary),
                        Button::new().text("Success").btn_type(ButtonType::Success),
                        Button::new().text("Info").btn_type(ButtonType::Info),
                        Button::new().text("Warning").btn_type(ButtonType::Warning),
                        Button::new().text("Danger").btn_type(ButtonType::Danger),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 椭圆按钮
    fn round_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![Text::h2("椭圆按钮"), Text::p("椭圆形状的按钮。")]))
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Round").shape(ButtonShape::Round),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Round),
                        Button::new()
                            .text("Success")
                            .btn_type(ButtonType::Success)
                            .shape(ButtonShape::Round),
                        Button::new()
                            .text("Info")
                            .btn_type(ButtonType::Info)
                            .shape(ButtonShape::Round),
                        Button::new()
                            .text("Warning")
                            .btn_type(ButtonType::Warning)
                            .shape(ButtonShape::Round),
                        Button::new()
                            .text("Danger")
                            .btn_type(ButtonType::Danger)
                            .shape(ButtonShape::Round),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 圆形按钮
    fn circle_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("圆形按钮"),
                Text::p("圆形形状的按钮，适合配合图标使用。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new()
                            .shape(ButtonShape::Circle)
                            .btn_type(ButtonType::Default)
                            .text("D"),
                        Button::new()
                            .shape(ButtonShape::Circle)
                            .btn_type(ButtonType::Primary)
                            .text("P"),
                        Button::new()
                            .shape(ButtonShape::Circle)
                            .btn_type(ButtonType::Success)
                            .text("S"),
                        Button::new()
                            .shape(ButtonShape::Circle)
                            .btn_type(ButtonType::Info)
                            .text("I"),
                        Button::new()
                            .shape(ButtonShape::Circle)
                            .btn_type(ButtonType::Warning)
                            .text("W"),
                        Button::new()
                            .shape(ButtonShape::Circle)
                            .btn_type(ButtonType::Danger)
                            .text("D"),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 不同尺寸按钮
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
                        Button::new()
                            .text("Small")
                            .size(ButtonSize::Small)
                            .btn_type(ButtonType::Primary),
                        Button::new()
                            .text("Medium")
                            .size(ButtonSize::Medium)
                            .btn_type(ButtonType::Primary),
                        Button::new()
                            .text("Large")
                            .size(ButtonSize::Large)
                            .btn_type(ButtonType::Primary),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 状态按钮
    fn state_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("按钮状态"),
                Text::p("禁用和加载状态的按钮。"),
            ]))
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Normal").btn_type(ButtonType::Primary),
                        Button::new()
                            .text("Disabled")
                            .btn_type(ButtonType::Primary)
                            .disabled(true),
                        Button::new()
                            .text("Loading")
                            .btn_type(ButtonType::Primary)
                            .loading(true),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 朴素按钮
    fn plain_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("朴素按钮"),
                Text::p("朴素样式的按钮，带有边框和浅色背景。"),
            ]))
            .childrens2(vec![
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Plain").shape(ButtonShape::Plain),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Plain),
                        Button::new()
                            .text("Success")
                            .btn_type(ButtonType::Success)
                            .shape(ButtonShape::Plain),
                        Button::new()
                            .text("Info")
                            .btn_type(ButtonType::Info)
                            .shape(ButtonShape::Plain),
                        Button::new()
                            .text("Warning")
                            .btn_type(ButtonType::Warning)
                            .shape(ButtonShape::Plain),
                        Button::new()
                            .text("Danger")
                            .btn_type(ButtonType::Danger)
                            .shape(ButtonShape::Plain),
                    ]),
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new()
                            .text("Disabled Plain")
                            .shape(ButtonShape::Plain)
                            .disabled(true),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Plain)
                            .disabled(true),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 链接按钮
    fn link_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("链接按钮"),
                Text::p("链接样式的按钮，常用于页面内导航。"),
            ]))
            .childrens2(vec![
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Link").shape(ButtonShape::Link),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Link),
                        Button::new()
                            .text("Success")
                            .btn_type(ButtonType::Success)
                            .shape(ButtonShape::Link),
                        Button::new()
                            .text("Info")
                            .btn_type(ButtonType::Info)
                            .shape(ButtonShape::Link),
                        Button::new()
                            .text("Warning")
                            .btn_type(ButtonType::Warning)
                            .shape(ButtonShape::Link),
                        Button::new()
                            .text("Danger")
                            .btn_type(ButtonType::Danger)
                            .shape(ButtonShape::Link),
                    ]),
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new()
                            .text("Disabled Link")
                            .shape(ButtonShape::Link)
                            .disabled(true),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Link)
                            .disabled(true),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 文字按钮
    fn text_buttons(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("文字按钮"),
                Text::p("文字样式的按钮，hover 时显示背景色。"),
            ]))
            .childrens2(vec![
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Text").shape(ButtonShape::Text),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Text),
                        Button::new()
                            .text("Success")
                            .btn_type(ButtonType::Success)
                            .shape(ButtonShape::Text),
                        Button::new()
                            .text("Info")
                            .btn_type(ButtonType::Info)
                            .shape(ButtonShape::Text),
                        Button::new()
                            .text("Warning")
                            .btn_type(ButtonType::Warning)
                            .shape(ButtonShape::Text),
                        Button::new()
                            .text("Danger")
                            .btn_type(ButtonType::Danger)
                            .shape(ButtonShape::Text),
                    ]),
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new()
                            .text("Disabled Text")
                            .shape(ButtonShape::Text)
                            .disabled(true),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .shape(ButtonShape::Text)
                            .disabled(true),
                    ]),
            ])
            .style(|s| s.margin_top("32px"))
    }

    /// 计数器示例 - 按钮与文本联动
    fn counter_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("计数器示例"),
                Text::p("按钮与 Text 组件的联动，点击按钮更新文本内容。"),
            ]))
            .children(CounterExample::default())
            .style(|s| s.margin_top("32px"))
    }
}

/// 状态组件 - 用于演示按钮和文本的联动
#[derive(Debug, Default, Clone)]
pub struct CounterExample {}

impl ToElement for CounterExample {
    fn to_element(&self) -> Element {
        let mut count = use_signal(|| 0);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                Text::new(format!("点击次数: {}", count()))
                    .style(|s| s.font_size("16px").color("#303133")),
            )
            .childrens2(vec![
                Button::new()
                    .text("点击加 1")
                    .as_primary()
                    .onclick(move |_| count.set(count() + 1)),
                Button::new()
                    .text("点击减 1")
                    .as_success()
                    .onclick(move |_| count.set(count() - 1)),
                Button::new()
                    .text("重置")
                    .as_warning()
                    .onclick(move |_| count.set(0)),
            ])
            .into()
    }
}
