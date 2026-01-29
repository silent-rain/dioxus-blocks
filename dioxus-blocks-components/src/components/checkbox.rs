//! Checkbox 多选框组件
//!
//! 提供多选框和多选框组组件，支持边框、按钮样式、中间状态等功能。
//!
//! # 组件模式
//!
//! Checkbox 和 CheckboxGroup 是**受控组件**，需要通过 Signal 传递值，并通过 change 事件更新状态。
//!
//! # 示例
//!
//! ## 基础用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Checkbox, CheckboxGroup, CheckboxValue};
//!
//! let mut checkbox = use_signal(|| vec![CheckboxValue::String("Option 1".to_string())]);
//! rsx! {
//!     CheckboxGroup { value: checkbox, onchange: move |v| checkbox.set(v),
//!         Checkbox { value: "Option 1", "Option 1" }
//!         Checkbox { value: "Option 2", "Option 2" }
//!         Checkbox { value: "Option 3", "Option 3" }
//!     }
//! }
//! ```
//!
//! ## 禁用状态
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Checkbox, CheckboxGroup};
//!
//! let mut checkbox = use_signal(|| vec![1]);
//! rsx! {
//!     CheckboxGroup { value: checkbox, onchange: move |v| checkbox.set(v),
//!         Checkbox { value: 1, disabled: true, "Option A" }
//!         Checkbox { value: 2, disabled: true, "Option B" }
//!     }
//! }
//! ```
//!
//! ## 按钮样式
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Checkbox, CheckboxGroup};
//!
//! let mut checkbox = use_signal(|| vec!["Shanghai"]);
//! rsx! {
//!     CheckboxGroup { value: checkbox, onchange: move |v| checkbox.set(v),
//!         Checkbox { value: "Shanghai", button: true, "Shanghai" }
//!         Checkbox { value: "Beijing", button: true, "Beijing" }
//!         Checkbox { value: "Guangzhou", button: true, "Guangzhou" }
//!     }
//! }
//! ```
//!
//! ## 带有边框
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Checkbox, CheckboxGroup};
//!
//! let mut checkbox = use_signal(|| vec![1]);
//! rsx! {
//!     CheckboxGroup { value: checkbox, onchange: move |v| checkbox.set(v),
//!         Checkbox { value: 1, border: true, "Option A" }
//!         Checkbox { value: 2, border: true, "Option B" }
//!     }
//! }
//! ```

use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, Text, traits::ToElement};

/// 多选框尺寸枚举
///
/// 定义多选框的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for CheckboxSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckboxSize::Medium => write!(f, ""),
            CheckboxSize::Small => write!(f, "t-checkbox--small"),
            CheckboxSize::Large => write!(f, "t-checkbox--large"),
        }
    }
}

/// 多选框值枚举
///
/// 支持多种类型的值。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckboxValue {
    /// 字符串类型
    String(String),
    /// 整数类型
    Int(i64),
    /// 布尔类型
    Bool(bool),
}

impl Default for CheckboxValue {
    fn default() -> Self {
        CheckboxValue::String(String::new())
    }
}

impl From<String> for CheckboxValue {
    fn from(v: String) -> Self {
        CheckboxValue::String(v)
    }
}

impl From<&str> for CheckboxValue {
    fn from(v: &str) -> Self {
        CheckboxValue::String(v.to_string())
    }
}

impl From<i64> for CheckboxValue {
    fn from(v: i64) -> Self {
        CheckboxValue::Int(v)
    }
}

impl From<i32> for CheckboxValue {
    fn from(v: i32) -> Self {
        CheckboxValue::Int(v as i64)
    }
}

impl From<bool> for CheckboxValue {
    fn from(v: bool) -> Self {
        CheckboxValue::Bool(v)
    }
}

impl std::fmt::Display for CheckboxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckboxValue::String(v) => write!(f, "{}", v),
            CheckboxValue::Int(v) => write!(f, "{}", v),
            CheckboxValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

/// Checkbox 多选框组件
#[derive(Debug, Clone, ComponentBase)]
pub struct Checkbox {
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

    /// 多选框的值
    value: Option<CheckboxValue>,
    /// 在 CheckboxGroup 中的选中值列表（用于判断是否选中）
    checked_values: Option<Signal<Vec<CheckboxValue>>>,
    /// 独立使用时的选中状态（用于单选模式）
    checked_bool: Option<Signal<bool>>,
    /// 值改变时的回调（用于 CheckboxGroup 中）
    onchange: Option<EventHandler<CheckboxValue>>,
    /// 多选框尺寸
    size: CheckboxSize,
    /// 是否禁用
    disabled: bool,
    /// 是否显示边框
    border: bool,
    /// 是否使用按钮样式
    button: bool,
    /// 是否为中间状态（仅用于全选场景）
    indeterminate: bool,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-checkbox".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            value: None,
            checked_values: None,
            checked_bool: None,
            onchange: None,
            size: CheckboxSize::Medium,
            disabled: false,
            border: false,
            button: false,
            indeterminate: false,
        }
    }
}

