//! Checkbox 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{
    Card, Checkbox, CheckboxGroup, CheckboxValue, Text, ToElement, View,
};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct CheckboxView {}

impl ToElement for CheckboxView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl CheckboxView {
    fn title(&self) -> View {
        View::new().childrens(vec![
            Text::h1("Checkbox 多选框"),
            Text::p(
                "在一组备选项中进行多选，支持基础用法、禁用状态、多选框组、中间状态、可选项目数量限制、按钮样式和带有边框等功能。"
            ),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens(vec![
            self.basic_usage(),
            self.disabled_state(),
            self.checkbox_group(),
            self.indeterminate(),
            self.min_max(),
            self.button_style(),
            self.with_border(),
        ])
    }

    /// 基础用法
    fn basic_usage(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("基础用法"),
                Text::p(
                    "单独使用可以表示两种状态之间的切换，写在标签中的内容为 checkbox 按钮后的介绍。",
                ),
            ]))
            .children(BasicUsage::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用状态
    fn disabled_state(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("禁用状态"),
                Text::p("多选框不可用状态。设置 disabled 属性即可。"),
            ]))
            .children(DisabledState::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 多选框组
    fn checkbox_group(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("多选框组"),
                Text::p("适用于多个勾选框绑定到同一个数组的情景，通过是否勾选来表示这一组选项中选中的项。"),
            ]))
            .children(CheckboxGroupExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 中间状态
    fn indeterminate(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("中间状态"),
                Text::p(
                    "indeterminate 属性用以表示 checkbox 的不确定状态，一般用于实现全选的效果。",
                ),
            ]))
            .children(IndeterminateExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 可选项目数量的限制
    fn min_max(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("可选项目数量的限制"),
                Text::p("使用 min 和 max 属性能够限制可以被勾选的项目的数量。"),
            ]))
            .children(MinMaxExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 按钮样式
    fn button_style(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens(vec![Text::h2("按钮样式"), Text::p("按钮样式的多选组合。")]),
            )
            .children(ButtonStyleExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 带有边框
    fn with_border(&self) -> Card {
        Card::new()
            .header(View::new().childrens(vec![
                Text::h2("带有边框"),
                Text::p("设置 border 属性可以渲染为带有边框的多选框。"),
            ]))
            .children(WithBorder::default())
            .style(|s| s.margin_top("32px"))
    }
}

/// 基础用法示例
#[derive(Debug, Default, Clone)]
pub struct BasicUsage {}

impl ToElement for BasicUsage {
    fn to_element(&self) -> Element {
        let checked1 = use_signal(|| true);
        let checked2 = use_signal(|| false);
        let checked3 = use_signal(|| false);
        let checked4 = use_signal(|| false);
        let checked5 = use_signal(|| false);
        let checked6 = use_signal(|| false);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // 大尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("大尺寸: "))
                    .children(Checkbox::new().value(1).label("Option 1").checked(checked1))
                    .children(Checkbox::new().value(2).label("Option 2").checked(checked2)),
            )
            // 中等尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("中等尺寸: "))
                    .children(Checkbox::new().value(1).label("Option 1").checked(checked3))
                    .children(Checkbox::new().value(2).label("Option 2").checked(checked4)),
            )
            // 小尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("小尺寸: "))
                    .children(Checkbox::new().value(1).label("Option 1").checked(checked5))
                    .children(Checkbox::new().value(2).label("Option 2").checked(checked6)),
            )
            .into()
    }
}

/// 禁用状态示例
#[derive(Debug, Default, Clone)]
pub struct DisabledState {}

impl ToElement for DisabledState {
    fn to_element(&self) -> Element {
        let checked1 = use_signal(|| false);
        let _checked2 = use_signal(|| true);

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
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("混合状态: "))
                    .children(
                        Checkbox::new()
                            .value(1)
                            .disabled(true)
                            .label("Disabled")
                            .checked(checked1),
                    )
                    .children(Checkbox::new().value(2).label("Not disabled")),
            )
            // 全部禁用
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("全部禁用: "))
                    .children(Checkbox::new().value(1).disabled(true).label("Option A"))
                    .children(Checkbox::new().value(2).disabled(true).label("Option B")),
            )
            .into()
    }
}

/// 多选框组示例
#[derive(Debug, Default, Clone)]
pub struct CheckboxGroupExample {}

