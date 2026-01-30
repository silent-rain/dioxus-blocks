//! Radio 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Radio, RadioGroup, RadioValue, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct RadioView {}

impl ToElement for RadioView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl RadioView {
    fn title(&self) -> View {
        View::new().childrens(vec![
            Text::h1("Radio 单选框"),
            Text::p("在一组备选项中进行单选，支持基础用法、禁用状态、单选框组、带有边框和单选按钮等功能。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens(vec![
            self.basic_usage(),
            self.disabled_state(),
            self.radio_group(),
            self.with_border(),
            self.radio_button(),
            self.different_sizes(),
        ])
    }

    /// 基础用法
    fn basic_usage(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("基础用法"),
                Text::p("单选框不应该有太多的可选项，如果你有很多的可选项你应该使用选择框而不是单选框。"),
            ]))
            .children(BasicUsage::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用状态
    fn disabled_state(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("禁用状态"),
                Text::p("disabled 属性可以用来控制单选框的禁用状态。"),
            ]))
            .children(DisabledState::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 单选框组
    fn radio_group(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("单选框组"),
                Text::p("适用于在多个互斥的选项中选择的场景。"),
            ]))
            .children(RadioGroupExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 带有边框
    fn with_border(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("带有边框"),
                Text::p("设置 border 属性为 true 可以渲染为带有边框的单选框。"),
            ]))
            .children(WithBorder::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 单选按钮
    fn radio_button(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("单选按钮"),
                Text::p("带有按钮组视觉效果的单选框。"),
            ]))
            .children(RadioButtonExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 不同尺寸
    fn different_sizes(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("不同尺寸"),
                Text::p("提供大、中、小三种尺寸。"),
            ]))
            .children(DifferentSizes::default())
            .style(|s| s.margin_top("32px"))
    }
}

/// 基础用法示例
#[derive(Debug, Default, Clone)]
pub struct BasicUsage {}

impl ToElement for BasicUsage {
    fn to_element(&self) -> Element {
        let radio_int = use_signal(|| RadioValue::Int(1));
        let radio_float = use_signal(|| RadioValue::Float(1.5));
        let radio_bool = use_signal(|| RadioValue::Bool(false));
        let radio_string = use_signal(|| RadioValue::String("New York".to_string()));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // int 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("Int 类型: "))
                    .children(
                        Radio::new()
                            .value(1)
                            .label("选项 1")
                            .checked_value(radio_int),
                    )
                    .children(
                        Radio::new()
                            .value(2)
                            .label("选项 2")
                            .checked_value(radio_int),
                    )
                    .children(
                        Radio::new()
                            .value(3)
                            // .label("选项 3")
                            .label_element(Text::new("选项 3"))
                            .checked_value(radio_int),
                    )
                    .children(Text::new(format!(
                        "当前值: {:?}",
                        radio_int.read().get_int()
                    ))),
            )
            // float 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("Float 类型: "))
                    .children(
                        Radio::new()
                            .value(1.5)
                            .label("选项 1.5")
                            .checked_value(radio_float),
                    )
                    .children(
                        Radio::new()
                            .value(2.5)
                            .label("选项 2.5")
                            .checked_value(radio_float),
                    )
                    .children(
                        Radio::new()
                            .value(3.5)
                            .label("选项 3.5")
                            .checked_value(radio_float),
                    )
                    .children(Text::new(format!(
                        "当前值: {:?}",
                        radio_float.read().get_float()
                    ))),
            )
            // string 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("String 类型: "))
                    .children(
                        Radio::new()
                            .value("New York")
                            .label("New York")
                            .checked_value(radio_string),
                    )
                    .children(
                        Radio::new()
                            .value("Washington")
                            .label("Washington")
                            .checked_value(radio_string),
                    )
                    .children(
                        Radio::new()
                            .value("Los Angeles")
                            .label("Los Angeles")
                            .checked_value(radio_string),
                    )
                    .children(Text::new(format!(
                        "当前值: {:?}",
                        radio_string.read().get_string()
                    ))),
            )
            // bool 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("Bool 类型: "))
                    .children(
                        Radio::new()
                            .value(false)
                            .label("False")
                            .checked_value(radio_bool),
                    )
                    .children(
                        Radio::new()
                            .value(true)
                            .label("True")
                            .checked_value(radio_bool),
                    )
                    .children(Text::new(format!(
                        "当前值: {:?}",
                        radio_bool.read().get_bool()
                    ))),
            )
            .into()
    }
}

