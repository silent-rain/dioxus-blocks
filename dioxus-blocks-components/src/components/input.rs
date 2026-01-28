//! Input 组件
//!
//! 提供一个功能完整的单行文本输入框组件，支持基础用法、禁用状态、一键清空、
//! 密码框、不同尺寸和输入长度限制等功能。
//!
//! # 组件模式
//!
//! Input 是一个**受控组件**，需要通过 Signal 传递值，并通过 onchange/oninput 回调更新状态。
//!
//! # 示例
//!
//! ## 基础使用
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| String::from("Hello"));
//!     Input::new()
//!         .value(value)
//!         .placeholder("请输入内容")
//!         .oninput(move |v| value.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 密码框
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut password = use_signal(|| String::new());
//!     Input::new()
//!         .value(password)
//!         .as_password()
//!         .placeholder("请输入密码")
//!         .clearable(true)
//!         .oninput(move |v| password.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 可清空的输入框
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut text = use_signal(|| String::from("可清空的内容"));
//!     Input::new()
//!         .value(text)
//!         .clearable(true)
//!         .onclear(move |_| text.set(String::new()))
//!         .oninput(move |v| text.set(v))
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
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut username = use_signal(|| String::new());
//!     Input::new()
//!         .value(username)
//!         .max_length(20)
//!         .show_word_limit(true)
//!         .placeholder("请输入用户名")
//!         .oninput(move |v| username.set(v))
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
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| String::new());
//!     Input::new()
//!         .value(value)
//!         .as_large()
//!         .placeholder("大尺寸输入框")
//!         .oninput(move |v| value.set(v))
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
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| String::from("禁用状态"));
//!     Input::new()
//!         .value(value)
//!         .disabled(true)
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 前置和后置图标
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Input, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| String::new());
//!     Input::new()
//!         .value(value)
//!         .prefix_icon("🔍")
//!         .suffix_icon("🔔")
//!         .placeholder("带图标的输入框")
//!         .oninput(move |v| value.set(v))
//!         .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```

use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 输入框类型枚举
///
/// 定义输入框的不同类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    /// 文本输入（默认）
    #[default]
    Text,
    /// 密码输入
    Password,
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Password => write!(f, "password"),
        }
    }
}

/// 输入框尺寸枚举
///
/// 定义输入框的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for InputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputSize::Medium => write!(f, ""),
            InputSize::Small => write!(f, "t-input--small"),
            InputSize::Large => write!(f, "t-input--large"),
        }
    }
}

/// 输入框组件结构体
///
/// 提供一个可自定义的单行文本输入框，支持多种输入类型、尺寸、禁用状态和事件处理。
/// 这是一个受控组件，必须通过 `Signal<String>` 传入值。
///
/// # 使用说明
///
/// - 必须通过 `.value(signal)` 传入 `Signal<String>`
/// - 通过 `.oninput(handler)` 或 `.onchange(handler)` 响应值的变化
/// - 可通过 `.clearable(true)` 启用一键清空功能
/// - 可通过 `.as_password()` 切换为密码输入框
#[derive(Debug, Clone, ComponentBase)]
pub struct Input {
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
    /// 输入框类型
    input_type: InputType,
    /// 是否禁用
    disabled: bool,
    /// 输入框尺寸
    size: InputSize,
    /// 占位符
    placeholder: String,
    /// 是否可清空
    clearable: bool,
    /// 最大输入长度
    max_length: Option<usize>,
    /// 是否显示字数统计
    show_word_limit: bool,
    /// 前置图标
    prefix_icon: Option<String>,
    /// 后置图标
    suffix_icon: Option<String>,
    /// 前置元素
    prepend: Option<Rc<dyn ToElement>>,
    /// 后置元素
    append: Option<Rc<dyn ToElement>>,
    /// 输入事件（实时）
    oninput: Option<EventHandler<String>>,
    /// 值改变事件（失去焦点或按回车时触发）
    onchange: Option<EventHandler<String>>,
    /// 失去焦点事件
    onblur: Option<EventHandler<FocusEvent>>,
    /// 获得焦点事件
    onfocus: Option<EventHandler<FocusEvent>>,
    /// 清空事件
    onclear: Option<EventHandler<MouseEvent>>,
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

impl Default for Input {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-input".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            value: None,
            input_type: InputType::default(),
            disabled: false,
            size: InputSize::default(),
            placeholder: String::new(),
            clearable: false,
            max_length: None,
            show_word_limit: false,
            prefix_icon: None,
            suffix_icon: None,
            prepend: None,
            append: None,
            oninput: None,
            onchange: None,
            onblur: None,
            onfocus: None,
            onclear: None,
            onkeydown: None,
            onmouseenter: None,
            onmouseleave: None,
            oncompositionstart: None,
            oncompositionupdate: None,
            oncompositionend: None,
        }
    }
}

impl Input {
    /// 创建一个新的输入框实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的输入框实例（需要通过 `.value()` 设置 Signal）
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

