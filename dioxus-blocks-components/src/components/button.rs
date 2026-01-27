//! Button 组件
//!
//! 提供一个可自定义的按钮组件，支持多种类型、样式、形状和尺寸。
//!
//! # 示例
//!
//! ## 基础使用
//!
//! ```rust
//! use dioxus_blocks_components::{Button, ButtonType};
//!
//! let button = Button::new()
//!     .text("Primary")
//!     .btn_type(ButtonType::Primary);
//! ```
//!
//! ## 椭圆按钮
//!
//! ```rust
//! use dioxus_blocks_components::{Button, ButtonType, ButtonShape};
//!
//! let button = Button::new()
//!     .text("Round")
//!     .btn_type(ButtonType::Primary)
//!     .shape(ButtonShape::Round);
//! ```
//!
//! ## 圆形按钮
//!
//! ```rust
//! use dioxus_blocks_components::{Button, ButtonType, ButtonShape};
//!
//! let button = Button::new()
//!     .shape(ButtonShape::Circle)
//!     .btn_type(ButtonType::Primary)
//!     .text("P");
//! ```
//!
//! ## 链接按钮
//!
//! ```rust
//! use dioxus_blocks_components::{Button, ButtonType, ButtonShape};
//!
//! let button = Button::new()
//!     .text("Link Button")
//!     .shape(ButtonShape::Link);
//! ```
//!
//! ## 文字按钮
//!
//! ```rust
//! use dioxus_blocks_components::{Button, ButtonType, ButtonShape};
//!
//! let button = Button::new()
//!     .text("Text Button")
//!     .shape(ButtonShape::Text);
//! ```
//!
//! ## 朴素按钮
//!
//! ```rust
//! use dioxus_blocks_components::{Button, ButtonType, ButtonShape};
//!
//! let button = Button::new()
//!     .text("Plain Button")
//!     .btn_type(ButtonType::Primary)
//!     .shape(ButtonShape::Plain);
//! ```
use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 按钮类型枚举
///
/// 定义按钮的不同类型，每种类型有不同的颜色主题。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonType {
    /// 默认按钮
    #[default]
    Default,
    /// 主要按钮
    Primary,
    /// 成功按钮
    Success,
    /// 信息按钮
    Info,
    /// 警告按钮
    Warning,
    /// 危险按钮
    Danger,
}

impl std::fmt::Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Default => write!(f, "t-button--default"),
            ButtonType::Primary => write!(f, "t-button--primary"),
            ButtonType::Success => write!(f, "t-button--success"),
            ButtonType::Info => write!(f, "t-button--info"),
            ButtonType::Warning => write!(f, "t-button--warning"),
            ButtonType::Danger => write!(f, "t-button--danger"),
        }
    }
}

/// 按钮形状枚举
///
/// 定义按钮的圆角风格和类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonShape {
    /// 默认圆角
    #[default]
    Default,
    /// 朴素按钮样式
    Plain,
    /// 椭圆
    Round,
    /// 圆形
    Circle,
    /// 链接按钮样式
    Link,
    /// 文字按钮样式
    Text,
}

impl std::fmt::Display for ButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonShape::Default => write!(f, ""),
            ButtonShape::Plain => write!(f, "t-button--plain"),
            ButtonShape::Round => write!(f, "t-button--round"),
            ButtonShape::Circle => write!(f, "t-button--circle"),
            ButtonShape::Link => write!(f, "t-button--link"),
            ButtonShape::Text => write!(f, "t-button--text"),
        }
    }
}

/// 按钮尺寸枚举
///
/// 定义按钮的大小。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// 中等尺寸
    #[default]
    Medium,
    /// 小尺寸
    Small,
    /// 大尺寸
    Large,
}

impl std::fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonSize::Medium => write!(f, ""),
            ButtonSize::Small => write!(f, "t-button--small"),
            ButtonSize::Large => write!(f, "t-button--large"),
        }
    }
}

