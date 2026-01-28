//! Input 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Input, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct InputView {}

impl ToElement for InputView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl InputView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("Input 组件"),
            Text::p("单行文本输入框组件，支持基础用法、禁用状态、一键清空、密码框、不同尺寸和输入长度限制等功能。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_usage(),
            self.disabled_state(),
            self.clearable(),
            self.password(),
            self.size_control(),
            self.length_limit(),
            self.prefix_suffix(),
            self.prepend_append(),
            self.events_example(),
        ])
    }

    /// 基础用法
    fn basic_usage(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础用法"),
                Text::p("基本的文本输入框，可使用 v-model 双向绑定。"),
            ]))
            .children(BasicUsage::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用状态
    fn disabled_state(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("禁用状态"),
                Text::p("禁用状态的输入框，不可编辑。"),
            ]))
            .children(DisabledState::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 一键清空
    fn clearable(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("一键清空"),
                Text::p("通过 clearable 属性设置可清空的输入框。"),
            ]))
            .children(Clearable::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 密码框
    fn password(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("密码框"),
                Text::p("通过设置 input_type 为 Password 来创建密码输入框。"),
            ]))
            .children(Password::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 不同尺寸
    fn size_control(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("不同尺寸"),
                Text::p("提供小、中、大三种尺寸的输入框。"),
            ]))
            .children(SizeControl::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 输入长度限制
    fn length_limit(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("输入长度限制"),
                Text::p("通过 max_length 和 show_word_limit 设置输入长度限制和字数统计。"),
            ]))
            .children(LengthLimit::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 前置和后置图标
    fn prefix_suffix(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("前置和后置图标"),
                Text::p("通过 prefix_icon 和 suffix_icon 设置前置和后置图标。"),
            ]))
            .children(PrefixSuffix::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 前置和后置元素
    fn prepend_append(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("前置和后置元素"),
                Text::p("通过 prepend 和 append 设置前置和后置元素。"),
            ]))
            .children(PrependAppend::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 事件示例
    fn events_example(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("事件示例"),
                Text::p("演示各种事件的触发时机。"),
            ]))
            .children(EventsExample::default())
            .style(|s| s.margin_top("32px"))
    }
}

/// 基础用法示例
#[derive(Debug, Default, Clone)]
pub struct BasicUsage {}

impl ToElement for BasicUsage {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| String::from("默认值"));
        let mut value2 = use_signal(|| String::new());

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
                    .children(Text::new("默认值:"))
                    .children(
                        Input::new()
                            .value(value1)
                            .placeholder("请输入内容")
                            .oninput(move |v| value1.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("空值:"))
                    .children(
                        Input::new()
                            .value(value2)
                            .placeholder("请输入内容")
                            .oninput(move |v| value2.set(v)),
                    ),
            )
            .into()
    }
}

/// 禁用状态示例
#[derive(Debug, Default, Clone)]
pub struct DisabledState {}

impl ToElement for DisabledState {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| String::from("禁用状态"));
        let mut value2 = use_signal(|| String::new());

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
                    .children(Text::new("启用状态:"))
                    .children(
                        Input::new()
                            .value(value1)
                            .placeholder("请输入内容")
                            .oninput(move |v| value1.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("禁用状态:"))
                    .children(
                        Input::new()
                            .value(value2)
                            .placeholder("禁用的输入框")
                            .disabled(true)
                            .oninput(move |v| value2.set(v)),
                    ),
            )
            .into()
    }
}

/// 一键清空示例
#[derive(Debug, Default, Clone)]
pub struct Clearable {}

impl ToElement for Clearable {
    fn to_element(&self) -> Element {
        let mut value1 = use_signal(|| String::from("可清空的内容"));
        let mut value2 = use_signal(|| String::new());

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
                    .children(Text::new("有内容:"))
                    .children(
                        Input::new()
                            .value(value1)
                            .clearable(true)
                            .placeholder("请输入内容")
                            .onclear(move |_| value1.set(String::new()))
                            .oninput(move |v| value1.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("空值:"))
                    .children(
                        Input::new()
                            .value(value2)
                            .clearable(true)
                            .placeholder("请输入内容")
                            .onclear(move |_| value2.set(String::new()))
                            .oninput(move |v| value2.set(v)),
                    ),
            )
            .into()
    }
}

/// 密码框示例
#[derive(Debug, Default, Clone)]
pub struct Password {}

impl ToElement for Password {
    fn to_element(&self) -> Element {
        let mut password1 = use_signal(|| String::new());
        let mut password2 = use_signal(|| String::new());

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
                    .children(Text::new("密码:"))
                    .children(
                        Input::new()
                            .value(password1)
                            .as_password()
                            .placeholder("请输入密码")
                            .clearable(true)
                            .onclear(move |_| password1.set(String::new()))
                            .oninput(move |v| password1.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("确认密码:"))
                    .children(
                        Input::new()
                            .value(password2)
                            .as_password()
                            .placeholder("请再次输入密码")
                            .clearable(true)
                            .onclear(move |_| password2.set(String::new()))
                            .oninput(move |v| password2.set(v)),
                    ),
            )
            .into()
    }
}

