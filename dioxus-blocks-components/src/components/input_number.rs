//! InputNumber 组件
//!
//! 提供一个数字输入框组件，支持精度控制、步进、不同尺寸和禁用状态。
//! 使用 Decimal 处理小数精度问题，避免 f64 的精度损失和额外小数位问题。
//!
//! # 组件模式
//!
//! InputNumber 是一个**受控组件**，需要通过 Signal 传递值，并通过 onchange 回调更新状态。
//!
//! # 示例
//!
//! ## 基础使用（整数）
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| InputNumberValue::Int(10));
//!     InputNumber::new()
//!             .value(value)
//!             .step_int(1)
//!             .onchange(move |v| value.set(v))
//!             .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 浮点数类型（精确精度）
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
//! use rust_decimal::Decimal;
//! use std::str::FromStr;
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| {
//!         InputNumberValue::Float(Decimal::from_str("1.234567").unwrap())
//!     });
//!     InputNumber::new()
//!             .value(value)
//!             .step_float(0.1)
//!             .onchange(move |v| {
//!                 println!("Value changed: {:?}", v);
//!                 value.set(v);
//!             })
//!             .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 禁用状态
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
//! use rust_decimal::Decimal;
//! use std::str::FromStr;
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| {
//!         InputNumberValue::Float(Decimal::from_str("5.0").unwrap())
//!     });
//!      InputNumber::new()
//!             .value(value)
//!             .disabled(true)
//!             .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 精度控制
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
//! use rust_decimal::Decimal;
//! use std::str::FromStr;
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| {
//!         InputNumberValue::Float(Decimal::from_str("3.141592653589793").unwrap())
//!     });
//!     InputNumber::new()
//!             .value(value)
//!             .precision(6)
//!             .onchange(move |v| value.set(v))
//!             .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 步进设置
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
//! use rust_decimal::Decimal;
//! use std::str::FromStr;
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut value = use_signal(|| {
//!         InputNumberValue::Float(Decimal::from_str("10.0").unwrap())
//!     });
//!     InputNumber::new()
//!             .value(value)
//!             .step_float(5.0)
//!             .onchange(move |v| value.set(v))
//!             .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```
//!
//! ## 尺寸
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
//! use dioxus::core::Mutations;
//!
//! let mut dom = VirtualDom::new(|| {
//!     let mut small = use_signal(|| InputNumberValue::Int(1));
//!     InputNumber::new()
//!             .value(small)
//!             .as_small()
//!             .onchange(move |v| small.set(v))
//!             .to_element()
//! });
//! let mut mutations = Mutations::default();
//! dom.rebuild(&mut mutations);
//! ```

use std::rc::Rc;

use dioxus::prelude::*;
use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
};

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 解析输入字符串为 InputNumberValue
///
/// # 参数
///
/// * `input` - 输入字符串
/// * `is_float_type` - 是否为浮点数类型
///
/// # 返回值
///
/// 返回解析后的值或 None（解析失败）
fn parse_input_value(input: &str, is_float_type: bool) -> Option<InputNumberValue> {
    if is_float_type {
        input.parse::<Decimal>().map(InputNumberValue::Float).ok()
    } else {
        input.parse::<i64>().map(InputNumberValue::Int).ok()
    }
}

/// 应用边界约束到输入值
///
/// # 参数
///
/// * `value` - 要约束的值
/// * `min` - 可选的最小值
/// * `max` - 可选的最大值
///
/// # 返回值
///
/// 返回应用边界后的值
fn apply_bounds(
    value: InputNumberValue,
    min: &Option<InputNumberValue>,
    max: &Option<InputNumberValue>,
) -> InputNumberValue {
    if let Some(min_val) = min
        && value.cmp(min_val) == std::cmp::Ordering::Less
    {
        return min_val.clone();
    }

    if let Some(max_val) = max
        && value.cmp(max_val) == std::cmp::Ordering::Greater
    {
        return max_val.clone();
    }

    value
}

/// 通过步进计算新值
///
/// # 参数
///
/// * `current` - 当前值
/// * `step` - 步进值
/// * `is_increase` - true 为增加，false 为减少
/// * `min` - 可选的最小值
/// * `max` - 可选的最大值
///
/// # 返回值
///
/// 返回计算后的新值
fn calculate_step_value(
    current: InputNumberValue,
    step: &InputNumberStep,
    is_increase: bool,
    min: &Option<InputNumberValue>,
    max: &Option<InputNumberValue>,
) -> InputNumberValue {
    let new_value = match current {
        InputNumberValue::Float(v) => {
            let step_decimal = step.as_decimal();
            let new_dec = if is_increase {
                v + step_decimal
            } else {
                v - step_decimal
            };
            InputNumberValue::Float(new_dec)
        }
        InputNumberValue::Int(v) => {
            let step_int = if let InputNumberStep::Int(s) = step {
                *s
            } else {
                1
            };
            let new_int = if is_increase {
                v + step_int
            } else {
                v - step_int
            };
            InputNumberValue::Int(new_int)
        }
    };

    apply_bounds(new_value, min, max)
}

/// 输入框尺寸枚举
///
/// 定义输入框的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputNumberSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for InputNumberSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputNumberSize::Medium => write!(f, ""),
            InputNumberSize::Small => write!(f, "t-input-number--small"),
            InputNumberSize::Large => write!(f, "t-input-number--large"),
        }
    }
}

/// 按钮方向枚举
///
/// 定义步进按钮的位置。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlsPosition {
    /// 右侧（默认）
    #[default]
    Right,
    /// 两侧
    Both,
}