/// 禁用状态示例
#[derive(Debug, Default, Clone)]
pub struct DisabledState {}

impl ToElement for DisabledState {
    fn to_element(&self) -> Element {
        let radio1 = use_signal(|| RadioValue::Int(0));
        let radio2 = use_signal(|| RadioValue::Int(0));
        let radio3 = use_signal(|| RadioValue::Int(0));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // 一个禁用，一个不禁用
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("混合状态: "))
                    .children(
                        Radio::new()
                            .value(1)
                            .disabled(true)
                            .label("禁用选项")
                            .checked_value(radio1),
                    )
                    .children(
                        Radio::new()
                            .value(2)
                            .label("可用选项")
                            .checked_value(radio1),
                    ),
            )
            // 全部禁用
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("全部禁用: "))
                    .children(
                        Radio::new()
                            .value(1)
                            .disabled(true)
                            .label("选项 A")
                            .checked_value(radio2),
                    )
                    .children(
                        Radio::new()
                            .value(2)
                            .disabled(true)
                            .label("选项 B")
                            .checked_value(radio2),
                    ),
            )
            // 单选组禁用
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("单选组禁用: "))
                    .children(
                        RadioGroup::new()
                            .value(radio3)
                            .disabled(true)
                            .radio(Radio::new().value(3).label("Option A"))
                            .radio(Radio::new().value(6).label("Option B"))
                            .radio(Radio::new().value(9).label("Option C")),
                    ),
            )
            .into()
    }
}

/// 单选框组示例
#[derive(Debug, Default, Clone)]
pub struct RadioGroupExample {}

impl ToElement for RadioGroupExample {
    fn to_element(&self) -> Element {
        let mut radio1 = use_signal(|| RadioValue::Int(3));
        let radio2 = use_signal(|| RadioValue::Int(1));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // 正常的单选框组
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("选择: "))
                    .children(
                        RadioGroup::new()
                            .value(radio1)
                            .onchange(move |v| {
                                println!("Radio changed: {:?}", v);
                                radio1.set(v)
                            }) // 可选，即切换时的回调函数，单选框组内置有状态管理
                            .radio(Radio::new().value(3).label("Option A"))
                            .radio(Radio::new().value(6).label("Option B"))
                            .radio(Radio::new().value(9).label("Option C")),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.margin_top("8px"))
                    .children(Text::new(format!("当前值: {:?}", radio1.read()))),
            )
            // 带有禁用选项的单选框组
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("8px"))
                    .children(Text::new("部分禁用: "))
                    .children(RadioGroup::new().value(radio2).radios(vec![
                        Radio::new().value(1).label("选项 A"),
                        Radio::new().value(2).disabled(true).label("禁用选项"),
                        Radio::new().value(3).label("选项 C"),
                    ])),
            )
            .children(
                View::new()
                    .style(|s| s.margin_top("8px"))
                    .children(Text::new(format!("当前值: {:?}", radio2.read()))),
            )
            .into()
    }
}

/// 带有边框示例
#[derive(Debug, Default, Clone)]
pub struct WithBorder {}

impl ToElement for WithBorder {
    fn to_element(&self) -> Element {
        let mut radio1 = use_signal(|| RadioValue::Int(1));
        let mut radio2 = use_signal(|| RadioValue::Int(1));
        let mut radio3 = use_signal(|| RadioValue::Int(1));
        let mut radio4 = use_signal(|| RadioValue::Int(1));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("20px")
            })
            .children(
                RadioGroup::new()
                    .value(radio1)
                    .onchange(move |v| radio1.set(v))
                    .radio(Radio::new().value(1).label("Option A"))
                    .radio(Radio::new().value(2).label("Option B"))
                    .border(true)
                    .as_large(),
            )
            .children(
                RadioGroup::new()
                    .value(radio2)
                    .onchange(move |v| radio2.set(v))
                    .radio(Radio::new().value(1).label("Option A"))
                    .radio(Radio::new().value(2).label("Option B"))
                    .border(true)
                    .as_medium(),
            )
            .children(
                RadioGroup::new()
                    .value(radio3)
                    .onchange(move |v| radio3.set(v))
                    .radio(Radio::new().value(1).label("Option A"))
                    .radio(Radio::new().value(2).disabled(true).label("Option B"))
                    .border(true)
                    .as_small(),
            )
            .children(
                RadioGroup::new()
                    .value(radio4)
                    .disabled(true)
                    .onchange(move |v| radio4.set(v))
                    .radio(Radio::new().value(1).label("Option A"))
                    .radio(Radio::new().value(2).label("Option B"))
                    .disabled(true)
                    .border(true)
                    .as_small(),
            )
            .into()
    }
}

