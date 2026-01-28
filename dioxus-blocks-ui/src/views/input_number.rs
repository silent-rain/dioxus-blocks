//! InputNumber 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, InputNumber, InputNumberValue, Text, ToElement, View};
use dioxus_blocks_macro::Route;
use rust_decimal::{Decimal, prelude::FromPrimitive};

#[derive(Debug, Default, Clone, Route)]
pub struct InputNumberView {}

impl ToElement for InputNumberView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl InputNumberView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("InputNumber 组件"),
            Text::p("数字输入框组件，支持精度控制、步进、不同尺寸和禁用状态。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_usage(),
            self.disabled_state(),
            self.precision_control(),
            self.step_control(),
            self.size_control(),
            self.min_max_values(),
            self.placeholder_example(),
            self.counter_example(),
            self.component_linkage(),
        ])
    }

    /// 基础用法
    fn basic_usage(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础用法"),
                Text::p("基本数字输入框，默认步进为 1。"),
            ]))
            .children(BasicUsage::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用状态
    fn disabled_state(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("禁用状态"),
                Text::p("禁用状态的数字输入框，不可编辑。"),
            ]))
            .children(DisabledState::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 精度控制
    fn precision_control(&self) -> Card {
        Card::new()
            .header(
                View::new().childrens2(vec![Text::h2("精度控制"), Text::p("控制显示的小数位数。")]),
            )
            .children(PrecisionControl::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 步进设置
    fn step_control(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("步进设置"),
                Text::p("设置每次增加或减少的步进值。"),
            ]))
            .children(StepControl::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 不同尺寸
    fn size_control(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("不同尺寸"),
                Text::p("不同大小的数字输入框。"),
            ]))
            .children(SizeControl::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 最小值和最大值
    fn min_max_values(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("最小值和最大值"),
                Text::p("设置输入范围的最小值和最大值。"),
            ]))
            .children(MinMaxValues::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 占位符示例
    fn placeholder_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("占位符"),
                Text::p("设置输入框的占位符文本。"),
            ]))
            .children(PlaceholderExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 计数器示例 - 实时显示值
    fn counter_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("实时显示值"),
                Text::p("通过 onchange 事件实时获取并显示当前值。"),
            ]))
            .children(CounterExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 组件联动示例
    fn component_linkage(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("组件联动"),
                Text::p("多个 InputNumber 组件与其他组件之间的数据联动。"),
            ]))
            .children(ComponentLinkage::default())
            .style(|s| s.margin_top("32px"))
    }
}

/// 基础用法示例
#[derive(Debug, Default, Clone)]
pub struct BasicUsage {}

impl ToElement for BasicUsage {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Int(1));
        let mut value2 = use_signal(|| InputNumberValue::Int(10));
        let mut value3 = use_signal(|| InputNumberValue::Int(100));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .onchange(move |v| value2.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value3)
                    .onchange(move |v| value3.set(v)),
            )
            .into()
    }
}

/// 禁用状态示例
#[derive(Debug, Default, Clone)]
pub struct DisabledState {}

impl ToElement for DisabledState {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Int(1));
        let mut value2 = use_signal(|| InputNumberValue::Int(10));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .disabled(true)
                    .onchange(move |v| value2.set(v)),
            )
            .into()
    }
}

/// 精度控制示例
#[derive(Debug, Default, Clone)]
pub struct PrecisionControl {}

impl ToElement for PrecisionControl {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Float(Decimal::from(1)));
        let mut value2 = use_signal(|| InputNumberValue::Float(Decimal::try_from(1.5).unwrap()));
        let mut value3 = use_signal(|| Decimal::from_f64(3.12).unwrap().into());
        let mut value4 = use_signal(|| 2.74.into());

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .precision(0)
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .precision(1)
                    .onchange(move |v| value2.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value3)
                    .precision(2)
                    .onchange(move |v| value3.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value4)
                    .precision(4)
                    .onchange(move |v| value4.set(v)),
            )
            .into()
    }
}

/// 步进设置示例
#[derive(Debug, Default, Clone)]
pub struct StepControl {}

