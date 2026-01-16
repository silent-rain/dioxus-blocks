//! Link 组件
//!
//! 提供一个可自定义的链接组件，支持路由跳转、字符串路径跳转。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Link, ToElement};
//!
//! # let mut dom = VirtualDom::new(|| {
//!
//!     #[component]
//!     fn App() -> Element {
//!         Link::new(NavigationTarget::<String>::from("/home")).new_tab(true).to_element();
//!
//!         Link::new("/home").text("Home").to_element();
//!
//!         Link::default().text("Home").to("/home").to_element();
//!
//!         Link::new(NavigationTarget::<String>::from("/home")).new_tab(true).to_element()
//!     }
//!     # App()
//! # });
//! # dom.rebuild(&mut dioxus_core::NoOpMutations);
//! ```

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 链接组件结构体
///
/// 提供一个可自定义的链接，支持多种跳转方式、样式和子元素。
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
    /// 是否在新标签页打开
    new_tab: bool,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_link".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            to: NavigationTarget::from(""),
            text: String::new(),
            new_tab: false,
        }
    }
}

impl ToElement for Link {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let class = self.class.clone();
        let style = self.style.clone().map(|s| s.to_string());
        let text = self.text.clone();
        let childrens = self.childrens_to_element();
        let to = self.to.clone();

        rsx! {
            Link {
                id,
                class,
                style,
                to,
                new_tab: self.new_tab,
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
}
