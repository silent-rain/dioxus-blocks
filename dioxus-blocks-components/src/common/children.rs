//! Children 组件
//!
//! 提供一个容器组件，用于包装和渲染子元素，支持自定义样式和布局。
//! 与 View 组件不同，Children 组件专注于子元素的管理和渲染。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Children, Text};
//!
//! fn App() -> Children {
//!     Children::new()
//!         .children(Text::new("Hello, World!"))
//! }
//! ```
use dioxus::prelude::*;

use crate::traits::IntoElement;

/// 子元素容器组件结构体
///
/// 提供一个容器来管理和渲染子元素，支持丰富的样式配置。
/// 与 View 组件的主要区别是 Children 专注于子元素的动态管理。
///
/// # 字段
///
/// * `childrens` - 容器组件的子元素列表
#[derive(Debug, Default, Clone)]
pub struct Children {
    childrens: Vec<Element>,
}

impl Children {
    /// 创建一个新的子容器实例
    ///
    /// # 返回值
    ///
    /// 返回一个新的容器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Children;
    /// let container = Children::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加单个子元素
    ///
    /// # 参数
    ///
    /// * `children` - 要添加的子元素，任何实现了 `Into<Element>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的组件实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus::core::Mutations;
    /// # use dioxus_blocks_components::{Children, Text, IntoElement};
    /// # let mut dom = VirtualDom::new(|| {
    ///     Children::new()
    ///         .children(Text::new("Child Element"))
    ///         .into_element()
    /// # });
    /// # let mut mutations = Mutations::default();
    /// # dom.rebuild(&mut mutations);
    /// ```
    pub fn children<T: Into<Element>>(mut self, children: T) -> Self {
        self.childrens.push(children.into());
        self
    }

    /// 添加多个子元素
    ///
    /// # 参数
    ///
    /// * `childrens` - 要添加的子元素列表，任何实现了 `Into<Element>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的组件实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus::core::Mutations;
    /// # use dioxus_blocks_components::{Children, Text, IntoElement};
    /// # let mut dom = VirtualDom::new(|| {
    ///    Children::new()
    ///         .childrens(vec![
    ///             Text::new("Child 1"),
    ///             Text::new("Child 2"),
    ///         ])
    ///         .into_element()
    /// # });
    /// # let mut mutations = Mutations::default();
    /// # dom.rebuild(&mut mutations);
    /// ```
    pub fn childrens<T: Into<Element>>(mut self, childrens: Vec<T>) -> Self {
        self.childrens
            .extend(childrens.into_iter().map(|e| e.into()));
        self
    }

    /// 清空所有子元素
    ///
    /// # 返回值
    ///
    /// 返回 self 以支持链式调用
    pub fn clear(mut self) -> Self {
        self.childrens.clear();
        self
    }

    /// 获取子元素数量
    ///
    /// # 返回值
    ///
    /// 返回当前子元素的数量
    pub fn count(&self) -> usize {
        self.childrens.len()
    }

    /// 是否为空
    ///
    /// # 返回值
    ///
    /// 返回当前容器是存在子元素
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus::core::Mutations;
    /// # use dioxus_blocks_components::{Children, Text, IntoElement};
    /// # let mut dom = VirtualDom::new(|| {
    ///     let mut container = Children::new();
    ///     assert!(container.is_empty());
    ///     container = container.childrens(vec![
    ///         Text::new("Child 1"),
    ///         Text::new("Child 2"),
    ///     ]);
    ///    assert!(!container.is_empty());
    ///
    /// #   container.into_element()
    /// # });
    /// # let mut mutations = Mutations::default();
    /// # dom.rebuild(&mut mutations);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.childrens.is_empty()
    }
}

impl IntoElement for Children {
    fn into_element(self) -> Element {
        let childrens = &self.childrens;

        rsx! {
            for children in childrens.iter() {
                {children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_children_creation() {
        let container = Children::new();

        // 测试容器的基本属性
        assert!(container.childrens.is_empty());
    }

    #[test]
    fn test_add_children() {
        let container =
            Children::new().childrens(vec![rsx! { div { "Child 1" } }, rsx! { div { "Child 2" } }]);

        assert_eq!(container.count(), 2);
    }

    #[test]
    fn test_clear_children() {
        let container = Children::new()
            .children(rsx! {
                div { "Test Child" }
            })
            .clear();

        assert_eq!(container.count(), 0);
    }
}
