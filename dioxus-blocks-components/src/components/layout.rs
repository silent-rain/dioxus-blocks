//! Layout 布局组件
//!
//! 提供行（Row）和列（Col）布局组件，类似于 Element Plus 的 Layout 组件。
//! 支持响应式布局、间距、对齐等配置。
//!
//! # 示例
//!
//! ## 基础使用
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Col, Row, ToElement, Text};
//!
//! #[component]
//! fn App() -> Element {
//!     Row::new(vec![
//!         Col::new(Text::new("左侧")),
//!         Col::new(Text::new("右侧")),
//!     ])
//!     .to_element()
//! }
//! ```
//!
//! ## 列间距
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Col, Row, ToElement, Text};
//!
//! #[component]
//! fn App() -> Element {
//!     Row::new(vec![
//!         Col::new(Text::new("1")),
//!         Col::new(Text::new("2")),
//!         Col::new(Text::new("3")),
//!     ])
//!     .gutter(20)
//!     .to_element()
//! }
//! ```
use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, ToElement};

/// 列宽度枚举
///
/// 定义列的宽度，支持固定值、百分比和自动计算。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColSpan {
    /// 自动宽度
    Auto,
    /// 固定宽度（24等分制）
    Span(u8),
    /// 百分比宽度（1-100）
    Percent(u8),
}

impl std::fmt::Display for ColSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColSpan::Auto => write!(f, "auto"),
            ColSpan::Span(n) => write!(f, "{}", n),
            ColSpan::Percent(n) => write!(f, "{}%", n),
        }
    }
}

impl Default for ColSpan {
    fn default() -> Self {
        ColSpan::Span(24)
    }
}

/// 对齐方式枚举
///
/// 定义Flex布局的对齐方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Justify {
    /// 默认对齐
    #[default]
    Start,
    /// 居中对齐
    Center,
    /// 结束对齐
    End,
    /// 两端对齐
    SpaceBetween,
    /// 分散对齐（首尾不留空）
    SpaceAround,
    /// 分散对齐（首尾留半空）
    SpaceEvenly,
}

impl std::fmt::Display for Justify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Justify::Start => write!(f, "flex-start"),
            Justify::Center => write!(f, "center"),
            Justify::End => write!(f, "flex-end"),
            Justify::SpaceBetween => write!(f, "space-between"),
            Justify::SpaceAround => write!(f, "space-around"),
            Justify::SpaceEvenly => write!(f, "space-evenly"),
        }
    }
}

/// Row 容器组件结构体
///
/// 提供一个可自定义的行容器，使用 Flexbox 布局。
#[derive(Debug, Clone, ComponentBase)]
pub struct Row {
    /// 行的唯一标识符
    id: Option<String>,
    /// 行的CSS类名
    class: String,
    /// 行的内联样式
    style: Option<Style>,
    /// 行的子元素列表, 当前组件不支持 childrens 子元素
    childrens: Vec<Rc<dyn ToElement>>,
    /// 行的点击事件
    onclick: Option<EventHandler<MouseEvent>>,

    /// 列
    cols: Vec<Col>,
    /// 列间距（通过 Col padding 实现）
    gutter: usize,
    /// 主轴对齐方式
    justify: Justify,
    /// 交叉轴对齐方式
    align_items: String,
    /// 是否垂直布局
    vertical: bool,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-row".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            cols: Vec::new(),
            gutter: 0,
            justify: Justify::default(),
            align_items: "stretch".to_string(),
            vertical: false,
        }
    }
}

impl ToElement for Row {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let mut style = self
            .style
            .clone()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let onclick_handler = self.onclick;

        // Flexbox 布局样式
        style.push_str("display: flex;");
        // 使用 border-box 确保盒模型一致
        style.push_str("box-sizing: border-box;");
        style.push_str(&format!(
            "flex-direction: {};",
            if self.vertical { "column" } else { "row" }
        ));

        style.push_str(&format!("justify-content: {};", self.justify));
        style.push_str(&format!("align-items: {};", self.align_items));

