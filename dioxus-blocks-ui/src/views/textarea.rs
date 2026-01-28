//! Textarea 组件使用示例

use dioxus::prelude::*;

use dioxus_blocks_components::{Card, Text, Textarea, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct TextareaView {}

impl ToElement for TextareaView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .into()
    }
}

impl TextareaView {
    fn title(&self) -> View {
        View::new().childrens2(vec![
            Text::h1("Textarea 组件"),
            Text::p("多行文本输入框组件，支持基础用法、禁用状态、自适应高度、不同尺寸和输入长度限制等功能。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens2(vec![
            self.basic_usage(),
            self.disabled_state(),
            self.rows_control(),
            self.autosize(),
            self.size_control(),
            self.length_limit(),
            self.events_example(),
        ])
    }

    /// 基础用法
    fn basic_usage(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("基础用法"),
                Text::p("基本的多行文本输入框，可使用 v-model 双向绑定。"),
            ]))
            .children(BasicUsage::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 禁用状态
    fn disabled_state(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("禁用状态"),
                Text::p("禁用状态的文本域，不可编辑。"),
            ]))
            .children(DisabledState::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 行数控制
    fn rows_control(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("行数控制"),
                Text::p("通过 rows 属性设置文本域的行数。"),
            ]))
            .children(RowsControl::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 自适应高度
    fn autosize(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("自适应高度"),
                Text::p(
                    "通过 autosize 属性启用自适应高度，可配合 min_rows 和 max_rows 设置高度范围。",
                ),
            ]))
            .children(AutosizeExample::default())
            .style(|s| s.margin_top("32px"))
    }

    /// 不同尺寸
    fn size_control(&self) -> Card {
        Card::new()
            .header(View::new().childrens2(vec![
                Text::h2("不同尺寸"),
                Text::p("提供小、中、大三种尺寸的文本域。"),
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
        let mut content = use_signal(|| String::from("这是一段多行文本\n支持换行输入"));

        View::new()
            .style(|s| s.padding("20px"))
            .children(Text::h3("请输入内容:").style(|s| s.margin_bottom("12px")))
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(
                        Textarea::new()
                            .value(content)
                            .placeholder("请输入多行文本内容")
                            .oninput(move |v| content.set(v)),
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
        let mut content1 = use_signal(|| String::from("可编辑的文本域"));
        let mut content2 = use_signal(|| String::from("禁用的文本域"));

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("16px")
            })
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("可编辑:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(content1)
                            .rows(3)
                            .placeholder("请输入内容")
                            .oninput(move |v| content1.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("禁用:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(content2)
                            .rows(3)
                            .disabled(true)
                            .placeholder("禁用的文本域")
                            .oninput(move |v| content2.set(v)),
                    ),
            )
            .into()
    }
}

/// 行数控制示例
#[derive(Debug, Default, Clone)]
pub struct RowsControl {}

impl ToElement for RowsControl {
    fn to_element(&self) -> Element {
        let mut rows2 = use_signal(String::new);
        let mut rows4 = use_signal(String::new);
        let mut rows6 = use_signal(String::new);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("16px")
            })
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("2 行:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(rows2)
                            .rows(2)
                            .placeholder("2 行文本域")
                            .oninput(move |v| rows2.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("4 行:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(rows4)
                            .rows(4)
                            .placeholder("4 行文本域")
                            .oninput(move |v| rows4.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("6 行:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(rows6)
                            .rows(6)
                            .placeholder("6 行文本域")
                            .oninput(move |v| rows6.set(v)),
                    ),
            )
            .into()
    }
}

/// 自适应高度示例
#[derive(Debug, Default, Clone)]
pub struct AutosizeExample {}

impl ToElement for AutosizeExample {
    fn to_element(&self) -> Element {
        let mut content = use_signal(String::new);

        View::new()
            .style(|s| s.padding("20px"))
            .children(Text::h3("自适应高度 (2-6 行):").style(|s| s.margin_bottom("12px")))
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(
                        Textarea::new()
                            .value(content)
                            .autosize(true)
                            .min_rows(2)
                            .max_rows(6)
                            .placeholder("请输入内容，高度会自动调整（2-6 行）")
                            .oninput(move |v| content.set(v)),
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
        let mut small = use_signal(String::new);
        let mut medium = use_signal(String::new);
        let mut large = use_signal(String::new);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("16px")
            })
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("小尺寸:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(small)
                            .as_small()
                            .rows(2)
                            .placeholder("小尺寸文本域")
                            .oninput(move |v| small.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("中等尺寸:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(medium)
                            .as_medium()
                            .rows(3)
                            .placeholder("中等尺寸文本域")
                            .oninput(move |v| medium.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("大尺寸:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(large)
                            .as_large()
                            .rows(4)
                            .placeholder("大尺寸文本域")
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
        let mut description = use_signal(String::new);
        let mut comment = use_signal(String::new);

        View::new()
            .style(|s| {
                s.padding("20px")
                    .display("flex")
                    .flex_direction("column")
                    .gap("16px")
            })
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("描述（最多50字）:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(description)
                            .max_length(50)
                            .show_word_limit(true)
                            .rows(3)
                            .placeholder("请输入描述")
                            .oninput(move |v| description.set(v)),
                    ),
            )
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(Text::new("评论（最多100字）:").style(|s| s.margin_bottom("8px")))
                    .children(
                        Textarea::new()
                            .value(comment)
                            .max_length(100)
                            .show_word_limit(true)
                            .rows(4)
                            .placeholder("请输入评论")
                            .oninput(move |v| comment.set(v)),
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
        let mut input_value = use_signal(String::new);
        let events = use_signal(Vec::new);

        let mut events_clone = events;

        View::new()
            .style(|s| s.padding("20px"))
            .children(Text::h3("事件日志:").style(|s| s.margin_bottom("12px")))
            .children(
                View::new()
                    .style(|s| s.width("100%").max_width("600px"))
                    .children(
                        Textarea::new()
                            .value(input_value)
                            .rows(3)
                            .placeholder("尝试输入、点击、按键盘等操作")
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
                    ),
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
                        "暂无事件记录，请在上方文本域中操作...".to_string()
                    } else {
                        events.read().join("\n")
                    })),
            )
            .into()
    }
}