impl ToElement for CheckboxGroupExample {
    fn to_element(&self) -> Element {
        let mut checkbox1 = use_signal(|| {
            vec![
                CheckboxValue::String("Value selected and disabled".to_string()),
                CheckboxValue::String("Value A".to_string()),
            ]
        });
        let mut checkbox2 = use_signal(|| vec![CheckboxValue::String("Option A".to_string())]);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // 正常的多选框组
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("选择: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox1)
                            .onchange(move |v| {
                                println!("Checkbox changed: {:?}", v);
                                checkbox1.set(v)
                            })
                            .checkbox(Checkbox::new().value("Value A").label("Option A"))
                            .checkbox(Checkbox::new().value("Value B").label("Option B"))
                            .checkbox(Checkbox::new().value("Value C").label("Option C"))
                            .checkbox(
                                Checkbox::new()
                                    .value("Value disabled")
                                    .label("disabled")
                                    .disabled(true),
                            )
                            .checkbox(
                                Checkbox::new()
                                    .value("Value selected and disabled")
                                    .label("selected and disabled")
                                    .disabled(true),
                            ),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.margin_top("8px"))
                    .children(Text::new(format!("当前值: {:?}", checkbox1.read()))),
            )
            // 带有禁用选项的多选框组
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("部分禁用: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox2)
                            .onchange(move |v| checkbox2.set(v))
                            .checkbox(Checkbox::new().value("Option A").label("Option A"))
                            .checkbox(
                                Checkbox::new()
                                    .value("Option B")
                                    .label("Option B")
                                    .disabled(true),
                            )
                            .checkbox(Checkbox::new().value("Option C").label("Option C")),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.margin_top("8px"))
                    .children(Text::new(format!("当前值: {:?}", checkbox2.read()))),
            )
            .into()
    }
}

/// 中间状态示例
#[derive(Debug, Default, Clone)]
pub struct IndeterminateExample {}

impl ToElement for IndeterminateExample {
    fn to_element(&self) -> Element {
        let cities = vec![
            "Shanghai".to_string(),
            "Beijing".to_string(),
            "Guangzhou".to_string(),
            "Shenzhen".to_string(),
        ];
        let mut checked_cities = use_signal(|| {
            vec![
                CheckboxValue::String("Shanghai".to_string()),
                CheckboxValue::String("Beijing".to_string()),
            ]
        });
        let mut check_all = use_signal(|| false);
        let mut is_indeterminate = use_signal(|| true);

        let cities_clone = cities.clone();
        let _handle_check_all_change = move |_val: CheckboxValue| {
            let current_checked = checked_cities.read().clone();
            if current_checked.len() == cities_clone.len() {
                // 全部选中，取消全选
                checked_cities.set(vec![]);
                is_indeterminate.set(false);
            } else {
                // 全选
                checked_cities.set(
                    cities_clone
                        .iter()
                        .map(|s| CheckboxValue::String(s.clone()))
                        .collect(),
                );
                is_indeterminate.set(false);
            }
        };

        let handle_checked_cities_change = move |values: Vec<CheckboxValue>| {
            let checked_count = values.len();
            check_all.set(checked_count == cities.len());
            is_indeterminate.set(checked_count > 0 && checked_count < cities.len());
        };

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("16px")
            })
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(
                        Checkbox::new()
                            .value("all")
                            .label("Check all")
                            .indeterminate(*is_indeterminate.read()),
                    )
                    .children(
                        CheckboxGroup::new()
                            .value(checked_cities)
                            .onchange(handle_checked_cities_change),
                    ),
            )
            .into()
    }
}

/// 可选项目数量的限制示例
#[derive(Debug, Default, Clone)]
pub struct MinMaxExample {}

impl ToElement for MinMaxExample {
    fn to_element(&self) -> Element {
        let checked_cities = use_signal(|| {
            vec![
                CheckboxValue::String("Shanghai".to_string()),
                CheckboxValue::String("Beijing".to_string()),
            ]
        });

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .align_items("center")
                    .gap("12px")
            })
            .children(
                CheckboxGroup::new()
                    .value(checked_cities)
                    .min(1)
                    .max(2)
                    .checkbox(Checkbox::new().value("Shanghai").label("Shanghai"))
                    .checkbox(Checkbox::new().value("Beijing").label("Beijing"))
                    .checkbox(Checkbox::new().value("Guangzhou").label("Guangzhou"))
                    .checkbox(Checkbox::new().value("Shenzhen").label("Shenzhen")),
            )
            .into()
    }
}

/// 按钮样式示例
#[derive(Debug, Default, Clone)]
pub struct ButtonStyleExample {}