impl Checkbox {
    /// 创建一个新的多选框实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置多选框的值
    pub fn value(mut self, value: impl Into<CheckboxValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置多选框的标签文本
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.childrens.push(Rc::new(Text::span(label)));
        self
    }

    /// 设置多选框的标签元素
    pub fn label_element<T>(mut self, component: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.childrens.push(Rc::new(component));
        self
    }

    /// 设置在 CheckboxGroup 中的选中值列表（CheckboxGroup 内部使用）
    pub fn checked_values(mut self, values: Signal<Vec<CheckboxValue>>) -> Self {
        self.checked_values = Some(values);
        self
    }

    /// 设置是否选中（独立使用时，用于单选模式）
    pub fn checked(mut self, checked: Signal<bool>) -> Self {
        self.checked_bool = Some(checked);
        self
    }

    /// 设置值改变回调（CheckboxGroup 内部使用）
    pub fn onchange(mut self, handler: impl FnMut(CheckboxValue) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件
    pub fn onchange2(mut self, handler: EventHandler<CheckboxValue>) -> Self {
        self.onchange = Some(handler);
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置多选框尺寸
    pub fn size(mut self, size: CheckboxSize) -> Self {
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

    /// 设置中间状态
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
}

impl ToElement for Checkbox {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        // 获取 value，如果未设置则使用默认值
        let item_value = self.value.clone().unwrap_or_default();

        // 判断是否选中 - 支持两种模式
        // 模式1: CheckboxGroup 中，使用 checked_values (Vec<CheckboxValue>)
        // 模式2: 独立使用，使用 checked_bool (bool)
        let checked_values_signal = self.checked_values.clone();
        let checked_bool_signal = self.checked_bool.clone();
        let item_value_for_check = item_value.clone();
        let is_checked = use_memo(move || {
            // 优先使用 checked_values (CheckboxGroup 模式)
            if let Some(signal) = &checked_values_signal {
                let current = signal.read();
                current.contains(&item_value_for_check)
            }
            // 其次使用 checked_bool (独立使用模式)
            else if let Some(signal) = &checked_bool_signal {
                *signal.read()
            } else {
                false
            }
        });

        // 计算样式类名
        let mut class_names = vec![self.class.clone()];

        if self.button {
            class_names.push("t-checkbox--button".to_string());
        }

        // 添加尺寸类名
        let size_class = self.size.to_string();
        if !size_class.is_empty() {
            class_names.push(size_class);
        }

        if self.border {
            class_names.push("is-bordered".to_string());
        }

        if *is_checked.read() {
            class_names.push("is-checked".to_string());
        }

        if self.disabled {
            class_names.push("is-disabled".to_string());
        }

        if self.indeterminate {
            class_names.push("is-indeterminate".to_string());
        }

        let class = class_names.join(" ");

        // 计算样式
        let mut style_str = String::new();
        if let Some(style) = &self.style {
            style_str = style.to_string();
        }

        let disabled = self.disabled;
        let onchange_handler = self.onchange;
        let item_value_for_onchange = item_value.clone();
        let item_value_for_input = item_value.to_string();
        let onclick_custom = self.onclick;
        let checked_values_signal_for_onclick = self.checked_values.clone();
        let checked_bool_signal_for_onclick = self.checked_bool.clone();
        let _indeterminate = self.indeterminate;

        // 点击事件
        let onclick = move |event: MouseEvent| {
            if disabled {
                return;
            }

            // 更新 checked_values（如果在 CheckboxGroup 中）
            if let Some(mut signal) = checked_values_signal_for_onclick {
                let mut current = signal.read().clone();
                if current.contains(&item_value_for_onchange) {
                    current.retain(|v| v != &item_value_for_onchange);
                } else {
                    current.push(item_value_for_onchange.clone());
                }
                signal.set(current);
            }

            // 更新 checked_bool（如果独立使用）
            if let Some(mut signal) = checked_bool_signal_for_onclick {
                let current = *signal.read();
                signal.set(!current);
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
                span { class: "t-checkbox__input",
                    span { class: "t-checkbox__inner" }
                    input {
                        r#type: "checkbox",
                        value: item_value_for_input,
                        checked: *is_checked.read(),
                        disabled,
                        onclick,
                    }
                }
                span { class: if self.button { "t-checkbox__button" } else { "t-checkbox__label" }, {childrens} }
            }
        }
    }
}

/// CheckboxGroup 多选框组组件
#[derive(Debug, Clone, ComponentBase)]
pub struct CheckboxGroup {
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

    /// 多选框列表
    checkboxes: Vec<Checkbox>,
    /// 当前值的 Signal（受控状态）
    value: Option<Signal<Vec<CheckboxValue>>>,
    /// 是否禁用
    disabled: bool,
    /// 多选框尺寸
    size: CheckboxSize,
    /// 最小可选数量
    min: Option<usize>,
    /// 最大可选数量
    max: Option<usize>,
    /// 绑定值变化时触发的事件
    onchange: Option<EventHandler<Vec<CheckboxValue>>>,
}

impl Default for CheckboxGroup {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-checkbox-group".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            checkboxes: Vec::new(),
            value: None,
            disabled: false,
            size: CheckboxSize::default(),
            min: None,
            max: None,
            onchange: None,
        }
    }
}

