//! Radio 单选框组件
//!
//! 提供单选框和单选框组组件，支持边框、按钮样式等功能。
//!
//! # 组件模式
//!
//! Radio 和 RadioGroup 是**受控组件**，需要通过 Signal 传递值，并通过 change 事件更新状态。
//!
//! # 示例
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Radio, RadioGroup};
//!
//! let mut radio = use_signal(|| 1);
//! rsx! {
//!     RadioGroup { value: radio, onchange: move |v| radio.set(v),
//!         Radio { value: 1, "Option 1" }
//!         Radio { value: 2, "Option 2" }
//!         Radio { value: 3, "Option 3" }
//!     }
//! }
//! ```
//!
//! ## 禁用状态
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Radio, RadioGroup};
//!
//! let mut radio = use_signal(|| 1);
//! rsx! {
//!     RadioGroup { value: radio, onchange: move |v| radio.set(v),
//!         Radio { value: 1, disabled: true, "Option A" }
//!         Radio { value: 2, disabled: true, "Option B" }
//!     }
//! }
//! ```
//!
//! ## 带有边框
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Radio, RadioGroup};
//!
//! let mut radio = use_signal(|| 1);
//! rsx! {
//!     RadioGroup { value: radio, onchange: move |v| radio.set(v),
//!         Radio { value: 1, border: true, "Option A" }
//!         Radio { value: 2, border: true, "Option B" }
//!     }
//! }
//! ```
//!
//! ## 单选按钮
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Radio, RadioGroup};
//!
//! let mut radio = use_signal(|| "New York");
//! rsx! {
//!     RadioGroup { value: radio, onchange: move |v| radio.set(v),
//!         Radio { value: "New York", button: true, "New York" }
//!         Radio { value: "Washington", button: true, "Washington" }
//!         Radio { value: "Los Angeles", button: true, "Los Angeles" }
//!     }
//! }
//! ```

use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, Text, traits::ToElement};

/// 单选框尺寸枚举
///
/// 定义单选框的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for RadioSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadioSize::Medium => write!(f, ""),
            RadioSize::Small => write!(f, "t-radio--small"),
            RadioSize::Large => write!(f, "t-radio--large"),
        }
    }
}

/// 单选框值枚举
///
/// 支持多种类型的值。
#[derive(Debug, Clone, PartialEq)]
pub enum RadioValue {
    /// 字符串类型
    String(String),
    /// 整数类型
    Int(i64),
    /// 浮点数类型
    Float(f64),
    /// 布尔类型
    Bool(bool),
}