    /// 设置输入框类型
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    /// 设置为密码输入框
    pub fn as_password(mut self) -> Self {
        self.input_type = InputType::Password;
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置输入框尺寸
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// 设置占位符
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// 设置是否可清空
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
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

    /// 设置前置图标
    pub fn prefix_icon(mut self, icon: impl Into<String>) -> Self {
        self.prefix_icon = Some(icon.into());
        self
    }

    /// 设置后置图标
    pub fn suffix_icon(mut self, icon: impl Into<String>) -> Self {
        self.suffix_icon = Some(icon.into());
        self
    }

    /// 设置前置元素
    pub fn prepend(mut self, prepend: Rc<dyn ToElement>) -> Self {
        self.prepend = Some(prepend);
        self
    }

    /// 设置后置元素
    pub fn append(mut self, append: Rc<dyn ToElement>) -> Self {
        self.append = Some(append);
        self
    }

    /// 设置输入事件（实时触发）
    pub fn oninput(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.oninput = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件（失去焦点或按回车时触发）
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

    /// 设置清空事件
    pub fn onclear(mut self, handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.onclear = Some(EventHandler::new(handler));
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

    /// 设置为小尺寸输入框
    pub fn as_small(mut self) -> Self {
        self.size = InputSize::Small;
        self
    }

    /// 设置为中等尺寸输入框
    pub fn as_medium(mut self) -> Self {
        self.size = InputSize::Medium;
        self
    }

    /// 设置为大尺寸输入框
    pub fn as_large(mut self) -> Self {
        self.size = InputSize::Large;
        self
    }
}

impl ToElement for Input {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        let mut class_names = vec![self.class.clone(), self.size.to_string()];
        if self.disabled {
            class_names.push("t-input--disabled".to_string());
        }
        if self.clearable && self.value.as_ref().is_some_and(|v| !v.read().is_empty()) {
            class_names.push("t-input--clearable".to_string());
        }
        if self.prefix_icon.is_some() || self.prepend.is_some() {
            class_names.push("t-input--prefix".to_string());
        }
        if self.suffix_icon.is_some() || self.append.is_some() || self.clearable {
            class_names.push("t-input--suffix".to_string());
        }
        let class = class_names.join(" ");

        let style = self.style.clone().map(|s| s.to_string());
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        let input_type_str = self.input_type.to_string();
        let max_length_attr = self.max_length.map(|l| l.to_string());

        // 获取 value signal，如果未设置则使用默认值
        let mut value_signal = self.value.unwrap_or_else(|| Signal::new(String::new()));

        let oninput_handler = self.oninput;
        let onchange_handler = self.onchange;
        let onblur_handler = self.onblur;
        let onfocus_handler = self.onfocus;
        let onclear_handler = self.onclear;
        let onkeydown_handler = self.onkeydown;
        let onmouseenter_handler = self.onmouseenter;
        let onmouseleave_handler = self.onmouseleave;
        let oncompositionstart_handler = self.oncompositionstart;
        let oncompositionupdate_handler = self.oncompositionupdate;
        let oncompositionend_handler = self.oncompositionend;

        let clearable = self.clearable;
        let show_word_limit = self.show_word_limit;
        let max_length = self.max_length;

        let prefix_icon = self.prefix_icon.clone();
        let suffix_icon = self.suffix_icon.clone();
        let prepend = self.prepend.clone();
        let append = self.append.clone();

        rsx! {
            div { id, class, style,
                // 前置元素
                if let Some(prepend_el) = &prepend {
                    div { class: "t-input__prepend", {prepend_el.to_element()} }
                }

                div { class: "t-input__wrapper",
                    // 前置图标
                    if let Some(icon) = prefix_icon {
                        span { class: "t-input__prefix",
                            span {
                                class: "t-input__icon",
                                dangerous_inner_html: "{icon}",
                            }
                        }
                    }

                    // 输入框
                    input {
                        r#type: input_type_str,
                        class: "t-input__inner",
                        placeholder,
                        disabled,
                        maxlength: max_length_attr,
                        value: value_signal.read().clone(),
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

                    // 后置图标（清空按钮 + 自定义图标）
                    if suffix_icon.is_some() || clearable || show_word_limit {
                        span { class: "t-input__suffix",
                            // 清空按钮
                            if clearable && !value_signal.read().is_empty() && !disabled {
                                span {
                                    class: "t-input__clear",
                                    onclick: move |event: MouseEvent| {
                                        event.stop_propagation();
                                        value_signal.set(String::new());
                                        if let Some(handler) = onclear_handler {
                                            handler.call(event);
                                        }
                                    },
                                    "×"
                                }
                            }

                            // 自定义后置图标
                            if let Some(icon) = suffix_icon {
                                span {
                                    class: "t-input__icon",
                                    dangerous_inner_html: "{icon}",
                                }
                            }

                            // 字数统计
                            if show_word_limit {
                                span { class: "t-input__count",
                                    "{value_signal.read().chars().count()}"
                                    if let Some(max_len) = max_length {
                                        span { class: "t-input__count-separator", "/" }
                                        span { "{max_len}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // 后置元素
                if let Some(append_el) = &append {
                    div { class: "t-input__append", {append_el.to_element()} }
                }
            }
        }
    }
}