/// 不同尺寸示例
#[derive(Debug, Default, Clone)]
pub struct SizeControl {}

impl ToElement for SizeControl {
    fn to_element(&self) -> Element {
        let mut small = use_signal(|| String::new());
        let mut medium = use_signal(|| String::new());
        let mut large = use_signal(|| String::new());

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
                    .children(Text::new("小尺寸:"))
                    .children(
                        Input::new()
                            .value(small)
                            .as_small()
                            .placeholder("小尺寸输入框")
                            .oninput(move |v| small.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("中等尺寸:"))
                    .children(
                        Input::new()
                            .value(medium)
                            .as_medium()
                            .placeholder("中等尺寸输入框")
                            .oninput(move |v| medium.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("大尺寸:"))
                    .children(
                        Input::new()
                            .value(large)
                            .as_large()
                            .placeholder("大尺寸输入框")
                            .oninput(move |v| large.set(v)),
                    ),
            )
            .into()
    }
}

/// 输入长度限制示例
#[derive(Debug, Default, Clone)]
pub struct LengthLimit {}

impl ToElement for LengthLimit {
    fn to_element(&self) -> Element {
        let mut username = use_signal(|| String::new());

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
                    .children(Text::new("用户名:"))
                    .children(
                        Input::new()
                            .value(username)
                            .max_length(20)
                            .show_word_limit(true)
                            .placeholder("请输入用户名（最多20字符）")
                            .oninput(move |v| username.set(v)),
                    ),
            )
            .into()
    }
}

/// 前置和后置图标示例
#[derive(Debug, Default, Clone)]
pub struct PrefixSuffix {}

impl ToElement for PrefixSuffix {
    fn to_element(&self) -> Element {
        let mut search = use_signal(|| String::new());
        let mut date = use_signal(|| String::new());

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
                    .children(Text::new("搜索:"))
                    .children(
                        Input::new()
                            .value(search)
                            .prefix_icon("🔍")
                            .placeholder("请输入搜索内容")
                            .oninput(move |v| search.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("日期:"))
                    .children(
                        Input::new()
                            .value(date)
                            .prefix_icon("📅")
                            .suffix_icon("📌")
                            .placeholder("请选择日期")
                            .oninput(move |v| date.set(v)),
                    ),
            )
            .into()
    }
}

/// 前置和后置元素示例
#[derive(Debug, Default, Clone)]
pub struct PrependAppend {}

impl ToElement for PrependAppend {
    fn to_element(&self) -> Element {
        let mut amount = use_signal(|| String::new());
        let mut website = use_signal(|| String::new());

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
                    .children(Text::new("金额:"))
                    .children(
                        Input::new()
                            .value(amount)
                            .prepend(std::rc::Rc::new(View::new().children(Text::new("￥"))))
                            .suffix_icon("元")
                            .placeholder("请输入金额")
                            .oninput(move |v| amount.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center").gap("12px"))
                    .children(Text::new("网站:"))
                    .children(
                        Input::new()
                            .value(website)
                            .prepend(std::rc::Rc::new(View::new().children(Text::new("http://"))))
                            .append(std::rc::Rc::new(View::new().children(Text::new(".com"))))
                            .placeholder("请输入网站名称")
                            .oninput(move |v| website.set(v)),
                    ),
            )
            .into()
    }
}

/// 事件示例
#[derive(Debug, Default, Clone)]
pub struct EventsExample {}

impl ToElement for EventsExample {
    fn to_element(&self) -> Element {
        let mut input_value = use_signal(|| String::new());
        let events = use_signal(|| Vec::new());

        let mut events_clone = events;

        View::new()
            .style(|s| s.padding("20px"))
            .children(Text::h3("事件日志:").style(|s| s.margin_bottom("12px")))
            .children(
                Input::new()
                    .value(input_value)
                    .placeholder("尝试输入、点击、按键盘等操作")
                    .clearable(true)
                    .oninput(move |v| input_value.set(v))
                    .onfocus(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "focus - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .onblur(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "blur - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .onclear(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "clear - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .onkeydown(move |event| {
                        let key = event.key();
                        let mut events = events_clone.write();
                        events.push(format!(
                            "keydown: {} - {}",
                            key,
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .onmouseenter(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "mouseenter - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .onmouseleave(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "mouseleave - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .oncompositionstart(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "compositionstart - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .oncompositionupdate(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "compositionupdate - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    })
                    .oncompositionend(move |_| {
                        let mut events = events_clone.write();
                        events.push(format!(
                            "compositionend - {}",
                            chrono::Local::now().format("%H:%M:%S%.3f")
                        ));
                        events.truncate(10);
                    }),
            )
            .children(
                View::new()
                    .style(|s| {
                        s.margin_top("16px")
                            .padding("12px")
                            .background_color("#f5f7fa")
                            .border_radius("4px")
                            .min_height("200px")
                            .font_size("12px")
                            .color("#606266")
                    })
                    .children(Text::new(if events.read().is_empty() {
                        "暂无事件记录，请在上方输入框中操作...".to_string()
                    } else {
                        events.read().join("\n")
                    })),
            )
            .into()
    }
}
