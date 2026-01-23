//! Button 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Button, ButtonShape, ButtonSize, ButtonType, Card, Text, View};
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
                        Button::new().text("Default"),
                        Button::new().text("Primary").btn_type(ButtonType::Primary),
                        Button::new().text("Success").btn_type(ButtonType::Success),
                        Button::new().text("Info").btn_type(ButtonType::Info),
                        Button::new().text("Warning").btn_type(ButtonType::Warning),
                        Button::new().text("Danger").btn_type(ButtonType::Danger),
                    ]),
            )
            .style(|s| s.margin_top("32px"))
    }

    /// 浅色按钮
    fn plain_buttons(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens2(vec![Text::h2("浅色按钮"), Text::p("浅色背景的按钮样式。")]),
            )
            .children(
                View::new()
                    .style(|s| s.padding("20px").display("flex").gap("12px"))
                    .childrens2(vec![
                        Button::new().text("Plain").plain(true),
                        Button::new()
                            .text("Primary")
                            .btn_type(ButtonType::Primary)
                            .plain(true),
                        Button::new()
                            .text("Success")
                            .btn_type(ButtonType::Success)
                            .plain(true),
                        Button::new()
                            .text("Info")
                            .btn_type(ButtonType::Info)
                            .plain(true),
                        Button::new()
                            .text("Warning")
                            .btn_type(ButtonType::Warning)
                            .plain(true),
                        Button::new()
                            .text("Danger")
                            .btn_type(ButtonType::Danger)
                            .plain(true),
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
}