impl std::fmt::Display for ControlsPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlsPosition::Right => write!(f, ""),
            ControlsPosition::Both => write!(f, "t-input-number--controls-both"),
        }
    }
}

/// 输入框值类型枚举
///
/// 支持整数和浮点数两种类型。
/// 使用 Decimal 避免浮点数精度问题。
#[derive(Debug, Clone)]
pub enum InputNumberValue {
    /// 整数类型
    Int(i64),
    /// 浮点数类型（使用 Decimal 精确表示）
    Float(Decimal),
}

impl From<i64> for InputNumberValue {
    fn from(v: i64) -> Self {
        InputNumberValue::Int(v)
    }
}

impl From<Decimal> for InputNumberValue {
    fn from(v: Decimal) -> Self {
        InputNumberValue::Float(v)
    }
}

impl From<f64> for InputNumberValue {
    fn from(v: f64) -> Self {
        InputNumberValue::Float(Decimal::from_f64(v).unwrap_or_default())
    }
}

impl PartialEq for InputNumberValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InputNumberValue::Int(a), InputNumberValue::Int(b)) => a == b,
            (InputNumberValue::Float(a), InputNumberValue::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for InputNumberValue {}

impl PartialOrd for InputNumberValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InputNumberValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.as_decimal();
        let b = other.as_decimal();
        a.cmp(&b)
    }
}

impl InputNumberValue {
    /// 转换为 Decimal 表示
    pub fn as_decimal(&self) -> Decimal {
        match self {
            InputNumberValue::Int(v) => Decimal::from(*v),
            InputNumberValue::Float(v) => *v,
        }
    }

    /// 获取整数值
    pub fn get_int(&self) -> Option<i64> {
        match self {
            InputNumberValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// 获取浮点数值（f64，可能损失精度）
    pub fn get_float_f64(&self) -> Option<f64> {
        match self {
            InputNumberValue::Float(v) => v.to_f64(),
            _ => None,
        }
    }

    /// 判断是否为整数类型
    pub fn is_int(&self) -> bool {
        matches!(self, InputNumberValue::Int(_))
    }

    /// 判断是否为浮点数类型
    pub fn is_float(&self) -> bool {
        matches!(self, InputNumberValue::Float(_))
    }

    /// 转换为字符串（根据精度）
    pub fn to_string_with_precision(&self, precision: Option<u32>) -> String {
        match self {
            InputNumberValue::Int(v) => v.to_string(),
            InputNumberValue::Float(v) => {
                if let Some(prec) = precision {
                    format!("{:.prec$}", v, prec = prec as usize)
                } else {
                    // 未指定精度时，使用 normalize() 去除不必要的尾随零
                    v.normalize().to_string()
                }
            }
        }
    }
}

impl std::fmt::Display for InputNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputNumberValue::Int(v) => write!(f, "{}", v),
            InputNumberValue::Float(v) => write!(f, "{}", v.normalize()),
        }
    }
}

/// 步进值类型枚举
///
/// 支持整数和浮点数两种类型的步进值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputNumberStep {
    /// 整数步进
    Int(i64),
    /// 浮点数步进（使用 Decimal 精确表示）
    Float(Decimal),
}