/// 按钮组件结构体
///
/// 提供一个可自定义的按钮，支持多种类型、样式、形状和尺寸。
#[derive(Debug, Clone, ComponentBase)]
pub struct Button {
    /// 按钮的唯一标识符
    id: Option<String>,
    /// 按钮的CSS类名
    class: String,
    /// 按钮的内联样式
    style: Option<Style>,
    /// 按钮的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 按钮点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 按钮显示的文本内容
    text: String,
    /// 按钮类型
    btn_type: ButtonType,
    /// 按钮形状
    shape: ButtonShape,
    /// 按钮尺寸
    size: ButtonSize,
    /// 是否禁用
    disabled: bool,
    /// 是否加载中
    loading: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-button".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            text: "Button".to_string(),
            btn_type: ButtonType::default(),
            shape: ButtonShape::default(),
            size: ButtonSize::default(),
            disabled: false,
            loading: false,
        }
    }
}

impl Button {
    /// 创建一个新的按钮实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的按钮实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// let button = Button::new();
    /// ```
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置按钮显示的文本
    ///
    /// # 参数
    ///
    /// * `text` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("点击我");
    /// ```
    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }

    /// 设置按钮类型
    ///
    /// # 参数
    ///
    /// * `btn_type` - 按钮类型，决定按钮的颜色主题
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Button, ButtonType};
    /// Button::new().btn_type(ButtonType::Primary);
    /// ```
    pub fn btn_type(mut self, btn_type: ButtonType) -> Self {
        self.btn_type = btn_type;
        self
    }

    /// 设置按钮形状
    ///
    /// # 参数
    ///
    /// * `shape` - 按钮形状，决定按钮的圆角风格
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Button, ButtonShape};
    /// Button::new().shape(ButtonShape::Round);
    /// ```
    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// 设置按钮尺寸
    ///
    /// # 参数
    ///
    /// * `size` - 按钮尺寸，决定按钮的大小
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Button, ButtonSize};
    /// Button::new().size(ButtonSize::Large);
    /// ```
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// 设置按钮是否禁用
    ///
    /// # 参数
    ///
    /// * `disabled` - 是否禁用按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().disabled(true);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置按钮是否加载中
    ///
    /// # 参数
    ///
    /// * `loading` - 是否显示加载状态
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().loading(true);
    /// ```
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

/// 便捷方法
impl Button {
    /// 设置为主要按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Primary").as_primary();
    /// ```
    pub fn as_primary(mut self) -> Self {
        self.btn_type = ButtonType::Primary;
        self
    }

    /// 设置为成功按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Success").as_success();
    /// ```
    pub fn as_success(mut self) -> Self {
        self.btn_type = ButtonType::Success;
        self
    }

    /// 设置为信息按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Info").as_info();
    /// ```
    pub fn as_info(mut self) -> Self {
        self.btn_type = ButtonType::Info;
        self
    }

    /// 设置为警告按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Warning").as_warning();
    /// ```
    pub fn as_warning(mut self) -> Self {
        self.btn_type = ButtonType::Warning;
        self
    }

    /// 设置为危险按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Danger").as_danger();
    /// ```
    pub fn as_danger(mut self) -> Self {
        self.btn_type = ButtonType::Danger;
        self
    }

    /// 设置为朴素按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Plain").as_primary().as_plain();
    /// ```
    pub fn as_plain(mut self) -> Self {
        self.shape = ButtonShape::Plain;
        self
    }

    /// 设置为椭圆按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Round").as_primary().as_round();
    /// ```
    pub fn as_round(mut self) -> Self {
        self.shape = ButtonShape::Round;
        self
    }

    /// 设置为圆形按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("C").as_primary().as_circle();
    /// ```
    pub fn as_circle(mut self) -> Self {
        self.shape = ButtonShape::Circle;
        self
    }

    /// 设置为链接按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Link").as_link();
    /// ```
    pub fn as_link(mut self) -> Self {
        self.shape = ButtonShape::Link;
        self
    }

    /// 设置为文字按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Text").as_text();
    /// ```
    pub fn as_text(mut self) -> Self {
        self.shape = ButtonShape::Text;
        self
    }

    /// 设置为小尺寸按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Small").as_primary().as_small();
    /// ```
    pub fn as_small(mut self) -> Self {
        self.size = ButtonSize::Small;
        self
    }

    /// 设置为中尺寸按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Medium").as_primary().as_medium();
    /// ```
    pub fn as_medium(mut self) -> Self {
        self.size = ButtonSize::Medium;
        self
    }

