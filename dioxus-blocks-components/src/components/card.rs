//! Card 组件
//!
//! 提供一个可自定义的卡片组件，支持头部、主体和底部内容，以及阴影效果和边框样式。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Card, CardShadow, ToElement, Text, Button};
//!
//! #[component]
//! fn App() -> Element {
//!     Card::new()
//!         .header(Text::new("卡片标题"))
//!         .body(Text::new("卡片内容"))
//!         .footer(Button::new().text("按钮"))
//!         .shadow(CardShadow::Hover)
//!         .to_element()
//! }
//! ```
use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 卡片阴影效果枚举
///
/// 定义卡片在不同状态下的阴影效果
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CardShadow {
    /// 始终显示阴影
    #[default]
    Always,
    /// 仅在悬停时显示阴影
    Hover,
    /// 从不显示阴影
    Never,
}

impl CardShadow {
    /// 获取阴影效果对应的 CSS 类名
    ///
    /// # 返回值
    ///
    /// 返回与阴影效果对应的 CSS 类名字符串
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::CardShadow;
    /// let class = CardShadow::Hover.as_class();
    /// ```
    pub fn as_class(&self) -> &'static str {
        match self {
            CardShadow::Always => "t_card-shadow-always",
            CardShadow::Hover => "t_card-shadow-hover",
            CardShadow::Never => "t_card-shadow-never",
        }
    }
}

/// 卡片组件结构体
///
/// 提供一个可自定义的卡片容器，支持头部、主体和底部内容，以及阴影效果和边框样式。
#[derive(Debug, Clone, ComponentBase)]
pub struct Card {
    /// 卡片的唯一标识符
    id: Option<String>,
    /// 卡片的CSS类名
    class: String,
    /// 卡片的内联样式
    style: Option<Style>,
    /// 卡片的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 卡片点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 卡片头部内容，可选
    header: Option<Rc<dyn ToElement>>,
    /// 卡片底部内容，可选
    footer: Option<Rc<dyn ToElement>>,

    /// 卡片阴影效果，默认为 Always
    /// Shadow control: always, hover, never
    shadow: CardShadow,
    /// 头部和主体之间是否有分隔线，默认为 true
    /// Divider between header and body: true/false
    header_divider: bool,
    /// 是否显示边框
    border: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_card".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            header: None,
            footer: None,
            shadow: CardShadow::default(),
            header_divider: true,
            border: false,
        }
    }
}

impl ToElement for Card {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let mut class = self.class.clone();
        let style = self.style.clone().map(|s| s.to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();

        // 添加阴影效果
        class.push_str(&format!(" {}", self.shadow.as_class()));

        // 添加边框
        if self.border {
            class.push_str(" t_card-border");
        } else {
            class.push_str(" t_card-no-border");
        }

        rsx! {
            div {
                id,
                class,
                style,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },

                // Header section
                if let Some(header) = &self.header {
                    div { class: if self.header_divider { "t_card-header t_card-header-divider" } else { "t_card-header" },
                        {header.to_element()}
                    }
                }

                // Body section
                div { class: "t_card-body", {childrens} }

                // Footer section
                if let Some(footer) = &self.footer {
                    div { class: "t_card-footer", {footer.to_element()} }
                }
            }
        }
    }
}

impl Card {
    /// 创建一个新的卡片实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的卡片实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Card;
    /// let card = Card::new();
    /// ```
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置卡片的头部内容
    ///
    /// # 参数
    ///
    /// * `header` - 要设置的头部内容，任何实现了 `ToElement + Clone + 'static` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的卡片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus::core::Mutations;
    /// # use dioxus_blocks_components::{Card, Text, ToElement};
    /// # let mut dom = VirtualDom::new(|| {
    ///     Card::new()
    ///         .header(Text::h3("卡片标题"))
    ///         .to_element()
    /// # });
    /// # let mut mutations = Mutations::default();
    /// # dom.rebuild(&mut mutations);
    /// ```
    pub fn header<T>(mut self, header: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.header = Some(Rc::new(header));
        self
    }

    /// 设置卡片的主体内容
    ///
    /// # 参数
    ///
    /// * `body` - 要设置的主体内容，任何实现了 `Into<Element>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的卡片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Card, Text};
    /// Card::new().body(Text::p("卡片内容"));
    /// ```
    pub fn body<T>(mut self, component: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.childrens.push(Rc::new(component));
        self
    }

    /// 设置卡片的底部内容
    ///
    /// # 参数
    ///
    /// * `footer` - 要设置的底部内容，任何实现了 `ToElement + Clone + 'static` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的卡片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus_blocks_components::{Card, Text, ToElement};
    /// Card::new().footer(Text::new("确定"));
    /// ```
    pub fn footer<T>(mut self, footer: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        self.footer = Some(Rc::new(footer));
        self
    }

    /// 设置卡片的阴影效果
    ///
    /// # 参数
    ///
    /// * `shadow` - CardShadow 枚举值：Always（始终显示）、Hover（悬停时显示）或 Never（从不显示）
    ///
    /// # 返回值
    ///
    /// 返回修改后的卡片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Card, CardShadow};
    /// Card::new().shadow(CardShadow::Hover);
    /// ```
    pub fn shadow(mut self, shadow: CardShadow) -> Self {
        self.shadow = shadow.clone();
        self
    }

    /// 设置卡片是否有边框
    ///
    /// # 参数
    ///
    /// * `border` - 布尔值：true 表示有边框，false 表示无边框
    ///
    /// # 返回值
    ///
    /// 返回修改后的卡片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Card;
    /// Card::new().border(false);
    /// ```
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    /// 设置头部和主体之间是否有分隔线
    ///
    /// # 参数
    ///
    /// * `divider` - 布尔值：true 表示有分隔线，false 表示无分隔线
    ///
    /// # 返回值
    ///
    /// 返回修改后的卡片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Card;
    /// Card::new().header_divider(false);
    /// ```
    pub fn header_divider(mut self, divider: bool) -> Self {
        self.header_divider = divider;
        self
    }
}
