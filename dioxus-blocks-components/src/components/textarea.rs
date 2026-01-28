//! Textarea 组件
//!
//! 提供一个多行文本输入框组件，支持自适应高度、行数控制和输入长度限制等功能。
//!
//! # 组件模式
//!
//! Textarea 是一个**受控组件**，需要通过 Signal 传递值，并通过 oninput/onchange 回调更新状态。
//!
//! # 示例
//!
//! ## 基础使用
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Textarea, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut content = use_signal(|| String::from("多行文本内容"));
//!     Textarea::new()
//!         .value(content)
//!         .placeholder("请输入内容")
//!         .oninput(move |v| content.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 固定行数
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Textarea, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut content = use_signal(|| String::new());
//!     Textarea::new()
//!         .value(content)
//!         .rows(6)
//!         .placeholder("请输入内容")
//!         .oninput(move |v| content.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 自适应高度
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Textarea, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut content = use_signal(|| String::new());
//!     Textarea::new()
//!         .value(content)
//!         .autosize(true)
//!         .min_rows(2)
//!         .max_rows(6)
//!         .placeholder("请输入内容")
//!         .oninput(move |v| content.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 输入长度限制
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Textarea, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut bio = use_signal(|| String::new());
//!     Textarea::new()
//!         .value(bio)
//!         .max_length(100)
//!         .show_word_limit(true)
//!         .rows(4)
//!         .placeholder("请输入个人简介")
//!         .oninput(move |v| bio.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 不同尺寸
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Textarea, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut content = use_signal(|| String::new());
//!     Textarea::new()
//!         .value(content)
//!         .as_large()
//!         .rows(4)
//!         .placeholder("请输入内容")
//!         .oninput(move |v| content.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 禁用状态
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Textarea, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut content = use_signal(|| String::from("禁用的文本域"));
//!     Textarea::new()
//!         .value(content)
//!         .disabled(true)
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```

use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 文本域尺寸枚举
///
/// 定义文本域的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextareaSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for TextareaSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextareaSize::Medium => write!(f, ""),
            TextareaSize::Small => write!(f, "t-textarea--small"),
            TextareaSize::Large => write!(f, "t-textarea--large"),
        }
    }
}

/// 文本域组件结构体
///
/// 提供一个可自定义的多行文本输入框，支持自适应高度、行数控制和输入长度限制。
/// 这是一个受控组件，必须通过 `Signal<String>` 传入值。
///
/// # 使用说明
///
/// - 必须通过 `.value(signal)` 传入 `Signal<String>`
/// - 通过 `.oninput(handler)` 或 `.onchange(handler)` 响应值的变化
/// - 可通过 `.autosize(true)` 启用自适应高度
/// - 可通过 `.min_rows()` 和 `.max_rows()` 设置高度范围
#[derive(Debug, Clone, ComponentBase)]
pub struct Textarea {
    /// 组件的唯一标识符
    id: Option<String>,
    /// 组件的CSS类名
    class: String,
    /// 组件的内联样式
    style: Option<Style>,
    /// 组件的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 组件的点击事件
    onclick: Option<EventHandler<MouseEvent>>,

    /// 当前值的 Signal（受控状态）
    value: Option<Signal<String>>,
    /// 是否禁用
    disabled: bool,
    /// 文本域尺寸
    size: TextareaSize,
    /// 占位符
    placeholder: String,
    /// 是否自适应高度
    autosize: bool,
    /// 最小行数
    min_rows: Option<usize>,
    /// 最大行数
    max_rows: Option<usize>,
    /// 固定行数
    rows: Option<usize>,
    /// 最大输入长度
    max_length: Option<usize>,
    /// 是否显示字数统计
    show_word_limit: bool,
    /// 输入事件（实时）
    oninput: Option<EventHandler<String>>,
    /// 值改变事件（失去焦点时触发）
    onchange: Option<EventHandler<String>>,
    /// 失去焦点事件
    onblur: Option<EventHandler<FocusEvent>>,
    /// 获得焦点事件
    onfocus: Option<EventHandler<FocusEvent>>,
    /// 键盘按下事件
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// 鼠标移入事件
    onmouseenter: Option<EventHandler<MouseEvent>>,
    /// 鼠标移出事件
    onmouseleave: Option<EventHandler<MouseEvent>>,
    /// 输入法开始事件
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    /// 输入法更新事件
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    /// 输入法结束事件
    oncompositionend: Option<EventHandler<CompositionEvent>>,
}

impl Default for Textarea {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-textarea".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            value: None,
            disabled: false,
            size: TextareaSize::default(),
            placeholder: String::new(),
            autosize: false,
            min_rows: None,
            max_rows: None,
            rows: None,
            max_length: None,
            show_word_limit: false,
            oninput: None,
            onchange: None,
            onblur: None,
            onfocus: None,
            onkeydown: None,
            onmouseenter: None,
            onmouseleave: None,
            oncompositionstart: None,
            oncompositionupdate: None,
            oncompositionend: None,
        }
    }
}

impl Textarea {
    /// 创建一个新的文本域实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的文本域实例（需要通过 `.value()` 设置 Signal）
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置当前值的 Signal（必需）
    pub fn value(mut self, value: Signal<String>) -> Self {
        self.value = Some(value);
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置文本域尺寸
    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    /// 设置占位符
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// 设置是否自适应高度
    pub fn autosize(mut self, autosize: bool) -> Self {
        self.autosize = autosize;
        self
    }

    /// 设置最小行数
    pub fn min_rows(mut self, rows: usize) -> Self {
        self.min_rows = Some(rows);
        self
    }

    /// 设置最大行数
    pub fn max_rows(mut self, rows: usize) -> Self {
        self.max_rows = Some(rows);
        self
    }

    /// 设置固定行数
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    /// 设置最大输入长度
    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// 设置是否显示字数统计
    pub fn show_word_limit(mut self, show: bool) -> Self {
        self.show_word_limit = show;
        self
    }

    /// 设置输入事件（实时触发）
    pub fn oninput(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.oninput = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件（失去焦点时触发）
    pub fn onchange(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置失去焦点事件
    pub fn onblur(mut self, handler: impl FnMut(FocusEvent) + 'static) -> Self {
        self.onblur = Some(EventHandler::new(handler));
        self
    }

    /// 设置获得焦点事件
    pub fn onfocus(mut self, handler: impl FnMut(FocusEvent) + 'static) -> Self {
        self.onfocus = Some(EventHandler::new(handler));
        self
    }

    /// 设置键盘按下事件
    pub fn onkeydown(mut self, handler: impl FnMut(KeyboardEvent) + 'static) -> Self {
        self.onkeydown = Some(EventHandler::new(handler));
        self
    }

    /// 设置鼠标移入事件
    pub fn onmouseenter(mut self, handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.onmouseenter = Some(EventHandler::new(handler));
        self
    }

    /// 设置鼠标移出事件
    pub fn onmouseleave(mut self, handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.onmouseleave = Some(EventHandler::new(handler));
        self
    }

    /// 设置输入法开始事件
    pub fn oncompositionstart(mut self, handler: impl FnMut(CompositionEvent) + 'static) -> Self {
        self.oncompositionstart = Some(EventHandler::new(handler));
        self
    }

    /// 设置输入法更新事件
    pub fn oncompositionupdate(mut self, handler: impl FnMut(CompositionEvent) + 'static) -> Self {
        self.oncompositionupdate = Some(EventHandler::new(handler));
        self
    }

    /// 设置输入法结束事件
    pub fn oncompositionend(mut self, handler: impl FnMut(CompositionEvent) + 'static) -> Self {
        self.oncompositionend = Some(EventHandler::new(handler));
        self
    }

    /// 设置为小尺寸文本域
    pub fn as_small(mut self) -> Self {
        self.size = TextareaSize::Small;
        self
    }

    /// 设置为中等尺寸文本域
    pub fn as_medium(mut self) -> Self {
        self.size = TextareaSize::Medium;
        self
    }

    /// 设置为大尺寸文本域
    pub fn as_large(mut self) -> Self {
        self.size = TextareaSize::Large;
        self
    }
}

impl ToElement for Textarea {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        let mut class_names = vec![self.class.clone(), self.size.to_string()];
        if self.disabled {
            class_names.push("t-textarea--disabled".to_string());
        }
        if self.max_length.is_some() {
            class_names.push("t-textarea--limit".to_string());
        }
        if self.autosize {
            class_names.push("t-textarea--autosize".to_string());
        }
        let class = class_names.join(" ");

        let style = self.style.clone().map(|s| s.to_string());
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        let max_length_attr = self.max_length.map(|l| l.to_string());

        // 获取 value signal，如果未设置则使用默认值
        let mut value_signal = self.value.unwrap_or_else(|| Signal::new(String::new()));

        // 自适应高度相关属性
        let autosize = self.autosize;
        let min_rows = self.min_rows;
        let max_rows = self.max_rows;
        let rows = self.rows;

        // 确定最终使用的行数
        let rows_attr = if autosize {
            // 如果启用自适应高度，使用 min_rows 作为初始值
            min_rows.or(Some(2)).map(|r| r.to_string())
        } else {
            // 否则使用 rows
            rows.map(|r| r.to_string())
        };

        let oninput_handler = self.oninput;
        let onchange_handler = self.onchange;
        let onblur_handler = self.onblur;
        let onfocus_handler = self.onfocus;
        let onkeydown_handler = self.onkeydown;
        let onmouseenter_handler = self.onmouseenter;
        let onmouseleave_handler = self.onmouseleave;
        let oncompositionstart_handler = self.oncompositionstart;
        let oncompositionupdate_handler = self.oncompositionupdate;
        let oncompositionend_handler = self.oncompositionend;

        let show_word_limit = self.show_word_limit;
        let max_length = self.max_length;

        rsx! {
            div { id, class, style,
                // 字数统计
                if show_word_limit {
                    div { class: "t-textarea__count",
                        "{value_signal.read().chars().count()}"
                        if let Some(max_len) = max_length {
                            span { class: "t-textarea__count-separator", "/" }
                            span { "{max_len}" }
                        }
                    }
                }

                textarea {
                    class: "t-textarea__inner",
                    placeholder,
                    disabled,
                    rows: rows_attr,
                    maxlength: max_length_attr,
                    value: value_signal.read().clone(),
                    // 自适应高度样式
                    style: {
                        let mut styles = Vec::new();
                        if autosize {
                            if let Some(min) = min_rows {
                                styles.push(format!("min-height: calc({} * 1.5715em + 10px)", min));
                            }
                            if let Some(max) = max_rows {
                                styles.push(format!("max-height: calc({} * 1.5715em + 10px)", max));
                                styles.push("overflow-y: auto".to_string());
                            } else {
                                styles.push("overflow-y: hidden".to_string());
                            }
                            styles.push("resize: none".to_string());
                        }
                        styles.join("; ")
                    },
                    oninput: move |event: Event<FormData>| {
                        if disabled {
                            return;
                        }
                        let input_value = event.value();

                        if let Some(max_len) = max_length
                            && input_value.chars().count() > max_len {
                            return;
                        }

                        value_signal.set(input_value.clone());

                        if let Some(handler) = oninput_handler {
                            handler.call(input_value);
                        }
                    },
                    onchange: move |event: Event<FormData>| {
                        if disabled {
                            return;
                        }
                        let input_value = event.value();
                        value_signal.set(input_value.clone());

                        if let Some(handler) = onchange_handler {
                            handler.call(input_value);
                        }
                    },
                    onblur: move |event: FocusEvent| {
                        if let Some(handler) = onblur_handler {
                            handler.call(event);
                        }
                    },
                    onfocus: move |event: FocusEvent| {
                        if let Some(handler) = onfocus_handler {
                            handler.call(event);
                        }
                    },
                    onkeydown: move |event: KeyboardEvent| {
                        if let Some(handler) = onkeydown_handler {
                            handler.call(event);
                        }
                    },
                    onmouseenter: move |event: MouseEvent| {
                        if let Some(handler) = onmouseenter_handler {
                            handler.call(event);
                        }
                    },
                    onmouseleave: move |event: MouseEvent| {
                        if let Some(handler) = onmouseleave_handler {
                            handler.call(event);
                        }
                    },
                    oncompositionstart: move |event: CompositionEvent| {
                        if let Some(handler) = oncompositionstart_handler {
                            handler.call(event);
                        }
                    },
                    oncompositionupdate: move |event: CompositionEvent| {
                        if let Some(handler) = oncompositionupdate_handler {
                            handler.call(event);
                        }
                    },
                    oncompositionend: move |event: CompositionEvent| {
                        if let Some(handler) = oncompositionend_handler {
                            handler.call(event);
                        }
                    },
                }
            }
        }
    }
}
