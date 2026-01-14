//! 组件特征定义

use dioxus::prelude::*;

/// 组件转换为 Element 的引用特征
///
/// 所有组件都应该实现这个特征，提供统一的方法来转换为 Dioxus Element。
pub trait ToElement: std::fmt::Debug {
    /// 将组件转换为 Dioxus Element
    ///
    /// # 返回值
    ///
    /// 返回一个 Dioxus Element，表示渲染后的组件
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus::core::Mutations;
    /// # use dioxus_blocks_components::{Button, ToElement};
    /// # let mut dom = VirtualDom::new(|| {
    ///     Button::new()
    ///         .id("test-button")
    ///         .text("Test Button")
    ///         .to_element()
    /// # });
    /// # let mut mutations = Mutations::default();
    /// # dom.rebuild(&mut mutations);
    /// ```
    fn to_element(&self) -> Element;
}