    /// 设置为大尺寸按钮
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("Large").as_primary().as_large();
    /// ```
    pub fn as_large(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }
}

impl ToElement for Button {
    fn to_element(&self) -> Element {
        // 构建完整的 class 列表
        let mut class_names = vec![
            self.class.clone(),
            self.btn_type.to_string(), // 添加类型 class
            self.shape.to_string(),    // 添加形状 class
            self.size.to_string(),     // 添加尺寸 class
        ];

        // 添加状态 class
        if self.disabled {
            class_names.push("t-button--disabled".to_string());
        }
        if self.loading {
            class_names.push("t-button--loading".to_string());
        }

        let id = self.id.clone();
        let class = class_names.join(" ");
        let style = self.style.clone().map(|s| s.to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();
        let text = self.text.clone();

        rsx! {
            button {
                id,
                class,
                style,
                disabled: "{self.disabled}",
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
                {text}
                {childrens}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, rc::Rc};

    use dioxus::core::{ElementId, Mutations};
    use dioxus_html::SerializedHtmlEventConverter;

    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new().text("测试按钮");

        // 测试按钮的基本属性
        assert_eq!(button.text, "测试按钮");
        assert!(button.class.contains("t-button"));
    }

    #[test]
    fn test_button_properties() {
        // 测试按钮的基本属性设置
        let button = Button::new().text("测试按钮").class("custom-button");

        assert_eq!(button.text, "测试按钮");
        assert!(button.class.contains("custom-button"));
        assert!(button.class.contains("t-button"));
    }

    #[test]
    fn test_button_default_values() {
        let button = Button::new();

        assert_eq!(button.text, "Button");
        assert_eq!(button.class, "t-button");
    }

    /// 创建虚拟DOM测试
    #[test]
    fn test_with_virtual_dom() {
        fn app() -> Element {
            Button::new()
                .id("test-button")
                .text("Test Button")
                .onclick(|_| {
                    println!("Button clicked again!");
                })
                .to_element()
        }

        let mut dom = VirtualDom::new(app);
        let mut mutations = Mutations::default();
        dom.rebuild(&mut mutations);

        // 第一个真实元素的 id
        // 更稳妥的方法是检查 rebuild_to_vec() 返回的 mutations，找到分配给 button 的 ElementId
        let button_id = ElementId(1);

        // 告诉 dioxus 使用序列化事件转换器
        dioxus::html::set_event_converter(Box::new(SerializedHtmlEventConverter));

        // 构造一个事件
        let payload = PlatformEventData::new(Box::<SerializedMouseData>::default());
        let event = Event::new(Rc::new(payload) as Rc<dyn Any>, true);

        // 触发事件处理器
        dom.runtime().handle_event("click", event, button_id);
    }

    #[test]
    fn test_wrap_with_button() {
        // 创建一个 VirtualDom 作为运行时环境
        let mut dom = VirtualDom::new(|| {
            // 在这个闭包中，我们可以安全地使用需要运行时的组件
            Button::new()
                .text("Test Button")
                .onclick2(EventHandler::new(move |_| {
                    println!("Button clicked!");
                }))
                .onclick(|_| {
                    println!("Button clicked again!");
                })
                .to_element()
        });

        // 重新渲染以使 VirtualDom 初始化完成
        dom.rebuild(&mut dioxus_core::NoOpMutations);

        // 获取渲染结果
        let html = dioxus_ssr::render(&dom);

        // 验证渲染结果
        assert!(html.contains("button"));
        assert!(html.contains("t-button"));
        assert!(html.contains("Test Button"));
    }

    /// 创建运行时上下文测试
    #[test]
    fn test_with_scope_provider() {
        #[derive(PartialEq, Props, Clone)]
        struct SomeProps {
            name: &'static str,
        }

        fn example(cx: SomeProps) -> Element {
            println!("Rendering with name: {}", cx.name);
            Button::new()
                .text(format!("hello {}", cx.name))
                .to_element()
        }

        let mut dom = VirtualDom::new_with_props(example, SomeProps { name: "world" });

        // 重建虚拟DOM
        dom.rebuild(&mut dioxus_core::NoOpMutations);
    }
}