impl CheckboxGroup {
    /// 创建一个新的多选框组实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 添加多选框
    pub fn checkbox(mut self, checkbox: Checkbox) -> Self {
        self.checkboxes.push(checkbox);
        self
    }

    /// 添加多选框列表
    pub fn checkboxes(mut self, checkbox: Vec<Checkbox>) -> Self {
        self.checkboxes = checkbox;
        self
    }

    /// 设置当前值的 Signal（必需）
    pub fn value(mut self, value: Signal<Vec<CheckboxValue>>) -> Self {
        self.value = Some(value);
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置多选框尺寸
    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    /// 设置最小可选数量
    pub fn min(mut self, min: usize) -> Self {
        self.min = Some(min);
        self
    }

    /// 设置最大可选数量
    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    /// 设置值改变事件
    pub fn onchange(mut self, handler: impl FnMut(Vec<CheckboxValue>) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件
    pub fn onchange2(mut self, handler: EventHandler<Vec<CheckboxValue>>) -> Self {
        self.onchange = Some(handler);
        self
    }
}

/// 便捷方法
impl CheckboxGroup {
    /// 设置为小尺寸多选框
    pub fn as_small(mut self) -> Self {
        self.size = CheckboxSize::Small;
        self
    }

    /// 设置为中等尺寸多选框
    pub fn as_medium(mut self) -> Self {
        self.size = CheckboxSize::Medium;
        self
    }

    /// 设置为大尺寸多选框
    pub fn as_large(mut self) -> Self {
        self.size = CheckboxSize::Large;
        self
    }
}

impl ToElement for CheckboxGroup {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        let mut class_names = vec![self.class.clone(), self.size.to_string()];
        if self.disabled {
            class_names.push("t-checkbox-group--disabled".to_string());
        }
        let class = class_names.join(" ");

        let style = self.style.clone().map(|s| s.to_string());

        // 获取 value signal，如果未设置则使用默认值
        let value_signal = self
            .value
            .unwrap_or_else(|| Signal::new(Vec::<CheckboxValue>::new()));
        let disabled = self.disabled;
        let size = self.size;
        let onchange_handler = self.onchange;
        let min = self.min;
        let max = self.max;
        let value_signal_for_check = value_signal.clone();

        let checkboxes = self
            .checkboxes
            .clone()
            .into_iter()
            .map(|checkbox: Checkbox| {
                let old_disabled = checkbox.disabled;

                let new_checkbox = checkbox
                    .checked_values(value_signal.clone())
                    .disabled(old_disabled || disabled)
                    .size(size)
                    .onchange(move |val| {
                        // 检查 min/max 限制
                        let current = value_signal_for_check.read().clone();

                        // 如果是取消选中，检查最小限制
                        if current.contains(&val) {
                            if let Some(min_count) = min {
                                if current.len() <= min_count {
                                    // 不允许取消选中
                                    return;
                                }
                            }
                        }

                        // 如果是选中，检查最大限制
                        else {
                            if let Some(max_count) = max {
                                if current.len() >= max_count {
                                    // 不允许选中
                                    return;
                                }
                            }
                        }

                        // 触发 onchange 回调（传递完整列表）
                        if let Some(handler) = onchange_handler {
                            handler.call(value_signal_for_check.read().clone());
                        }
                    });
                new_checkbox
            })
            .collect::<Vec<Checkbox>>();

        rsx! {
            div { id, class, style,
                for checkbox in checkboxes.iter() {
                    {checkbox.to_element()}
                }
            }
        }
    }
}
