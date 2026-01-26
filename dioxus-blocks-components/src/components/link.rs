//! Link 组件
//!
//! 提供一个可自定义的链接组件，支持路由跳转、字符串路径跳转、多种类型和下划线样式。
//!
//! # 示例
//!
//! ## 基础使用
//!
//! ```rust
//! use dioxus_blocks_components::Link;
//!
//! let link = Link::new("/home").text("Home");
//! ```
//!
//! ## 类型链接
//!
//! ```rust
//! use dioxus_blocks_components::Link;
//!
//! let link = Link::new("/home").text("Primary Link").primary();
//! ```
//!
//! ## 下划线样式
//!
//! ```rust
//! use dioxus_blocks_components::{Link, LinkUnderline};
//!
//! let link = Link::new("/home")
//!     .text("Link")
//!     .underline(LinkUnderline::Hover);
//! ```

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 链接类型枚举
///
/// 定义链接的不同类型，每种类型有不同的颜色主题。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkType {
    /// 默认链接
    #[default]
    Default,
    /// 主要链接
    Primary,
    /// 成功链接
    Success,
    /// 信息链接
    Info,
    /// 警告链接
    Warning,
    /// 危险链接
    Danger,
}

impl std::fmt::Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkType::Default => write!(f, ""),
            LinkType::Primary => write!(f, "t-link--primary"),
            LinkType::Success => write!(f, "t-link--success"),
            LinkType::Info => write!(f, "t-link--info"),
            LinkType::Warning => write!(f, "t-link--warning"),
            LinkType::Danger => write!(f, "t-link--danger"),
        }
    }
}

/// 下划线样式枚举
///
/// 定义链接的下划线显示方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkUnderline {
    /// 总是显示下划线
    Always,
    /// 鼠标悬停时显示下划线
    #[default]
    Hover,
    /// 从不显示下划线
    Never,
}

impl std::fmt::Display for LinkUnderline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkUnderline::Always => write!(f, "t-link--underline-always"),
            LinkUnderline::Hover => write!(f, "t-link--underline-hover"),
            LinkUnderline::Never => write!(f, "t-link--underline-never"),
        }
    }
}

/// 链接组件结构体
///
/// 提供一个可自定义的链接，支持多种跳转方式、样式、类型和子元素。
#[derive(Debug, Clone, ComponentBase)]
pub struct Link {
    /// 链接的唯一标识符
    id: Option<String>,
    /// 链接的CSS类名
    class: String,
    /// 链接的内联样式
    style: Option<Style>,
    /// 链接的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 链接的点击事件
    onclick: Option<EventHandler<MouseEvent>>,

    /// 链接目标
    to: NavigationTarget,
    /// 链接显示的文本内容
    text: String,
    /// 链接类型
    link_type: LinkType,
    /// 下划线样式
    underline: LinkUnderline,
    /// 是否禁用
    disabled: bool,
    /// 是否在新标签页打开
    new_tab: bool,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            id: None,
            class: "t-link".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            to: NavigationTarget::from(""),
            text: String::new(),
            link_type: LinkType::default(),
            underline: LinkUnderline::default(),
            disabled: false,
            new_tab: false,
        }
    }
}

impl ToElement for Link {
    fn to_element(&self) -> Element {
        // 构建完整的 class 列表
        let mut class_names = vec![
            self.class.clone(),
            self.link_type.to_string(),
            self.underline.to_string(),
        ];

        // 添加状态 class
        if self.disabled {
            class_names.push("t-link--disabled".to_string());
        }

        let id = self.id.clone();
        let class = class_names.join(" ");
        let style = self.style.clone().map(|s| s.to_string());
        let text = self.text.clone();
        let childrens = self.childrens_to_element();
        let to = self.to.clone();
        let onclick_handler = self.onclick;

        rsx! {
            Link {
                id,
                class,
                style,
                to,
                new_tab: self.new_tab,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
                {childrens}
                {text}
            }
        }
    }
}

/// Link 组件实现
impl Link {
    /// 创建一个新的链接实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的链接实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::NavigationTarget;
    /// # use dioxus_blocks_components::{Link, ToElement};
    /// # use dioxus::prelude::*;
    ///
    /// # let mut dom = VirtualDom::new(|| {
    ///
    ///     #[component]
    ///     fn App() -> Element {
    ///         Link::new(NavigationTarget::<String>::from("/home")).text("点击跳转").to_element()
    ///     }
    ///     # App()
    ///
    /// # });
    /// # dom.rebuild(&mut dioxus_core::NoOpMutations);
    /// ```
    pub fn new<P: Into<NavigationTarget>>(to: P) -> Self {
        Self {
            to: to.into(),
            ..Default::default()
        }
    }