impl InputNumberStep {
    /// 转换为 Decimal 表示
    pub fn as_decimal(&self) -> Decimal {
        match self {
            InputNumberStep::Int(v) => Decimal::from(*v),
            InputNumberStep::Float(v) => *v,
        }
    }
}

/// 数字输入框组件结构体
///
/// 提供一个可自定义的数字输入框，支持精度控制、步进、不同尺寸和禁用状态。
/// 这是一个受控组件，必须通过 `Signal<InputNumberValue>` 传入值。
///
/// # 使用说明
///
/// - 必须通过 `.value(signal)` 传入 `Signal<InputNumberValue>`
/// - 通过 `.onchange(handler)` 响应值的变化，通常需要更新 signal
/// - 不再使用 `.value(InputNumberValue)` 设置初始值
#[derive(Debug, Clone, ComponentBase)]
pub struct InputNumber {
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
    value: Option<Signal<InputNumberValue>>,
    /// 最小值
    min: Option<InputNumberValue>,
    /// 最大值
    max: Option<InputNumberValue>,
    /// 步进值
    step: InputNumberStep,
    /// 精度（小数位数，仅浮点数有效）
    precision: Option<u32>,
    /// 是否禁用
    disabled: bool,
    /// 输入框尺寸
    size: InputNumberSize,
    /// 按钮位置
    controls_position: ControlsPosition,
    /// 占位符
    placeholder: String,
    /// 值改变事件（接收新值，通常需要更新 signal）
    onchange: Option<EventHandler<InputNumberValue>>,
    /// 失去焦点事件
    onblur: Option<EventHandler<FocusEvent>>,
    /// 获得焦点事件
    onfocus: Option<EventHandler<FocusEvent>>,
}

impl Default for InputNumber {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-input-number".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            value: None,
            min: None,
            max: None,
            step: InputNumberStep::Int(1),
            precision: None,
            disabled: false,
            size: InputNumberSize::default(),
            controls_position: ControlsPosition::default(),
            placeholder: String::new(),
            onchange: None,
            onblur: None,
            onfocus: None,
        }
    }
}