        let childs = self
            .cols
            .clone()
            .into_iter()
            .map(|child| child.with_gutter(self.gutter).to_element())
            .collect::<Vec<Element>>();

        // 渲染子元素
        let childrens = self.childrens_to_element();

        rsx! {
            div {
                id,
                class: self.class.clone(),
                style,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
                for child in childs {
                    {child}
                }
                {childrens}
            }
        }
    }
}

impl Row {
    /// 创建一个新的行容器实例
    ///
    /// # 参数
    ///
    /// * `cols` - 子元素列表
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的行容器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Row, Col, ToElement, Text};
    /// let row = Row::new(vec![
    ///     Col::new(Text::new("左侧")),
    ///     Col::new(Text::new("右侧")),
    /// ]);
    /// ```
    pub fn new(cols: Vec<Col>) -> Self {
        Self {
            cols,
            ..Default::default()
        }
    }

    /// 添加列
    ///
    /// # 参数
    ///
    /// * `component` - 要添加的组件
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Row, Col, ToElement, Text};
    /// let row = Row::new(vec![]).col(Text::new("内容"));
    /// ```
    pub fn col<T>(mut self, component: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.cols.push(Col::new(component));
        self
    }

    /// 添加列
    ///
    /// # 参数
    ///
    /// * `components` - 要添加的组件列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Row, Col, ToElement, Text};
    /// let row = Row::new(vec![]).cols(vec![Text::new("内容1"), Text::new("内容2")]);
    /// ```
    pub fn cols<T>(mut self, components: Vec<T>) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.cols.extend(components.into_iter().map(Col::new));
        self
    }

    /// 添加列
    ///
    /// # 参数
    ///
    /// * `cols` - 要添加的列列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Row, Col, ToElement, Text};
    /// let row = Row::default().cols2(vec![Col::new(Text::new("内容1")), Col::new(Text::new("内容2"))]);
    /// ```
    pub fn cols2(mut self, cols: Vec<Col>) -> Self {
        self.cols.extend(cols);
        self
    }

    /// 设置列间距（Gutter）
    ///
    /// 类似于 Element Plus 的 gutter，通过 Col padding 实现间距，不会超出容器边界。
    ///
    /// # 参数
    ///
    /// * `gutter` - 间距值（像素）
    ///
    /// # 返回值
    ///
    /// 返回修改后的行容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Row, Col, ToElement, Text};
    /// Row::new(vec![
    ///     Col::new(Text::new("1")),
    ///     Col::new(Text::new("2")),
    /// ]).gutter(20);
    /// ```
    pub fn gutter(mut self, gutter: usize) -> Self {
        self.gutter = gutter;
        self
    }

    /// 设置主轴对齐方式
    ///
    /// # 参数
    ///
    /// * `justify` - 对齐方式
    ///
    /// # 返回值
    ///
    /// 返回修改后的行容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Justify, Row};
    /// Row::default().justify(Justify::Center);
    /// ```
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// 设置交叉轴对齐方式
    ///
    /// # 参数
    ///
    /// * `align` - 对齐方式
    ///
    /// # 返回值
    ///
    /// 返回修改后的行容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Row;
    /// Row::default().align_items("center");
    /// ```
    pub fn align_items<T: Into<String>>(mut self, align: T) -> Self {
        self.align_items = align.into();
        self
    }

    /// 设置为垂直布局
    ///
    /// # 返回值
    ///
    /// 返回修改后的行容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Row;
    /// Row::default().vertical();
    /// ```
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// 设置为水平布局
    ///
    /// # 返回值
    ///
    /// 返回修改后的行容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Row;
    /// Row::default().horizontal();
    /// ```
    pub fn horizontal(mut self) -> Self {
        self.vertical = false;
        self
    }
}

