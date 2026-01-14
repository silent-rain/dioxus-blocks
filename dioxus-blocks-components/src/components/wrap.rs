//! Wrap 组件
//!
//! 提供一个空的容器组件，用于包装其他元素，支持自定义样式和布局。
//!
//! # 示例
//!
//! ## 基本用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Wrap, Text, ToElement};
//!
//! # let mut dom = VirtualDom::new(|| {
//!
//! #[component]
//! fn App() -> Element {
//!     let wrap = Wrap::new()
//!         .class("container")
//!         .style(|s| s.custom("padding: 20px; margin: 10px;"))
//!         .children(Text::new("Hello, Wrap!"))
//!         .to_element();
//!     rsx! {
//!         {wrap}
//!     }
//! }
//!
//! # App()
//! # });
//! # dom.rebuild(&mut dioxus_core::NoOpMutations);
//!
//! ```
//! ```
use std::sync::Arc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 容器组件结构体
///
/// 提供一个空的容器，用于包装其他元素，支持丰富的样式配置。
#[derive(Debug, Clone, ComponentBase)]
pub struct Wrap {
    /// 容器组件的唯一标识符
    id: Option<String>,
    /// 容器组件的CSS类名
    class: String,
    /// 容器组件的内联样式
    style: Option<Style>,
    /// 容器组件的子元素列表
    childrens: Vec<Arc<dyn ToElement>>,
    /// 容器组件的点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 是否裸露渲染（不使用 div 包装），默认为 false
    bare: bool,
}

impl Default for Wrap {
    fn default() -> Self {
        Self {
            id: None,
            class: "".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            bare: false,
        }
    }
}

impl Wrap {
    /// 创建一个新的容器实例
    ///
    /// # 返回值
    ///
    /// 返回一个新的容器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Wrap;
    /// let wrap = Wrap::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否裸露渲染（不使用 div 包装）
    ///
    /// # 参数
    ///
    /// * `bare` - 是否裸露渲染，true 为不使用 div 包装，false 为使用 div 包装
    ///
    /// # 返回值
    ///
    /// 返回修改后的容器实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Wrap;
    /// let wrap = Wrap::new().bare(true);
    /// ```
    pub fn bare(mut self, bare: bool) -> Self {
        self.bare = bare;
        self
    }
}

impl ToElement for Wrap {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let class = self.class.clone();
        let style = self.style.clone().map(|s| s.to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();

        if !self.bare {
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
                    {childrens}
                }
            }
        } else {
            childrens
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Text;

    use super::*;

    #[test]
    fn test_wrap_creation() {
        let wrap = Wrap::new();

        assert!(wrap.class.contains("t_wrap"));
        assert!(wrap.childrens.is_empty());
    }

    #[test]
    fn test_wrap_properties() {
        let wrap = Wrap::new()
            .id("test-wrap")
            .class("custom-container")
            .style(|s| s.custom("padding: 20px; margin: 10px;"));

        assert_eq!(wrap.id, Some("test-wrap".to_string()));
        assert_eq!(wrap.class, "t_wrap custom-container");
        assert_eq!(
            wrap.style.map(|s| s.to_string()),
            Some("padding: 20px; margin: 10px;".to_string())
        );
    }

    #[test]
    fn test_wrap_with_children() {
        let wrap = Wrap::new().children(Text::new("Dynamic Text 1"));
        assert!(!wrap.childrens.is_empty());
    }

    #[test]
    fn test_wrap_with_dynamic_children() {
        use crate::Text;

        let wrap = Wrap::new()
            .children(Text::new("Dynamic Text 1"))
            .children(Text::new("Dynamic Text 2"));

        assert_eq!(wrap.childrens.len(), 2);
    }

    #[test]
    fn test_wrap_dynamic_batch() {
        use crate::Text;

        let components = vec![
            Text::new("Batch 1"),
            Text::new("Batch 2"),
            Text::new("Batch 3"),
        ];

        let wrap = Wrap::new().childrens2(components);
        assert_eq!(wrap.childrens.len(), 3);
    }

    #[test]
    fn test_bare_default() {
        let wrap = Wrap::new();
        assert!(!wrap.bare);
    }

    #[test]
    fn test_bare_true() {
        let wrap = Wrap::new().bare(true);
        assert!(wrap.bare);
    }

    #[test]
    fn test_bare_false() {
        let wrap = Wrap::new().bare(false);
        assert!(!wrap.bare);
    }
}