impl InputNumber {
    /// 创建一个新的数字输入框实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的数字输入框实例（需要通过 `.value()` 设置 Signal）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new().value(value).onchange(move |v| value.set(v)).to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置当前值的 Signal（必需）
    ///
    /// # 参数
    ///
    /// * `value` - 包含当前值的 Signal
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///      InputNumber::new()
    ///             .value(value)
    ///             .onchange(move |v| value.set(v))
    ///             .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn value(mut self, value: Signal<InputNumberValue>) -> Self {
        self.value = Some(value);
        self
    }

    /// 设置最小值
    ///
    /// # 参数
    ///
    /// * `min` - 最小值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///             .value(value)
    ///             .min(InputNumberValue::Int(0))
    ///             .onchange(move |v| value.set(v))
    ///             .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn min(mut self, min: InputNumberValue) -> Self {
        self.min = Some(min);
        self
    }

    /// 设置最小值（整数）
    ///
    /// # 参数
    ///
    /// * `min` - 最小值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .min_int(0)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn min_int(mut self, min: i64) -> Self {
        self.min = Some(InputNumberValue::Int(min));
        self
    }

    /// 设置最小值（浮点数）
    ///
    /// # 参数
    ///
    /// * `min` - 最小值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use rust_decimal::Decimal;
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Float(Decimal::from(10)));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .min_float(0.0)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn min_float(mut self, min: f64) -> Self {
        self.min = Some(InputNumberValue::Float(
            Decimal::from_f64(min).unwrap_or_default(),
        ));
        self
    }

    /// 设置最大值
    ///
    /// # 参数
    ///
    /// * `max` - 最大值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .max(InputNumberValue::Int(100))
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn max(mut self, max: InputNumberValue) -> Self {
        self.max = Some(max.clone());
        self
    }

    /// 设置最大值（整数）
    ///
    /// # 参数
    ///
    /// * `max` - 最大值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .max_int(100)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn max_int(mut self, max: i64) -> Self {
        self.max = Some(InputNumberValue::Int(max));
        self
    }

    /// 设置最大值（浮点数）
    ///
    /// # 参数
    ///
    /// * `max` - 最大值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use rust_decimal::Decimal;
    /// use rust_decimal::prelude::FromPrimitive;
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Float(Decimal::from(10)));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .max_float(100.5)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn max_float(mut self, max: f64) -> Self {
        self.max = Some(InputNumberValue::Float(
            Decimal::from_f64(max).unwrap_or_default(),
        ));
        self
    }

    /// 设置步进值
    ///
    ///
    /// # 参数
    ///
    /// * `step` - 步进值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, InputNumberStep, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .step(InputNumberStep::Int(1))
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn step(mut self, step: InputNumberStep) -> Self {
        self.step = step;
        self
    }

    /// 设置步进值（整数）
    ///
    /// # 参数
    ///
    /// * `step` - 步进值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .step_int(1)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn step_int(mut self, step: i64) -> Self {
        self.step = InputNumberStep::Int(step);
        self
    }

    /// 设置步进值（浮点数）
    ///
    /// # 参数
    ///
    /// * `step` - 步进值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use rust_decimal::Decimal;
    /// use rust_decimal::prelude::FromPrimitive;
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Float(Decimal::from(10)));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .step_float(0.1)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn step_float(mut self, step: f64) -> Self {
        self.step = InputNumberStep::Float(Decimal::from_f64(step).unwrap_or_default());
        self
    }

    /// 设置精度（小数位数）
    ///
    /// # 参数
    ///
    /// * `precision` - 小数位数
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use rust_decimal::Decimal;
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Float(Decimal::from(10)));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .precision(2)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn precision(mut self, precision: u32) -> Self {
        self.precision = Some(precision);
        self
    }

    /// 设置是否禁用
    ///
    /// # 参数
    ///
    /// * `disabled` - 是否禁用
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .disabled(true)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置输入框尺寸
    ///
    /// # 参数
    ///
    /// * `size` - 输入框尺寸
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, InputNumberSize, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .size(InputNumberSize::Large)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn size(mut self, size: InputNumberSize) -> Self {
        self.size = size;
        self
    }

    /// 设置按钮位置
    ///
    /// # 参数
    ///
    /// * `position` - 按钮位置
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ControlsPosition, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .controls_position(ControlsPosition::Both)
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn controls_position(mut self, position: ControlsPosition) -> Self {
        self.controls_position = position;
        self
    }

    /// 设置占位符
    ///
    /// # 参数
    ///
    /// * `placeholder` - 占位符文本
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .placeholder("请输入数字")
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn placeholder<T: Into<String>>(mut self, placeholder: T) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// 设置值改变事件
    ///
    /// # 参数
    ///
    /// * `handler` - 事件处理器，接收改变后的值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(0));
    ///     InputNumber::new()
    ///             .value(value)
    ///             .onchange(|value| println!("Value: {:?}", value))
    ///             .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn onchange(mut self, handler: impl FnMut(InputNumberValue) + 'static) -> Self {
        self.onchange = Some(EventHandler::new(handler));
        self
    }

    /// 设置值改变事件
    ///
    /// # 参数
    ///
    /// * `handler` - 事件处理器，接收改变后的值
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(0));
    ///     InputNumber::new()
    ///             .value(value)
    ///             .onchange2(EventHandler::new(|value| println!("Value: {:?}", value)))
    ///             .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn onchange2(mut self, handler: EventHandler<InputNumberValue>) -> Self {
        self.onchange = Some(handler);
        self
    }

    /// 设置失去焦点事件
    ///
    /// # 参数
    ///
    /// * `handler` - 事件处理器
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .onblur(|event| println!("Blurred: {:?}", event))
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn onblur(mut self, handler: impl FnMut(FocusEvent) + 'static) -> Self {
        self.onblur = Some(EventHandler::new(handler));
        self
    }

    /// 设置失去焦点事件
    ///
    /// # 参数
    ///
    /// * `handler` - 事件处理器
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .onblur2(EventHandler::new(|event| println!("Blurred: {:?}", event)))
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn onblur2(mut self, handler: EventHandler<FocusEvent>) -> Self {
        self.onblur = Some(handler);
        self
    }

    /// 设置获得焦点事件
    ///
    /// # 参数
    ///
    /// * `handler` - 事件处理器
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .onfocus(|event| println!("Focused: {:?}", event))
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn onfocus(mut self, handler: impl FnMut(FocusEvent) + 'static) -> Self {
        self.onfocus = Some(EventHandler::new(handler));
        self
    }

    /// 设置获得焦点事件
    ///
    /// # 参数
    ///
    /// * `handler` - 事件处理器
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .onfocus2(EventHandler::new(|event| println!("Focused: {:?}", event)))
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn onfocus2(mut self, handler: EventHandler<FocusEvent>) -> Self {
        self.onfocus = Some(handler);
        self
    }
}