impl RadioValue {
    /// 获取字符串
    pub fn get_string(&self) -> Option<&str> {
        match self {
            RadioValue::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    /// 获取整数
    pub fn get_int(&self) -> Option<i64> {
        match self {
            RadioValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// 获取浮点数
    pub fn get_float(&self) -> Option<f64> {
        match self {
            RadioValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    /// 获取布尔值
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            RadioValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}

impl Default for RadioValue {
    fn default() -> Self {
        RadioValue::String(String::new())
    }
}

impl From<String> for RadioValue {
    fn from(v: String) -> Self {
        RadioValue::String(v)
    }
}

impl From<&str> for RadioValue {
    fn from(v: &str) -> Self {
        RadioValue::String(v.to_string())
    }
}

impl From<i64> for RadioValue {
    fn from(v: i64) -> Self {
        RadioValue::Int(v)
    }
}

impl From<i32> for RadioValue {
    fn from(v: i32) -> Self {
        RadioValue::Int(v as i64)
    }
}

impl From<f64> for RadioValue {
    fn from(v: f64) -> Self {
        RadioValue::Float(v)
    }
}

impl From<f32> for RadioValue {
    fn from(v: f32) -> Self {
        RadioValue::Float(v as f64)
    }
}

impl From<bool> for RadioValue {
    fn from(v: bool) -> Self {
        RadioValue::Bool(v)
    }
}

impl std::fmt::Display for RadioValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadioValue::String(v) => write!(f, "{}", v),
            RadioValue::Int(v) => write!(f, "{}", v),
            RadioValue::Float(v) => write!(f, "{}", v),
            RadioValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

/// Radio 单选框组件
#[derive(Debug, Clone, ComponentBase)]
pub struct Radio {
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

    /// 单选框的值
    value: Option<RadioValue>,
    /// 在 RadioGroup 中的选中值（用于判断是否选中）
    checked_value: Option<Signal<RadioValue>>,
    /// 值改变时的回调（用于 RadioGroup 中）
    onchange: Option<EventHandler<RadioValue>>,
    /// 单选框尺寸
    size: RadioSize,
    /// 是否禁用
    disabled: bool,
    /// 是否显示边框
    border: bool,
    /// 是否使用按钮样式
    button: bool,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-radio".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            value: None,
            checked_value: None,
            onchange: None,
            size: RadioSize::Medium,
            disabled: false,
            border: false,
            button: false,
        }
    }
}

impl Radio {
    /// 创建一个新的单选框实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置单选框的标签文本
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.childrens.push(Rc::new(Text::span(label)));
        self
    }

    /// 设置单选框的标签元素
    pub fn label_element<T>(mut self, component: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.childrens.push(Rc::new(component));
        self
    }

    /// 设置单选框的值
    pub fn value(mut self, value: impl Into<RadioValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置在 RadioGroup 中的选中值（RadioGroup 内部使用）
    pub fn checked_value(mut self, value: Signal<RadioValue>) -> Self {
        self.checked_value = Some(value);
        self
    }

    /// 设置值改变回调
    pub fn onchange(mut self, handler: impl FnMut(RadioValue) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件
    pub fn onchange2(mut self, handler: EventHandler<RadioValue>) -> Self {
        self.onchange = Some(handler);
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置单选框尺寸
    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否显示边框
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    /// 设置是否使用按钮样式
    pub fn button(mut self, button: bool) -> Self {
        self.button = button;
        self
    }
}

/// 便捷方法
impl Radio {
    /// 设置为小尺寸单选框
    pub fn as_small(mut self) -> Self {
        self.size = RadioSize::Small;
        self
    }

    /// 设置为中等尺寸单选框
    pub fn as_medium(mut self) -> Self {
        self.size = RadioSize::Medium;
        self
    }

    /// 设置为大尺寸单选框
    pub fn as_large(mut self) -> Self {
        self.size = RadioSize::Large;
        self
    }
}

impl ToElement for Radio {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        // 获取 value，如果未设置则使用默认值
        let value = self.value.clone().unwrap_or_default();

        // 判断是否选中
        let is_checked_signal = self.checked_value;
        let value_for_check = value.clone();
        let is_checked = use_memo(move || {
            if let Some(signal) = &is_checked_signal {
                let current = signal.read();
                *current == value_for_check
            } else {
                false
            }
        });

        // 计算样式类名
        let mut class_names = vec![self.class.clone()];

        // 按钮样式
        if self.button {
            class_names.push("t-radio--button".to_string());
        }
        // 按钮边框样式
        if self.border {
            class_names.push("t-radio--button__border".to_string());
        }

        // 添加尺寸类名
        let size_class = self.size.to_string();
        if !size_class.is_empty() {
            class_names.push(size_class);
        }

        if *is_checked.read() {
            class_names.push("is-checked".to_string());
        }

        if self.disabled {
            class_names.push("is-disabled".to_string());
        }

        let class = class_names.join(" ");

        // 计算样式
        let mut style_str = String::new();
        if let Some(style) = &self.style {
            style_str = style.to_string();
        }

        let disabled = self.disabled;
        let onchange_handler = self.onchange;
        let item_value_for_onchange = value.clone();
        let item_value_for_input = value.to_string();
        let onclick_custom = self.onclick;
        let is_checked_signal_for_onclick = is_checked_signal;

        // 点击事件
        let onclick = move |event: MouseEvent| {
            if disabled {
                return;
            }

            // 更新 checked_value（如果在 RadioGroup 中）
            if let Some(mut signal) = is_checked_signal_for_onclick {
                signal.set(item_value_for_onchange.clone());
            }

            // 触发 onchange 回调
            if let Some(handler) = &onchange_handler {
                handler.call(item_value_for_onchange.clone());
            }

            // 触发自定义 onclick
            if let Some(handler) = &onclick_custom {
                handler.call(event);
            }
        };

        // 获取 label or 子元素内容
        let childrens = self.childrens_to_element();

        rsx! {
            label { id, class, style: style_str,
                span { class: "t-radio__input",
                    span { class: "t-radio__inner" }
                    input {
                        r#type: "radio",
                        value: item_value_for_input,
                        checked: *is_checked.read(),
                        disabled,
                        onclick,
                    }
                }
                span { class: if self.button { "t-radio__button" } else { "t-radio__label" }, {childrens} }
            }
        }
    }
}

/// RadioGroup 单选框组组件
#[derive(Debug, Clone, ComponentBase)]
pub struct RadioGroup {
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

    /// 单选框列表
    radios: Vec<Radio>,
    /// 当前值的 Signal（受控状态）
    value: Option<Signal<RadioValue>>,
    /// 是否禁用
    disabled: bool,
    /// 单选框尺寸
    size: RadioSize,
    /// 是否显示边框
    border: bool,
    /// 是否使用按钮样式
    button: bool,
    /// 绑定值变化时触发的事件
    onchange: Option<EventHandler<RadioValue>>,
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-radio-group".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            radios: Vec::new(),
            value: None,
            disabled: false,
            size: RadioSize::default(),
            border: false,
            button: false,
            onchange: None,
        }
    }
}

impl RadioGroup {
    /// 创建一个新的单选框组实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 添加单选框
    pub fn radio(mut self, radio: Radio) -> Self {
        self.radios.push(radio);
        self
    }

    /// 添加单选框列表
    pub fn radios(mut self, radio: Vec<Radio>) -> Self {
        self.radios = radio;
        self
    }

    /// 设置当前值的 Signal（必需）
    pub fn value(mut self, value: Signal<RadioValue>) -> Self {
        self.value = Some(value);
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置单选框尺寸
    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否显示边框
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    /// 设置是否使用按钮样式
    pub fn button(mut self, button: bool) -> Self {
        self.button = button;
        self
    }

    /// 设置值改变事件
    pub fn onchange(mut self, handler: impl FnMut(RadioValue) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件
    pub fn onchange2(mut self, handler: EventHandler<RadioValue>) -> Self {
        self.onchange = Some(handler);
        self
    }
}

/// 便捷方法
impl RadioGroup {
    /// 设置为小尺寸单选框
    pub fn as_small(mut self) -> Self {
        self.size = RadioSize::Small;
        self
    }

    /// 设置为中等尺寸单选框
    pub fn as_medium(mut self) -> Self {
        self.size = RadioSize::Medium;
        self
    }

    /// 设置为大尺寸单选框
    pub fn as_large(mut self) -> Self {
        self.size = RadioSize::Large;
        self
    }
}

impl ToElement for RadioGroup {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        let mut class_names = vec![self.class.clone(), self.size.to_string()];
        if self.disabled {
            class_names.push("t-radio-group--disabled".to_string());
        }
        let class = class_names.join(" ");

        let style = self.style.clone().map(|s| s.to_string());

        // 获取 value signal，如果未设置则使用默认值
        let value_signal = self
            .value
            .unwrap_or_else(|| Signal::new(RadioValue::default()));
        let disabled = self.disabled;
        let size = self.size;
        let button = self.button;
        let border = self.border;
        let onchange_handler = self.onchange;

        let radios = self
            .radios
            .clone()
            .into_iter()
            .map(|radio: Radio| {
                let old_disabled = radio.disabled;
                let mut new_radio = radio
                    .checked_value(value_signal)
                    .disabled(old_disabled || disabled)
                    .size(size)
                    .button(button)
                    .border(border);
                if let Some(handler) = onchange_handler {
                    new_radio = new_radio.onchange2(handler);
                }
                new_radio
            })
            .collect::<Vec<Radio>>();

        rsx! {
            div { id, class, style,
                for radio in radios.iter() {
                    {radio.to_element()}
                }
            }
        }
    }
}
