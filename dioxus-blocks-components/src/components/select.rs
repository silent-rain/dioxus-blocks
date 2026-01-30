//! Select 选择器组件
//!
//! 提供单选和多选下拉选择器组件，支持基础用法、禁用状态、可清空、尺寸筛选选项等功能。

use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// Select 尺寸枚举
///
/// 定义选择器的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for SelectSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectSize::Medium => write!(f, ""),
            SelectSize::Small => write!(f, "t-select--small"),
            SelectSize::Large => write!(f, "t-select--large"),
        }
    }
}

/// Select 值枚举
///
/// 支持多种类型的值。
#[derive(Debug, Clone, PartialEq)]
pub enum SelectValue {
    /// 字符串类型
    String(String),
    /// 整数类型
    Int(i64),
    /// 浮点数类型
    Float(f64),
    /// 布尔类型
    Bool(bool),
}

impl Default for SelectValue {
    fn default() -> Self {
        SelectValue::String(String::new())
    }
}

impl From<String> for SelectValue {
    fn from(v: String) -> Self {
        SelectValue::String(v)
    }
}

impl From<&str> for SelectValue {
    fn from(v: &str) -> Self {
        SelectValue::String(v.to_string())
    }
}

impl From<i64> for SelectValue {
    fn from(v: i64) -> Self {
        SelectValue::Int(v)
    }
}

impl From<i32> for SelectValue {
    fn from(v: i32) -> Self {
        SelectValue::Int(v as i64)
    }
}

impl From<f64> for SelectValue {
    fn from(v: f64) -> Self {
        SelectValue::Float(v)
    }
}

impl From<f32> for SelectValue {
    fn from(v: f32) -> Self {
        SelectValue::Float(v as f64)
    }
}

impl From<bool> for SelectValue {
    fn from(v: bool) -> Self {
        SelectValue::Bool(v)
    }
}

impl std::fmt::Display for SelectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectValue::String(v) => write!(f, "{}", v),
            SelectValue::Int(v) => write!(f, "{}", v),
            SelectValue::Float(v) => write!(f, "{}", v),
            SelectValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

/// SelectOption 选项组件
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    /// 选项的值
    value: SelectValue,
    /// 选项的标签
    label: String,
    /// 是否禁用
    disabled: bool,
}

impl SelectOption {
    /// 创建一个新的选项实例
    pub fn new(value: impl Into<SelectValue>) -> Self {
        let value = value.into();
        let label = value.to_string();
        Self {
            value,
            label,
            disabled: false,
        }
    }

    /// 设置选项的标签
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Select 选择器组件
#[derive(Debug, Clone, ComponentBase)]
pub struct Select {
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

    /// 当前值（受控状态）
    value: Option<SelectValue>,
    /// 当前多选值（受控状态）
    multiple_value: Option<Signal<Vec<SelectValue>>>,
    /// 选项列表
    options: Vec<SelectOption>,
    /// 是否多选
    multiple: bool,
    /// 是否禁用
    disabled: bool,
    /// 选择器尺寸
    size: SelectSize,
    /// 是否可清空
    clearable: bool,
    /// 是否可筛选
    filterable: bool,
    /// 占位符文本
    placeholder: String,
    /// 值改变时的回调（单选）
    onchange: Option<EventHandler<SelectValue>>,
    /// 值改变时的回调（多选）
    onchange_multiple: Option<EventHandler<Vec<SelectValue>>>,
    /// 清空时的回调
    onclear: Option<EventHandler<MouseEvent>>,
}

impl Default for Select {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-select".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            value: None,
            multiple_value: None,
            options: Vec::new(),
            multiple: false,
            disabled: false,
            size: SelectSize::Medium,
            clearable: false,
            filterable: false,
            placeholder: "Select".to_string(),
            onchange: None,
            onchange_multiple: None,
            onclear: None,
        }
    }
}