impl ToElement for StepControl {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Float(Decimal::from(10)));
        let mut value2 = use_signal(|| InputNumberValue::Float(Decimal::from(10)));
        let mut value3 = use_signal(|| InputNumberValue::Float(Decimal::from(10)));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .step_float(1.0)
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .step_float(5.5)
                    .onchange(move |v| value2.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value3)
                    .step_float(10.0)
                    .onchange(move |v| value3.set(v)),
            )
            .into()
    }
}

/// 不同尺寸示例
#[derive(Debug, Default, Clone)]
pub struct SizeControl {}

impl ToElement for SizeControl {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Int(1));
        let mut value2 = use_signal(|| InputNumberValue::Int(2));
        let mut value3 = use_signal(|| InputNumberValue::Int(3));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .as_small()
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .as_medium()
                    .onchange(move |v| value2.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value3)
                    .as_large()
                    .onchange(move |v| value3.set(v)),
            )
            .into()
    }
}

/// 最小值和最大值示例
#[derive(Debug, Default, Clone)]
pub struct MinMaxValues {}

impl ToElement for MinMaxValues {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Int(50));
        let mut value2 = use_signal(|| InputNumberValue::Int(5));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .min_int(0)
                    .max_int(100)
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .min_int(0)
                    .max_int(10)
                    .onchange(move |v| value2.set(v)),
            )
            .into()
    }
}

/// 占位符示例
#[derive(Debug, Default, Clone)]
pub struct PlaceholderExample {}

impl ToElement for PlaceholderExample {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| InputNumberValue::Int(0));
        let mut value2 = use_signal(|| InputNumberValue::Int(0));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                InputNumber::new()
                    .value(value1)
                    .placeholder("请输入数字")
                    .onchange(move |v| value1.set(v)),
            )
            .children(
                InputNumber::new()
                    .value(value2)
                    .placeholder("数量")
                    .min_int(0)
                    .onchange(move |v| value2.set(v)),
            )
            .into()
    }
}

/// 计数器示例 - 实时显示值
#[derive(Debug, Default, Clone)]
pub struct CounterExample {}

impl ToElement for CounterExample {
    fn to_element(&self) -> Element {
        let mut count = use_signal(|| InputNumberValue::Int(0));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .gap("16px")
                    .align_items("center")
            })
            .children(
                Text::new(format!("当前值: {}", count()))
                    .style(|s| s.font_size("16px").color("#303133")),
            )
            .children(
                InputNumber::new()
                    .value(count)
                    .min_int(0)
                    .max_int(100)
                    .step_int(1)
                    .onchange(move |v| count.set(v)),
            )
            .into()
    }
}

/// 组件联动示例
#[derive(Debug, Default, Clone)]
pub struct ComponentLinkage {}

impl ToElement for ComponentLinkage {
    fn to_element(&self) -> Element {
        let mut width = use_signal(|| InputNumberValue::Int(200));
        let mut height = use_signal(|| InputNumberValue::Int(150));

        View::new()
            .style(|s| s.padding("20px"))
            .children(
                Text::h3("宽 x 高: {width} x {height}")
                    .style(|s| s.margin_bottom("16px").color("#409eff")),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.display("flex")
                            .gap("16px")
                            .align_items("center")
                            .margin_bottom("16px")
                    })
                    .children(Text::new("宽度: "))
                    .children(
                        InputNumber::new()
                            .value(width)
                            .min_int(100)
                            .max_int(500)
                            .step_int(10)
                            .onchange(move |v| width.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.display("flex")
                            .gap("16px")
                            .align_items("center")
                            .margin_bottom("16px")
                    })
                    .children(Text::new("高度: "))
                    .children(
                        InputNumber::new()
                            .value(height)
                            .min_int(100)
                            .max_int(500)
                            .step_int(10)
                            .onchange(move |v| height.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(move |s| {
                        s.width(format!("{}px", width.read().as_decimal()))
                            .height(format!("{}px", height.read().as_decimal()))
                            .background_color("#409eff")
                            .border_radius("8px")
                            .margin_top("16px")
                            .display("flex")
                            .align_items("center")
                            .justify_content("center")
                            .color("white")
                    })
                    .children(Text::new(format!("{} x {}", width.read(), height.read()))),
            )
            .into()
    }
}