/// 便捷方法
impl InputNumber {
    /// 设置为小尺寸输入框
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .as_small()
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn as_small(mut self) -> Self {
        self.size = InputNumberSize::Small;
        self
    }

    /// 设置为中尺寸输入框
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .as_medium()
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn as_medium(mut self) -> Self {
        self.size = InputNumberSize::Medium;
        self
    }

    /// 设置为大尺寸输入框
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .as_large()
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn as_large(mut self) -> Self {
        self.size = InputNumberSize::Large;
        self
    }

    /// 设置为右侧控制按钮位置
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .as_right()
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn as_right(mut self) -> Self {
        self.controls_position = ControlsPosition::Right;
        self
    }

    /// 设置为两侧控制按钮位置
    ///
    /// # 返回值
    ///
    /// 返回修改后的数字输入框实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_blocks_components::{InputNumber, InputNumberValue, ToElement};
    /// use dioxus::core::Mutations;
    ///
    /// let mut dom = VirtualDom::new(|| {
    ///     let mut value = use_signal(|| InputNumberValue::Int(10));
    ///     InputNumber::new()
    ///         .value(value)
    ///         .as_both()
    ///         .onchange(move |v| value.set(v))
    ///         .to_element()
    /// });
    /// let mut mutations = Mutations::default();
    /// dom.rebuild(&mut mutations);
    /// ```
    pub fn as_both(mut self) -> Self {
        self.controls_position = ControlsPosition::Both;
        self
    }
}