/// 列容器组件结构体
///
/// 提供一个可自定义的列容器，支持灵活的宽度配置。
#[derive(Debug, Clone, ComponentBase)]
pub struct Col {
    /// 列的唯一标识符
    id: Option<String>,
    /// 列的CSS类名
    class: String,
    /// 列的内联样式
    style: Option<Style>,
    /// 列的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 列的点击事件
    onclick: Option<EventHandler<MouseEvent>>,

    /// 列的宽度
    span: ColSpan,
    /// 列的偏移量（24等分制）
    offset: u8,
    gutter: usize,
}

impl Default for Col {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-col".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            span: ColSpan::default(),
            offset: 0,
            gutter: 0,
        }
    }
}

impl ToElement for Col {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let mut style = self
            .style
            .clone()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();

        // Flexbox 布局样式
        style.push_str("display: flex;");
        // 使用 border-box 使 padding 包含在宽度内
        style.push_str("box-sizing: border-box;");

        // 根据 span 类型计算 flex 属性
        match self.span {
            ColSpan::Span(n) => {
                // 将 24 等分转换为百分比: span / 24 * 100
                // 保留小数精度，避免四舍五入导致总宽度超出 100%
                let percent = n as f64 / 24.0 * 100.0;
                style.push_str(&format!("flex: 0 0 {}%;", percent));
            }
            ColSpan::Percent(p) => {
                style.push_str(&format!("flex: 0 0 {}%;", p));
            }
            ColSpan::Auto => {
                style.push_str("flex: 1 1 auto;");
            }
        }

        // 偏移量（margin-left）
        if self.offset > 0 {
            let offset_percent = self.offset as f64 / 24.0 * 100.0;
            style.push_str(&format!("margin-left: {}%;", offset_percent));
        }

        // 通过 CSS 变量从父级 Row 读取 gutter 值
        if self.gutter != 0 {
            let gutter_half = self.gutter as f64 / 2.0;
            style.push_str(&format!("padding-left: {}px;", gutter_half));
            style.push_str(&format!("padding-right: {}px;", gutter_half));
        }

        rsx! {
            div {
                id,
                class: self.class.clone(),
                style,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
                {childrens}
            }
        }
    }
}

impl Col {
    /// 创建一个新的列容器实例
    ///
    /// # 参数
    ///
    /// * `component` - 子元素
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的列容器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Col, ToElement, Text};
    /// let col = Col::new(Text::new("内容"));
    /// ```
    pub fn new<T>(component: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        Self {
            childrens: vec![Rc::new(component)],
            ..Default::default()
        }
    }

    /// 设置列的宽度（24等分制）
    ///
    /// # 参数
    ///
    /// * `span` - 宽度值（1-24）
    ///
    /// # 返回值
    ///
    /// 返回修改后的列容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Col;
    /// Col::default().span(12);
    /// ```
    pub fn span(mut self, span: u8) -> Self {
        self.span = ColSpan::Span(span);
        self
    }

    /// 设置列的宽度为百分比
    ///
    /// # 参数
    ///
    /// * `percent` - 百分比值（1-100）
    ///
    /// # 返回值
    ///
    /// 返回修改后的列容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Col;
    /// Col::default().percent(50);
    /// ```
    pub fn percent(mut self, percent: u8) -> Self {
        self.span = ColSpan::Percent(percent);
        self
    }

    /// 设置列的宽度为自动
    ///
    /// # 返回值
    ///
    /// 返回修改后的列容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Col;
    /// Col::default().auto();
    /// ```
    pub fn auto(mut self) -> Self {
        self.span = ColSpan::Auto;
        self
    }

    /// 设置列的偏移量（24等分制）
    ///
    /// # 参数
    ///
    /// * `offset` - 偏移值（0-24）
    ///
    /// # 返回值
    ///
    /// 返回修改后的列容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Col;
    /// Col::default().offset(6);
    /// ```
    pub fn offset(mut self, offset: u8) -> Self {
        self.offset = offset;
        self
    }

    pub(crate) fn with_gutter(mut self, gutter: usize) -> Self {
        self.gutter = gutter;
        self
    }
}