impl ToElement for ButtonStyleExample {
    fn to_element(&self) -> Element {
        let checkbox_group1 = use_signal(|| vec![CheckboxValue::String("Shanghai".to_string())]);
        let checkbox_group2 = use_signal(|| vec![CheckboxValue::String("Shanghai".to_string())]);
        let checkbox_group3 = use_signal(|| vec![CheckboxValue::String("Shanghai".to_string())]);
        let checkbox_group4 = use_signal(|| vec![CheckboxValue::String("Shanghai".to_string())]);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // 大尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("大尺寸: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox_group1)
                            .size(dioxus_blocks_components::CheckboxSize::Large)
                            .checkbox(Checkbox::new().value("Shanghai").label("Shanghai"))
                            .checkbox(Checkbox::new().value("Beijing").label("Beijing"))
                            .checkbox(Checkbox::new().value("Guangzhou").label("Guangzhou"))
                            .checkbox(Checkbox::new().value("Shenzhen").label("Shenzhen")),
                    ),
            )
            // 中等尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("中等尺寸: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox_group2)
                            .checkbox(Checkbox::new().value("Shanghai").label("Shanghai"))
                            .checkbox(Checkbox::new().value("Beijing").label("Beijing"))
                            .checkbox(Checkbox::new().value("Guangzhou").label("Guangzhou"))
                            .checkbox(Checkbox::new().value("Shenzhen").label("Shenzhen")),
                    ),
            )
            // 小尺寸（带有禁用选项）
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("小尺寸: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox_group3)
                            .size(dioxus_blocks_components::CheckboxSize::Small)
                            .checkbox(Checkbox::new().value("Shanghai").label("Shanghai"))
                            .checkbox(
                                Checkbox::new()
                                    .value("Beijing")
                                    .label("Beijing")
                                    .disabled(true),
                            )
                            .checkbox(Checkbox::new().value("Guangzhou").label("Guangzhou"))
                            .checkbox(Checkbox::new().value("Shenzhen").label("Shenzhen")),
                    ),
            )
            // 全部禁用
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("全部禁用: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox_group4)
                            .size(dioxus_blocks_components::CheckboxSize::Small)
                            .disabled(true)
                            .checkbox(Checkbox::new().value("Shanghai").label("Shanghai"))
                            .checkbox(Checkbox::new().value("Beijing").label("Beijing"))
                            .checkbox(Checkbox::new().value("Guangzhou").label("Guangzhou"))
                            .checkbox(Checkbox::new().value("Shenzhen").label("Shenzhen")),
                    ),
            )
            .into()
    }
}

/// 带有边框示例
#[derive(Debug, Default, Clone)]
pub struct WithBorder {}

impl ToElement for WithBorder {
    fn to_element(&self) -> Element {
        let checked1 = use_signal(|| true);
        let checked2 = use_signal(|| false);
        let checked3 = use_signal(|| false);
        let checked4 = use_signal(|| true);

        let checkbox_group1 = use_signal(|| vec![CheckboxValue::String("Value1".to_string())]);
        let checkbox_group2 = use_signal(|| vec![CheckboxValue::String("Value1".to_string())]);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("24px")
            })
            // 大尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("大尺寸: "))
                    .children(
                        Checkbox::new()
                            .value(1)
                            .label("Option1")
                            .border(true)
                            .checked(checked1),
                    )
                    .children(
                        Checkbox::new()
                            .value(2)
                            .label("Option2")
                            .border(true)
                            .checked(checked2),
                    ),
            )
            // 中等尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("中等尺寸: "))
                    .children(
                        Checkbox::new()
                            .value(1)
                            .label("Option1")
                            .border(true)
                            .checked(checked3),
                    )
                    .children(
                        Checkbox::new()
                            .value(2)
                            .label("Option2")
                            .border(true)
                            .checked(checked4),
                    ),
            )
            // 小尺寸
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("小尺寸: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox_group1)
                            .size(dioxus_blocks_components::CheckboxSize::Small)
                            .checkbox(
                                Checkbox::new()
                                    .value("Value1")
                                    .label("Option1")
                                    .border(true),
                            )
                            .checkbox(
                                Checkbox::new()
                                    .value("Value2")
                                    .label("Option2")
                                    .border(true),
                            ),
                    ),
            )
            // 全部禁用
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("全部禁用: "))
                    .children(
                        CheckboxGroup::new()
                            .value(checkbox_group2)
                            .size(dioxus_blocks_components::CheckboxSize::Small)
                            .disabled(true)
                            .checkbox(
                                Checkbox::new()
                                    .value("Value1")
                                    .label("Option1")
                                    .border(true),
                            )
                            .checkbox(
                                Checkbox::new()
                                    .value("Value2")
                                    .label("Option2")
                                    .border(true),
                            ),
                    ),
            )
            .into()
    }
}