    /// 设置链接目标 - 支持字符串路径
    ///
    /// # 参数
    ///
    /// * `path` - 链接目标路径
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::NavigationTarget;
    /// # use dioxus_blocks_components::{Link, ToElement};
    /// # use dioxus::prelude::*;
    ///
    /// # let mut dom = VirtualDom::new(|| {
    ///
    ///     #[component]
    ///     fn App() -> Element {
    ///         Link::default().text("点击跳转").to("/home").to_element()
    ///     }
    ///     # App()
    ///
    /// # });
    /// # dom.rebuild(&mut dioxus_core::NoOpMutations);
    /// ```
    pub fn to<T: Into<NavigationTarget>>(mut self, path: T) -> Self {
        self.to = path.into();
        self
    }

    /// 设置链接显示的文本
    ///
    /// # 参数
    ///
    /// * `text` - 要显示的文本内容
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::NavigationTarget;
    /// # use dioxus_blocks_components::{Link, ToElement};
    /// # use dioxus::prelude::*;
    ///
    /// # let mut dom = VirtualDom::new(|| {
    ///
    ///     #[component]
    ///     fn App() -> Element {
    ///         Link::new(NavigationTarget::<String>::from("/home")).text("点击跳转").to_element()
    ///     }
    ///     # App()
    ///
    /// # });
    /// # dom.rebuild(&mut dioxus_core::NoOpMutations);
    /// ```
    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }

    /// 设置是否在新标签页打开
    ///
    /// # 参数
    ///
    /// * `new_tab` - 布尔值，true 表示在新标签页打开
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::NavigationTarget;
    /// # use dioxus_blocks_components::{Link, ToElement};
    /// # use dioxus::prelude::*;
    ///
    /// # let mut dom = VirtualDom::new(|| {
    ///
    ///     #[component]
    ///     fn App() -> Element {
    ///         Link::new(NavigationTarget::<String>::from("/home")).new_tab(true).to_element()
    ///     }
    ///     # App()
    /// # });
    /// # dom.rebuild(&mut dioxus_core::NoOpMutations);
    /// ```
    pub fn new_tab(mut self, new_tab: bool) -> Self {
        self.new_tab = new_tab;
        self
    }

    /// 设置是否禁用链接
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Disabled Link").disabled();
    /// ```
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// 类型便捷方法
impl Link {
    /// 设置为主要链接
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Primary Link").as_primary();
    /// ```
    pub fn as_primary(mut self) -> Self {
        self.link_type = LinkType::Primary;
        self
    }

    /// 设置为成功链接
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Success Link").as_success();
    /// ```
    pub fn as_success(mut self) -> Self {
        self.link_type = LinkType::Success;
        self
    }

    /// 设置为信息链接
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Info Link").as_info();
    /// ```
    pub fn as_info(mut self) -> Self {
        self.link_type = LinkType::Info;
        self
    }

    /// 设置为警告链接
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Warning Link").as_warning();
    /// ```
    pub fn as_warning(mut self) -> Self {
        self.link_type = LinkType::Warning;
        self
    }

    /// 设置为危险链接
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Danger Link").as_danger();
    /// ```
    pub fn as_danger(mut self) -> Self {
        self.link_type = LinkType::Danger;
        self
    }

    // ==================== 下划线样式便捷方法 ====================

    /// 设置总是显示下划线
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Link").as_underline_always();
    /// ```
    pub fn as_underline_always(mut self) -> Self {
        self.underline = LinkUnderline::Always;
        self
    }

    /// 设置悬停时显示下划线
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Link").as_underline_hover();
    /// ```
    pub fn as_underline_hover(mut self) -> Self {
        self.underline = LinkUnderline::Hover;
        self
    }

    /// 设置从不显示下划线
    ///
    /// # 返回值
    ///
    /// 返回修改后的链接实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Link;
    /// Link::new("/home").text("Link").as_underline_never();
    /// ```
    pub fn as_underline_never(mut self) -> Self {
        self.underline = LinkUnderline::Never;
        self
    }
}