impl Select {
    /// 创建一个新的选择器实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置当前值（单选）
    pub fn value(mut self, value: impl Into<SelectValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置当前值（多选）
    pub fn multiple_value(mut self, value: Signal<Vec<SelectValue>>) -> Self {
        self.multiple_value = Some(value);
        self
    }

    /// 添加选项
    pub fn option(mut self, option: SelectOption) -> Self {
        self.options.push(option);
        self
    }

    /// 添加选项列表
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    /// 设置禁用状态
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置选择器尺寸
    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    /// 设置是否可清空
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// 设置是否可筛选
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// 设置占位符文本
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// 设置是否多选
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// 设置值改变事件（单选）
    pub fn onchange(mut self, handler: impl FnMut(SelectValue) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件（单选）
    pub fn onchange2(mut self, handler: EventHandler<SelectValue>) -> Self {
        self.onchange = Some(handler);
        self
    }

    /// 设置值改变事件（多选）
    pub fn onchange_multiple(mut self, handler: impl FnMut(Vec<SelectValue>) + 'static) -> Self {
        self.onchange_multiple = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件（多选）
    pub fn onchange_multiple2(mut self, handler: EventHandler<Vec<SelectValue>>) -> Self {
        self.onchange_multiple = Some(handler);
        self
    }

    /// 设置清空事件
    pub fn onclear(mut self, handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.onclear = Some(EventHandler::new(handler));
        self
    }

    /// 设置清空事件
    pub fn onclear2(mut self, handler: EventHandler<MouseEvent>) -> Self {
        self.onclear = Some(handler);
        self
    }
}

/// 便捷方法
impl Select {
    /// 设置为小尺寸选择器
    pub fn as_small(mut self) -> Self {
        self.size = SelectSize::Small;
        self
    }

    /// 设置为中等尺寸选择器
    pub fn as_medium(mut self) -> Self {
        self.size = SelectSize::Medium;
        self
    }

    /// 设置为大尺寸选择器
    pub fn as_large(mut self) -> Self {
        self.size = SelectSize::Large;
        self
    }
}

impl ToElement for Select {
    fn to_element(&self) -> Element {
        let mut is_opened = use_signal(|| false);
        let mut filter_text = use_signal(|| String::new());
        let current_value = use_signal(|| self.value.clone());

        // 克隆闭包需要的所有数据
        let class = self.class.clone();
        let size = self.size;
        let style = self.style.clone();
        let disabled = self.disabled;
        let multiple = self.multiple;
        let multiple_value = self.multiple_value.clone();
        let clearable = self.clearable;
        let filterable = self.filterable;
        let placeholder = self.placeholder.clone();
        let onchange = self.onchange.clone();
        let onchange_multiple = self.onchange_multiple.clone();
        let onclear = self.onclear.clone();
        let options = self.options.clone();

        let display_class = if multiple {
            "is-multiple"
        } else if disabled {
            "is-disabled"
        } else {
            ""
        };

        // 计算显示的标签
        let get_selected_label = {
            let options = options.clone();
            let placeholder = placeholder.clone();
            let current_value = current_value.clone();
            move || {
                let current_value = current_value.read();
                if let Some(value) = current_value.as_ref() {
                    options
                        .iter()
                        .find(|opt| &opt.value == value)
                        .map(|opt| opt.label.clone())
                        .unwrap_or_else(|| placeholder.clone())
                } else {
                    placeholder.clone()
                }
            }
        };

        // 判断是否显示清空按钮
        let get_show_clear = {
            let clearable = clearable;
            let disabled = disabled;
            let is_opened = is_opened.clone();
            let current_value = current_value.clone();
            move || {
                let current_value = current_value.read();
                clearable
                    && current_value.is_some()
                    && !disabled
                    && !is_opened()
            }
        };

        rsx! {
            div {
                class: format_args!("{} {} {}", class, size, display_class),
                style: style.as_ref().map(|s| s.to_string()),
                onclick: move |_e: Event<MouseData>| {
                    if !disabled {
                        is_opened.set(!is_opened());
                    }
                },

                // 选择器输入区域
                div {
                    class: "t-select__wrapper",
                    style: if disabled { "cursor: not-allowed;" } else { "" },

                    // 输入框
                    div { class: "t-select__input",
                        if filterable && is_opened() && !disabled {
                            input {
                                value: "{filter_text}",
                                placeholder: placeholder.clone(),
                                oninput: move |e: Event<FormData>| {
                                    filter_text.set(e.value());
                                },
                                onclick: move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                },
                            }
                        } else if multiple && multiple_value.is_some() {
                            // 多选标签显示
                            span {
                                {
                                    if let Some(mv) = &multiple_value {
                                        let options = options.clone();
                                        mv.read()
                                            .iter()
                                            .filter_map(|v| {
                                                options
                                                    .iter()
                                                    .find(|opt| &opt.value == v)
                                                    .map(|opt| opt.label.clone())
                                            }
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    } else {
                                        String::new()
                                    }
                                }
                            }
                        } else {
                            // 单选显示
                            span { "{get_selected_label()}" }
                        }
                    }

                    // 清空按钮
                    {
                        let onclear = onclear.clone();
                        let onchange = onchange.clone();
                        let mut filter_text = filter_text.clone();
                        let mut current_value = current_value.clone();
                        let get_show_clear = get_show_clear.clone();

                        if get_show_clear() {
                            rsx! {
                                span {
                                    class: "t-select__clear",
                                    onclick: move |e: Event<MouseData>| {
                                        e.stop_propagation();
                                        if let Some(ref handler) = onclear {
                                            handler.call(e.clone());
                                        }
                                        if let Some(ref handler) = onchange {
                                            handler.call(SelectValue::String(String::new()));
                                        }
                                        current_value.set(None);
                                        filter_text.set(String::new());
                                    },
                                    "×"
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }

                    // 下拉箭头
                    span {
                        class: "t-select__arrow",
                        style: if is_opened() { "transform: translateY(-50%) rotate(180deg);" } else { "" },
                        if is_opened() {
                            "▲"
                        } else {
                            "▼"
                        }
                    }
                }

                // 下拉菜单
                {
                    let dropdown_options = options.clone();
                    let dropdown_multiple = multiple;
                    let dropdown_multiple_value = multiple_value.clone();
                    let dropdown_onchange = onchange.clone();
                    let dropdown_onchange_multiple = onchange_multiple.clone();
                    let mut dropdown_is_opened = is_opened.clone();
                    let dropdown_current_value = current_value.clone();
                    let dropdown_filter_text = filter_text.clone();
                    let dropdown_filterable = filterable;
                    let dropdown_disabled = disabled;

                    if is_opened() && !dropdown_disabled {
                        rsx! {
                            div {
                                class: "t-select__dropdown",
                                onclick: move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                },
                                onblur: move |_| {
                                    dropdown_is_opened.set(false);
                                },

                                if dropdown_filterable && !dropdown_filter_text().is_empty()
                                    && dropdown_options
                                        .iter()
                                        .all(|opt| {
                                            !opt
                                                .label
                                                .to_lowercase()
                                                .contains(&dropdown_filter_text().to_lowercase())
                                        })
                                {
                                    div { class: "t-select__empty", "暂无数据" }
                                } else {
                                    {
                                        dropdown_options
                                            .iter()
                                            .map(|option| {
                                                let is_hidden = dropdown_filterable && !dropdown_filter_text().is_empty()
                                                    && !option
                                                        .label
                                                        .to_lowercase()
                                                        .contains(&dropdown_filter_text().to_lowercase());
                                                let is_selected = if dropdown_multiple {
                                                    dropdown_multiple_value
                                                        .as_ref()
                                                        .map(|mv| { mv.read().contains(&option.value) })
                                                        .unwrap_or(false)
                                                } else {
                                                    dropdown_current_value.read().as_ref() == Some(&option.value)
                                                }
                                                mv.set(values.clone());
                                                let option_value = option.value.clone();
                                                let option_label = option.label.clone();
                                                let option_disabled = option.disabled;
                                                let option_onchange = dropdown_onchange.clone();
                                                let option_onchange_multiple = dropdown_onchange_multiple.clone();
                                                let mut option_is_opened = dropdown_is_opened.clone();
                                                let mut option_current_value = dropdown_current_value.clone();
                                                let option_is_multiple = dropdown_multiple;
                                                let option_multiple_value = dropdown_multiple_value.clone();
                                                if is_hidden {
                                                    rsx! {}
                                                }
                                                }
                                                    rsx! {
                                                        div {
                                                            class: if option_disabled { "t-select__option is-disabled" } else if is_selected { "t-select__option is-selected" } else { "t-select__option" },
                                                            onclick: move |e: Event<MouseData>| {
                                                                e.stop_propagation();
                                                                if !option_disabled {
                                                                    if option_is_multiple {
                                                                        if let Some(mut mv) = option_multiple_value {
                                                                            let mut values = mv.read().clone();
                                                                            if let Some(pos) = values.iter().position(|v| v == &option_value) {
                                                                                values.remove(pos);
                                                                            } else {
                                                                                values.push(option_value.clone());
                                                                            }
                                                                            mv.set(values.clone());
                                                                            if let Some(ref handler) = option_onchange_multiple {
                                                                                handler.call(values);
                                                                            }
                                                                        }
                                                                    } else {
                                                                        option_current_value.set(Some(option_value.clone()));
                                                                        option_is_opened.set(false);
                                                                        if let Some(ref handler) = option_onchange {
                                                                            handler.call(option_value.clone());
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            {option_label}
                                                        }
                                                    }
                                                }
                                            })
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }

                // 点击外部关闭下拉菜单
                div {
                    class: "t-select__backdrop",
                    style: if is_opened() { "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 1999;" } else { "display: none;" },
                    onclick: move |_| {
                        is_opened.set(false);
                    },
                }
            }
        }
    }
}
