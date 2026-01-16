//! View 组件
//!
//! 提供一个空的容器组件，用于包装其他元素，支持自定义样式和布局。
//!
//! # 示例
//!
//! ## 基本用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{View, Text, ToElement};
//!
//! # let mut dom = VirtualDom::new(|| {
//!
//! #[component]
//! fn App() -> Element {
//!     let view = View::new()
//!         .class("container")
//!         .style(|s| s.custom("padding: 20px; margin: 10px;"))
//!         .children(Text::new("Hello, View!"))
//!         .to_element();
//!     rsx! {
//!         {view}
//!     }
//! }
//!
//! # App()
//! # });
//! # dom.rebuild(&mut dioxus_core::NoOpMutations);
//!
//! ```
//! ```
use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// View 组件结构体
///
/// 提供一个空的容器，用于包装其他元素，支持丰富的样式配置。
/// 类似 HTML 的 div 或 Vue 的 template，支持裸露渲染（bare）模式。
#[derive(Debug, Clone, ComponentBase)]
pub struct View {
    /// 容器组件的唯一标识符
    id: Option<String>,
    /// 容器组件的CSS类名
    class: String,
    /// 容器组件的内联样式
    style: Option<Style>,
    /// 容器组件的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 容器组件的点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 是否裸露渲染（不使用 div 包装），默认为 false
    bare: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for View {
    fn default() -> Self {
        Self {
            id: None,
            class: String::new(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            bare: false,
        }
    }
}

impl View {
    /// 创建一个新的容器实例
    ///
    /// # 返回值
    ///
    /// 返回一个新的容器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::View;
    /// let view = View::new();
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
    /// # use dioxus_blocks_components::View;
    /// let view = View::new().bare(true);
    /// ```
    pub fn bare(mut self, bare: bool) -> Self {
        self.bare = bare;
        self
    }
}

impl ToElement for View {
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
    fn test_view_creation() {
        let view = View::new();

        assert!(view.childrens.is_empty());
    }

    #[test]
    fn test_view_properties() {
        let view = View::new()
            .id("test-view")
            .class("custom-container")
            .style(|s| s.custom("padding: 20px; margin: 10px;"));

        assert_eq!(view.id, Some("test-view".to_string()));
        assert_eq!(
            view.style.map(|s| s.to_string()),
            Some("padding: 20px; margin: 10px;".to_string())
        );
    }

    #[test]
    fn test_view_with_children() {
        let view = View::new().children(Text::new("Dynamic Text 1"));
        assert!(!view.childrens.is_empty());
    }

    #[test]
    fn test_view_with_dynamic_children() {
        use crate::Text;

        let view = View::new()
            .children(Text::new("Dynamic Text 1"))
            .children(Text::new("Dynamic Text 2"));

        assert_eq!(view.childrens.len(), 2);
    }

    #[test]
    fn test_view_dynamic_batch() {
        use crate::Text;

        let components = vec![
            Text::new("Batch 1"),
            Text::new("Batch 2"),
            Text::new("Batch 3"),
        ];

        let view = View::new().childrens2(components);
        assert_eq!(view.childrens.len(), 3);
    }

    #[test]
    fn test_bare_default() {
        let view = View::new();
        assert!(!view.bare);
    }

    #[test]
    fn test_bare_true() {
        let view = View::new().bare(true);
        assert!(view.bare);
    }

    #[test]
    fn test_bare_false() {
        let view = View::new().bare(false);
        assert!(!view.bare);
    }
}
