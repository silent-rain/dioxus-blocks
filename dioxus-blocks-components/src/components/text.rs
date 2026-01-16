//! Text 组件
//!
//! 提供一个可自定义的文本组件，支持多种文本样式、大小、颜色和对齐方式。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Text, ToElement};
//!
//! # let mut dom = VirtualDom::new(|| {
//!
//! #[component]
//! fn App() -> Element {
//!     let text = Text::new("Hello, World!")
//!         .style(|s| s.color("#333333").text_align("center").font_weight("bold"))
//!         .to_element();
//!     rsx! {
//!         {text}
//!     }
//! }
//!
//! # App()
//! # });
//! # dom.rebuild(&mut dioxus_core::NoOpMutations);
//! ```
use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 文本标签
#[derive(Debug, Clone, Default)]
pub enum TextTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    #[default]
    Span,
}

/// 文本组件结构体
///
/// 提供一个可自定义的文本显示组件，支持丰富的文本样式配置。
#[derive(Debug, Clone, ComponentBase)]
pub struct Text {
    /// 文本组件的唯一标识符
    id: Option<String>,
    /// 文本组件的CSS类名
    class: String,
    /// 文本组件的内联样式
    style: Option<Style>,
    /// 文本组件的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 文本点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 文本显示的内容
    content: String,
    /// 文本的标签（如H1, H2, P等），默认为Span
    tag: TextTag,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_text".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            content: "".to_string(),
            tag: TextTag::Span,
        }
    }
}

impl Text {
    /// 创建一个新的文本实例，默认使用 span 标签
    ///
    /// # 返回值
    ///
    /// 返回一个具有span标签的文本实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// let text = Text::new("Hello, World!");
    /// ```
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::Span,
            ..Default::default()
        }
    }

    /// H1 标题
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::h1("Hi, World!");
    /// ```
    pub fn h1<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::H1,
            ..Default::default()
        }
    }

    /// H2 标题
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::h2("Hi, World!");
    /// ```
    pub fn h2<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::H2,
            ..Default::default()
        }
    }

    /// H3 标题
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::h3("Hi, World!");
    /// ```
    pub fn h3<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::H3,
            ..Default::default()
        }
    }

    /// H4 标题
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::h4("Hi, World!");
    /// ```
    pub fn h4<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::H4,
            ..Default::default()
        }
    }

    /// H5 标题
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::h5("Hi, World!");
    /// ```
    pub fn h5<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::H5,
            ..Default::default()
        }
    }

    /// H6 标题
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::h6("Hi, World!");
    /// ```
    pub fn h6<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::H6,
            ..Default::default()
        }
    }

    /// 行内文本（span）
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::span("Hi, World!");
    /// ```
    pub fn span<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::Span,
            ..Default::default()
        }
    }

    /// 段落
    ///
    /// # 参数
    ///
    /// * `content` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的文本实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Text;
    /// Text::p("Hi, World!");
    /// ```
    pub fn p<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            tag: TextTag::P,
            ..Default::default()
        }
    }
}

impl ToElement for Text {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let class = self.class.clone();
        let style = self.style.clone().map(|s| s.to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();
        let content = self.content.clone();

        match self.tag {
            TextTag::H1 => rsx! {
                h1 {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
            TextTag::H2 => rsx! {
                h2 {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
            TextTag::H3 => rsx! {
                h3 {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    for childrens in childrens.iter() {
                        {childrens}
                    }
                }
            },
            TextTag::H4 => rsx! {
                h4 {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
            TextTag::H5 => rsx! {
                h5 {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
            TextTag::H6 => rsx! {
                h6 {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
            TextTag::P => rsx! {
                p {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
            TextTag::Span => rsx! {
                span {
                    id,
                    class,
                    style,
                    onclick: move |event: MouseEvent| {
                        if let Some(handler) = onclick_handler {
                            handler.call(event);
                        }
                    },
                    {content}
                    {childrens}
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = Text::new("测试文本");

        // 测试文本的基本属性
        assert_eq!(text.content, "测试文本");
        assert!(text.class.contains("t_text"));
    }

    #[test]
    fn test_text_properties() {
        // 测试文本的基本属性设置
        let text = Text::new("测试文本").style(|s| {
            s.font_size("16px")
                .color("#333333")
                .text_align("center")
                .font_weight("bold")
        });

        assert_eq!(text.content, "测试文本");
        assert_eq!(
            text.style.map(|v| v.to_string()),
            Some(
                "font-size: 16px; color: #333333; text-align: center; font-weight: bold;"
                    .to_string()
            )
        );
    }
}
