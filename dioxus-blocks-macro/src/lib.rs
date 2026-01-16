//! Dioxus Blocks 过程宏库
//!
//! 提供用于 Dioxus 组件的过程宏，包括组件基础功能实现和路由组件自动生成。
//!
//! ## 宏
//!
//! - [`Route`][]: 为组件自动生成对应的路由组件
//! - [`ComponentBase`]: 为组件提供基础方法（id、class、style 等）
use proc_macro::TokenStream;

mod component;
mod route;

/// 为给定的结构体实现 `Route` 派生宏
///
/// 此宏会自动生成一个与结构体同名的组件函数（后缀为"Route"），
/// 该组件会创建并渲染原始结构体的实例。
///
/// # 示例
///
/// ```rust
/// use std::rc::Rc;
/// use dioxus_blocks_components::{Element, ToElement, View};
///
/// #[derive(Debug, Default, Clone)]
/// pub struct Header {}
///
/// impl ToElement for Header {
///     fn to_element(&self) -> Element {
///         View::new()
///             .class("t_header")
///             .children(View::new().class("logo"))
///             .to_element()
///     }
/// }
/// ```
#[proc_macro_derive(Route)]
pub fn derive_route(input: TokenStream) -> TokenStream {
    route::impl_derive_route(input)
}

/// 为给定的结构体实现 `ComponentBase` 派生宏
///
/// 此宏会为结构体自动实现基础方法，包括 id/class/style/children/class 等。
///
/// # 示例
///
/// ```rust
/// # use std::rc::Rc;
/// # use dioxus::prelude::*;
/// # use dioxus_blocks_macro::ComponentBase;
/// # use dioxus_blocks_components::{Style, ToElement};
///
/// #[derive(Debug, Default, Clone, ComponentBase)]
/// pub struct MyComponent {
///     id: Option<String>,
///     class: String,
///     style: Option<Style>,
///     childrens: Vec<Rc<dyn ToElement>>,
///     onclick: Option<EventHandler<MouseEvent>>,
/// }
///
/// impl ToElement for MyComponent {
///     fn to_element(&self) -> Element {
///         rsx! {
///         }
///     }
/// }
/// ```
#[proc_macro_derive(ComponentBase)]
pub fn derive_component_base(input: TokenStream) -> TokenStream {
    component::impl_component_base(input)
}