/// 单选按钮示例
#[derive(Debug, Default, Clone)]
pub struct RadioButtonExample {}

impl ToElement for RadioButtonExample {
    fn to_element(&self) -> Element {
        let mut radio1 = use_signal(|| RadioValue::String("New York".to_string()));
        let mut radio2 = use_signal(|| RadioValue::String("New York".to_string()));
        let mut radio3 = use_signal(|| RadioValue::String("New York".to_string()));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("20px")
            })
            .children(
                RadioGroup::new()
                    .value(radio1)
                    .onchange(move |v| radio1.set(v))
                    .radio(Radio::new().value("New York").label("New York"))
                    .radio(Radio::new().value("Washington").label("Washington"))
                    .radio(Radio::new().value("Los Angeles").label("Los Angeles"))
                    .radio(Radio::new().value("Chicago").button(true).label("Chicago"))
                    .button(true),
            )
            .children(
                RadioGroup::new()
                    .value(radio2)
                    .onchange(move |v| radio2.set(v))
                    .radio(Radio::new().value("New York").label("New York"))
                    .radio(Radio::new().value("Washington").label("Washington"))
                    .radio(Radio::new().value("Los Angeles").label("Los Angeles"))
                    .radio(Radio::new().value("Chicago").button(true).label("Chicago"))
                    .button(true),
            )
            .children(
                RadioGroup::new()
                    .value(radio3)
                    .onchange(move |v| radio3.set(v))
                    .radio(Radio::new().value("New York").label("New York"))
                    .radio(
                        Radio::new()
                            .value("Washington")
                            .disabled(true)
                            .label("Washington"),
                    )
                    .radio(Radio::new().value("Los Angeles").label("Los Angeles"))
                    .radio(Radio::new().value("Chicago").button(true).label("Chicago"))
                    .button(true),
            )
            .into()
    }
}

/// 不同尺寸示例
#[derive(Debug, Default, Clone)]
pub struct DifferentSizes {}

impl ToElement for DifferentSizes {
    fn to_element(&self) -> Element {
        let radio1 = use_signal(|| RadioValue::Int(1));
        let radio2 = use_signal(|| RadioValue::Int(1));
        let radio3 = use_signal(|| RadioValue::Int(1));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("20px")
            })
            .children(Text::h4("小尺寸: "))
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("默认样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio1)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .as_small(),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("边框样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio1)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .border(true)
                            .as_small(),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("边按钮样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio1)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .button(true)
                            .as_small(),
                    ),
            )
            // 中等尺寸
            .children(Text::h4("中等尺寸: "))
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("默认样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio2)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .as_medium(),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("边框样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio2)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .border(true)
                            .as_medium(),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("边按钮样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio2)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .button(true)
                            .as_medium(),
                    ),
            )
            // 大尺寸
            .children(Text::h4("大尺寸: "))
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("默认样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio3)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .as_large(),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("边框样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio3)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .border(true)
                            .as_large(),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_left("80px")
                            .display("flex")
                            .align_items("center")
                            .gap("8px")
                    })
                    .children(Text::new("边按钮样式: "))
                    .children(
                        RadioGroup::new()
                            .value(radio3)
                            .radios(vec![
                                Radio::new().value(1).label("选项 A"),
                                Radio::new().value(2).label("选项 B"),
                            ])
                            .button(true)
                            .as_large(),
                    ),
            )
            .into()
    }
}