impl ToElement for InputNumber {
    fn to_element(&self) -> Element {
        let id = self.id.clone();

        let mut class_names = vec![
            self.class.clone(),
            self.size.to_string(),
            self.controls_position.to_string(),
        ];
        if self.disabled {
            class_names.push("t-input-number--disabled".to_string());
        }
        let class = class_names.join(" ");

        let style = self.style.clone().map(|s| s.to_string());
        let disabled = self.disabled;
        let min = self.min.clone();
        let max = self.max.clone();
        let step = self.step.clone();
        let placeholder = self.placeholder.clone();
        let precision = self.precision;

        // 获取 value signal，如果未设置则使用默认值
        let mut value_signal = self
            .value
            .unwrap_or_else(|| Signal::new(InputNumberValue::Int(0)));
        let is_float_type = value_signal.read().is_float();

        let onchange_handler = self.onchange;
        let onblur_handler = self.onblur;
        let onfocus_handler = self.onfocus;

        // 格式化显示值
        let format_value =
            move |v: &InputNumberValue| -> String { v.to_string_with_precision(precision) };

        // 为各个事件处理器克隆必要的值，避免所有权移动问题
        let min_for_input = min.clone();
        let max_for_input = max.clone();
        let min_for_change = min.clone();
        let max_for_change = max.clone();
        let min_for_decrease = min.clone();
        let max_for_decrease = max.clone();
        let min_for_increase = min.clone();
        let max_for_increase = max.clone();
        let step_for_decrease = step.clone();
        let step_for_increase = step.clone();

        // 为 input HTML 属性克隆必要的值
        let min_for_attr = min.clone();
        let max_for_attr = max.clone();
        let step_for_attr = step.clone();

        rsx! {
            div { id, class, style,
                if self.controls_position == ControlsPosition::Right {
                    // 右侧按钮布局
                    div { class: "t-input-number__wrapper" }
                }

                input {
                    r#type: "number",
                    class: "t-input-number__inner",
                    value: format_value(&value_signal.read()),
                    placeholder,
                    disabled,
                    min: min_for_attr.as_ref().map(|m| m.to_string()),
                    max: max_for_attr.as_ref().map(|m| m.to_string()),
                    step: step_for_attr.as_decimal().to_string(),
                    oninput: move |event: Event<FormData>| {
                        if disabled {
                            return;
                        }
                        let input_value = event.value();

                        if let Some(new_value) = parse_input_value(&input_value, is_float_type) {
                            let clamped_value = apply_bounds(new_value, &min_for_input, &max_for_input);

                            // 更新 signal
                            value_signal.set(clamped_value.clone());

                            // 触发 onchange 回调
                            if let Some(handler) = onchange_handler {
                                handler.call(clamped_value);
                            }
                        }
                    },
                    onchange: move |event: Event<FormData>| {
                        if disabled {
                            return;
                        }
                        let input_value = event.value();

                        if let Some(new_value) = parse_input_value(&input_value, is_float_type) {
                            let clamped_value = apply_bounds(
                                new_value,
                                &min_for_change,
                                &max_for_change,
                            );

                            // 更新 signal
                            value_signal.set(clamped_value.clone());

                            // 触发 onchange 回调
                            if let Some(handler) = onchange_handler {
                                handler.call(clamped_value);
                            }
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
                }

                // 步进按钮
                div { class: "t-input-number__controls",
                    // 减号按钮
                    button {
                        class: "t-input-number__decrease",
                        disabled,
                        onclick: move |event: MouseEvent| {
                            if disabled {
                                event.stop_propagation();
                                return;
                            }
                            let current = value_signal.read().clone();
                            let new_value = calculate_step_value(
                                current,
                                &step_for_decrease,
                                false,
                                &min_for_decrease,
                                &max_for_decrease,
                            );

                            // 更新 signal
                            value_signal.set(new_value.clone());

                            // 触发 onchange 回调
                            if let Some(handler) = onchange_handler {
                                handler.call(new_value);
                            }
                        },
                        svg {
                            "viewBox": "0 0 1024 1024",
                            "width": "1em",
                            "height": "1em",
                            path { "d": "M960 704L512 256l-448 448z" }
                        }
                    }

                    // 加号按钮
                    button {
                        class: "t-input-number__increase",
                        disabled,
                        onclick: move |event: MouseEvent| {
                            if disabled {
                                event.stop_propagation();
                                return;
                            }
                            let current = value_signal.read().clone();
                            let new_value = calculate_step_value(
                                current,
                                &step_for_increase,
                                true,
                                &min_for_increase,
                                &max_for_increase,
                            );

                            // 更新 signal
                            value_signal.set(new_value.clone());

                            // 触发 onchange 回调
                            if let Some(handler) = onchange_handler {
                                handler.call(new_value);
                            }
                        },
                        svg {
                            "viewBox": "0 0 1024 1024",
                            "width": "1em",
                            "height": "1em",
                            path { "d": "M64 320l448 448 448-448z" }
                        }

                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::prelude::FromPrimitive;

    use super::*;

    #[test]
    fn test_input_number_size() {
        let mut dom =
            VirtualDom::new(|| InputNumber::new().size(InputNumberSize::Large).to_element());
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("t-input-number--large"));
    }

    #[test]
    fn test_input_number_controls_position() {
        let mut dom = VirtualDom::new(|| {
            InputNumber::new()
                .controls_position(ControlsPosition::Both)
                .to_element()
        });
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("t-input-number--controls-both"));
    }

    #[test]
    fn test_input_number_disabled() {
        let mut dom = VirtualDom::new(|| InputNumber::new().disabled(true).to_element());
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("t-input-number--disabled"));
    }

    #[test]
    fn test_parse_input_value_int() {
        assert_eq!(
            parse_input_value("123", false),
            Some(InputNumberValue::Int(123))
        );
        assert_eq!(parse_input_value("abc", false), None);
    }

    #[test]
    fn test_parse_input_value_float() {
        let expected = InputNumberValue::Float(Decimal::from_f64(12.34).unwrap());
        assert_eq!(parse_input_value("12.34", true), Some(expected));
        assert_eq!(parse_input_value("abc", true), None);
    }

    #[test]
    fn test_apply_bounds() {
        let value = InputNumberValue::Int(50);
        let min = Some(InputNumberValue::Int(10));
        let max = Some(InputNumberValue::Int(100));

        // 在范围内
        assert_eq!(apply_bounds(value.clone(), &min, &max), value);

        // 小于最小值
        assert_eq!(
            apply_bounds(InputNumberValue::Int(5), &min, &max),
            InputNumberValue::Int(10)
        );

        // 大于最大值
        assert_eq!(
            apply_bounds(InputNumberValue::Int(150), &min, &max),
            InputNumberValue::Int(100)
        );
    }

    #[test]
    fn test_float() {
        let f_decimal = Decimal::from_str("12.34").unwrap();
        println!("f_decimal: {f_decimal:?}  {f_decimal:.20}");

        let f_decimal = Decimal::from_f64(12.34).unwrap();
        println!("f_decimal: {f_decimal:?}  {f_decimal:.20}");

        let f: f64 = 12.34;
        println!("f: {f:?}  {f:.20}");
    }
}
