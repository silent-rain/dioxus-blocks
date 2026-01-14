//! 组件基础功能宏实现
//!
//! 提供用于实现组件基础功能的宏，包括 `ComponentBase` 派生宏的实现。
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// 为结构体自动实现 ComponentBase 的宏
///
/// 此宏会为结构体自动实现基础方法，包括 id、class、style 和 children 等。
/// 这些方法允许链式调用，方便组件的配置和使用。
pub fn impl_component_base(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl #name {
            /// 设置组件的 ID
            ///
            /// # 参数
            ///
            /// * `id` - 要设置的 ID，任何实现了 `Into<String>` 的类型都可以
            ///
            /// # 返回值
            ///
            /// 返回修改后的组件实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus_blocks_components::Text;
            /// Text::new("Hello").id("my_text");
            /// ```
            pub fn id<T: Into<String>>(mut self, id: T) -> Self {
                self.id = Some(id.into());
                self
            }

            /// 设置组件的 CSS 类名
            ///
            /// # 参数
            ///
            /// * `class` - 要添加的 CSS 类名，任何实现了 `Into<String>` 的类型都可以
            ///
            /// # 返回值
            ///
            /// 返回修改后的组件实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus_blocks_components::Text;
            /// Text::new("Hello").class("highlight");
            /// ```
            pub fn class<T: Into<String>>(mut self, class: T) -> Self {
                let default_class = Self::default().class.clone();
                self.class = format!("{} {}", default_class, class.into());
                self
            }

            /// 使用闭包设置样式
            ///
            /// # 参数
            ///
            /// * `style_handler` - 一个闭包，接受样式构建器并返回样式字符串
            ///
            /// # 返回值
            ///
            /// 返回修改后的文本实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus_blocks_components::{Text, Style};
            /// Text::new("Hello").style(|s| s.color("red").font_size("16px"));
            /// ```
            pub fn style<F>(mut self, style_handler: F) -> Self
            where
                F: FnOnce(Style) -> Style,
            {
                let style = self.style.unwrap_or_default();
                self.style = Some(style_handler(style));
                self
            }

            /// 添加动态组件到 children 容器中
            ///
            /// # 参数
            ///
            /// * `component` - 实现了 ToElement + Clone 的组件
            ///
            /// # 返回值
            ///
            /// 返回修改后的容器实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus::prelude::*;
            /// # use dioxus::core::Mutations;
            /// # use dioxus_blocks_components::{Text, ToElement};
            /// # let mut dom = VirtualDom::new(|| {
            ///     Text::new("Hello")
            ///         .children(Text::new("World"))
            ///         .to_element()
            /// # });
            /// # let mut mutations = Mutations::default();
            /// # dom.rebuild(&mut mutations);
            /// ```
            pub fn children<T>(mut self, component: T) -> Self
            where
                T: ToElement + Clone + 'static,
            {
                self.childrens.push(Arc::new(component));
                self
            }

            /// 批量添加组件到 children 容器中
            ///
            /// # 参数
            ///
            /// * `components` - 实现了 ToElement + Clone 的组件向量
            ///
            /// # 返回值
            ///
            /// 返回修改后的容器实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus_blocks_components::{Text, ToElement};
            /// let components = vec![
            ///     Text::new("First"),
            ///     Text::new("Second"),
            /// ];
            /// Text::new("Hello").childrens2(components);
            /// ```
            ///
            pub fn childrens2<T>(mut self, components: Vec<T>) -> Self
            where
                T: ToElement + Clone + 'static,
            {
                for component in components {
                    self.childrens.push(Arc::new(component));
                }
                self
            }

            /// 批量添加不同类型的组件到 children 容器中
            ///
            /// # 参数
            ///
            /// * `components` - 实现了 ToElement 的 trait 对象向量，使用 Arc 进行引用计数
            ///
            /// # 返回值
            ///
            /// 返回修改后的容器实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus_blocks_components::{Text, Button, ToElement};
            /// # use std::sync::Arc;
            ///
            /// let components: Vec<Arc<dyn ToElement>> = vec![
            ///     Arc::new(Text::h1("标题")),
            ///     Arc::new(Button::new()),
            /// ];
            ///
            /// Text::new("Hello").childrens(components);
            /// ```
            pub fn childrens(mut self, components: Vec<Arc<dyn ToElement>>) -> Self {
                for component in components {
                    self.childrens.push(component);
                }
                self
            }

            /// 将子元素转换为 Element 类型
            ///
            /// # 返回值
            ///
            ///
            /// 返回一个新的 Element 实例
            ///
            /// # 示例
            ///
            ///
            /// ```rust
            /// # use dioxus_blocks_components::Wrap;
            /// let wrap = Wrap::new();
            /// let element = wrap.childrens_to_element();
            /// ```
            pub fn childrens_to_element(&self) -> Element {
                rsx! {
                    for children in self.childrens.iter() {
                        {children.to_element()}
                    }
                }
            }

            /// 设置按钮的点击事件处理器
            ///
            /// # 参数
            ///
            /// * `onclick` - 当按钮被点击时调用的闭包或函数
            ///
            /// # 返回值
            ///
            /// 返回修改后的按钮实例，支持链式调用
            ///
            /// # 示例
            ///
            /// ```rust
            /// # use dioxus::prelude::*;
            /// # use dioxus::core::Mutations;
            /// # use dioxus_blocks_components::{Button, ToElement};
            /// # let mut dom = VirtualDom::new(|| {
            ///     Button::new()
            ///         .onclick(EventHandler::new(move |_| {
            ///             println!("按钮被点击了");
            ///         }))
            ///         .to_element()
            /// # });
            /// # let mut mutations = Mutations::default();
            /// # dom.rebuild(&mut mutations);
            /// ```
            pub fn onclick(mut self, onclick: EventHandler<MouseEvent>) -> Self {
                self.onclick = Some(onclick);
                self
            }
        }


        /// 将组件转换为 Element
        impl From<#name> for Element {
            fn from(component: #name) -> Self {
                component.to_element()
            }
        }

    };

    TokenStream::from(expanded)
}
