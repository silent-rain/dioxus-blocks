//! Select 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Select, SelectOption, SelectSize, SelectValue, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct SelectView {}

impl ToElement for SelectView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl SelectView {
    fn title(&self) -> View {
        View::new().childrens(vec![
            Text::h1("Select 选择器"),
            Text::p("当选项过多时，使用下拉菜单展示并选择内容。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens(vec![
            self.basic_usage(),
            self.disabled_options(),
            self.disabled_state(),
            self.clearable(),
            self.different_sizes(),
            self.filterable(),
        ])
    }

    /// 基础用法
    fn basic_usage(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("基础用法"),
                Text::p("适用广泛的基础单选，支持 String、Int、Float、Bool 多种类型。"),
            ]))
            .children(BasicUsage::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 有禁用选项
    fn disabled_options(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("有禁用选项"),
                Text::p("在 `SelectOption` 中设置 `disabled` 属性来禁用该选项。"),
            ]))
            .children(DisabledOptions::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用状态
    fn disabled_state(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("禁用状态"),
                Text::p("选择器本身不可用。"),
            ]))
            .children(DisabledState::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 可清空
    fn clearable(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("可清空"),
                Text::p("包含清空按钮，可将选择器清空为初始状态。"),
            ]))
            .children(ClearableExample::default())
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

    /// 可筛选
    fn filterable(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("可筛选"),
                Text::p("利用搜索功能快速查找选项。"),
            ]))
            .children(Filterable::default())
            .style(|s| s.margin_top("32px"))
    }
}

/// 基础用法示例
#[derive(Debug, Default, Clone)]
pub struct BasicUsage {}

impl ToElement for BasicUsage {
    fn to_element(&self) -> Element {
        let mut value_string = use_signal(|| SelectValue::String("选项1".to_string()));
        let mut value_int = use_signal(|| SelectValue::Int(1));
        let mut value_float = use_signal(|| SelectValue::Float(1.5));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // String 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("String 类型: "))
                    .children(
                        Select::new()
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange(move |v| value_string.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value_string.read()
                    ))),
            )
            // Int 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("Int 类型: "))
                    .children(
                        Select::new()
                            .options(vec![
                                SelectOption::new(1),
                                SelectOption::new(2),
                                SelectOption::new(3),
                            ])
                            .onchange(move |v| value_int.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value_int.read()
                    ))),
            )
            // Float 类型
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("Float 类型: "))
                    .children(
                        Select::new()
                            .options(vec![
                                SelectOption::new(1.5),
                                SelectOption::new(2.5),
                                SelectOption::new(3.5),
                            ])
                            .onchange(move |v| value_float.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value_float.read()
                    ))),
            )
            .into()
    }
}

/// 有禁用选项示例
#[derive(Debug, Default, Clone)]
pub struct DisabledOptions {}

impl ToElement for DisabledOptions {
    fn to_element(&self) -> Element {
        let mut value = use_signal(|| SelectValue::String("选项1".to_string()));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("部分选项禁用: "))
                    .children(
                        Select::new()
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2").disabled(true),
                                SelectOption::new("选项3"),
                            ])
                            .onchange(move |v| value.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value.read()
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
        let mut value1 = use_signal(|| SelectValue::String("选项1".to_string()));
        let mut value2 = use_signal(|| SelectValue::String("选项1".to_string()));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("正常状态: "))
                    .children(
                        Select::new()
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange(move |v| value1.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value1.read()
                    ))),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("禁用状态: "))
                    .children(
                        Select::new()
                            .disabled(true)
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange(move |v| value2.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value2.read()
                    ))),
            )
            .into()
    }
}

/// 可清空示例
#[derive(Debug, Default, Clone)]
pub struct ClearableExample {}

impl ToElement for ClearableExample {
    fn to_element(&self) -> Element {
        let mut value = use_signal(|| SelectValue::String("选项1".to_string()));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("可清空: "))
                    .children(
                        Select::new()
                            .clearable(true)
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange(move |v| value.set(v))
                            .onclear(move |_| {
                                value.set(SelectValue::String(String::new()));
                            }),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value.read()
                    ))),
            )
            .into()
    }
}

/// 不同尺寸示例
#[derive(Debug, Default, Clone)]
pub struct DifferentSizes {}

impl ToElement for DifferentSizes {
    fn to_element(&self) -> Element {
        let mut value_small = use_signal(|| SelectValue::String("选项1".to_string()));
        let mut value_medium = use_signal(|| SelectValue::String("选项1".to_string()));
        let mut value_large = use_signal(|| SelectValue::String("选项1".to_string()));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("小尺寸: "))
                    .children(
                        Select::new()
                            .size(SelectSize::Small)
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange2(move |v| value_small.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value_small.read()
                    ))),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("中等尺寸: "))
                    .children(
                        Select::new()
                            .size(SelectSize::Medium)
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange2(move |v| value_medium.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value_medium.read()
                    ))),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("大尺寸: "))
                    .children(
                        Select::new()
                            .size(SelectSize::Large)
                            .options(vec![
                                SelectOption::new("选项1"),
                                SelectOption::new("选项2"),
                                SelectOption::new("选项3"),
                            ])
                            .onchange2(move |v| value_large.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value_large.read()
                    ))),
            )
            .into()
    }
}

/// 可筛选示例
#[derive(Debug, Default, Clone)]
pub struct Filterable {}

impl ToElement for Filterable {
    fn to_element(&self) -> Element {
        let mut value = use_signal(|| SelectValue::String("选项1".to_string()));

        let options = vec![
            SelectOption::new("选项1"),
            SelectOption::new("选项2"),
            SelectOption::new("选项3"),
            SelectOption::new("选项4"),
            SelectOption::new("选项5"),
            SelectOption::new("选项6"),
            SelectOption::new("选项7"),
            SelectOption::new("选项8"),
            SelectOption::new("选项9"),
            SelectOption::new("选项10"),
        ];

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("16px"))
                    .children(Text::new("可筛选: "))
                    .children(
                        Select::new()
                            .filterable(true)
                            .options(options)
                            .onchange(move |v| value.set(v)),
                    )
                    .children(Text::new(format!(
                        "{:?}",
                        value.read()
                    ))),
            )
            .into()
    }
}
