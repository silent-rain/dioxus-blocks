//! Button 组件
//!
//! 提供一个可自定义的按钮组件，支持文本、样式和点击事件处理。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Button, ToElement};
//!
//! # let mut dom = VirtualDom::new(|| {
//!
//! #[component]
//! fn App() -> Element {
//!     Button::new()
//!         .text("点击我")
//!         .class("btn-primary")
//!         .onclick(move |_: MouseEvent| {
//!             println!("Button clicked!");
//!         })
//!         .to_element()
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

/// 按钮组件结构体
///
/// 提供一个可自定义的按钮，支持文本、样式和点击事件处理。
#[derive(Debug, Clone, ComponentBase)]
pub struct Button {
    /// 按钮的唯一标识符
    id: Option<String>,
    /// 按钮的CSS类名
    class: String,
    /// 按钮的内联样式
    style: Option<Style>,
    /// 按钮的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 按钮点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 按钮显示的文本内容
    text: String,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_button".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            text: "Button".to_string(),
        }
    }
}

impl Button {
    /// 创建一个新的按钮实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的按钮实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// let button = Button::new();
    /// ```
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 设置按钮显示的文本
    ///
    /// # 参数
    ///
    /// * `text` - 要显示的文本内容，任何实现了 `Into<String>` 的类型都可以
    ///
    /// # 返回值
    ///
    /// 返回修改后的按钮实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Button;
    /// Button::new().text("点击我");
    /// ```
    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }
}

impl ToElement for Button {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let class = self.class.clone();
        let style = self.style.clone().map(|s| s.to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();
        let text = self.text.clone();
        rsx! {
            button {
                id,
                class,
                style,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
                {text}
                {childrens}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, rc::Rc};

    use dioxus::core::{ElementId, Mutations};
    use dioxus_html::SerializedHtmlEventConverter;

    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new().text("测试按钮");

        // 测试按钮的基本属性
        assert_eq!(button.text, "测试按钮");
        assert!(button.class.contains("t_button"));
    }

    #[test]
    fn test_button_properties() {
        // 测试按钮的基本属性设置
        let button = Button::new().text("测试按钮").class("custom-button");

        assert_eq!(button.text, "测试按钮");
        assert!(button.class.contains("custom-button"));
        assert!(button.class.contains("t_button"));
    }

    #[test]
    fn test_button_default_values() {
        let button = Button::new();

        assert_eq!(button.text, "Button");
        assert_eq!(button.class, "t_button");
    }

    /// 创建虚拟DOM测试
    #[test]
    fn test_with_virtual_dom() {
        fn app() -> Element {
            Button::new()
                .id("test-button")
                .text("Test Button")
                .onclick(|_| {
                    println!("Button clicked again!");
                })
                .to_element()
        }

        let mut dom = VirtualDom::new(app);
        let mut mutations = Mutations::default();
        dom.rebuild(&mut mutations);

        // 第一个真实元素的 id
        // 更稳妥的方法是检查 rebuild_to_vec() 返回的 mutations，找到分配给 button 的 ElementId
        let button_id = ElementId(1);

        // 告诉 dioxus 使用序列化事件转换器
        dioxus::html::set_event_converter(Box::new(SerializedHtmlEventConverter));

        // 构造一个事件
        let payload = PlatformEventData::new(Box::<SerializedMouseData>::default());
        let event = Event::new(Rc::new(payload) as Rc<dyn Any>, true);

        // 触发事件处理器
        dom.runtime().handle_event("click", event, button_id);
    }

    #[test]
    fn test_wrap_with_button() {
        // 创建一个 VirtualDom 作为运行时环境
        let mut dom = VirtualDom::new(|| {
            // 在这个闭包中，我们可以安全地使用需要运行时的组件
            Button::new()
                .text("Test Button")
                .onclick2(EventHandler::new(move |_| {
                    println!("Button clicked!");
                }))
                .onclick(|_| {
                    println!("Button clicked again!");
                })
                .to_element()
        });

        // 重新渲染以使 VirtualDom 初始化完成
        dom.rebuild(&mut dioxus_core::NoOpMutations);

        // 获取渲染结果
        let html = dioxus_ssr::render(&dom);

        // 验证渲染结果
        assert!(html.contains("button"));
        assert!(html.contains("class=\"t_button\""));
        assert!(html.contains("Test Button"));
    }

    /// 创建运行时上下文测试
    #[test]
    fn test_with_scope_provider() {
        #[derive(PartialEq, Props, Clone)]
        struct SomeProps {
            name: &'static str,
        }

        fn example(cx: SomeProps) -> Element {
            println!("Rendering with name: {}", cx.name);
            Button::new()
                .text(format!("hello {}", cx.name))
                .to_element()
        }

        let mut dom = VirtualDom::new_with_props(example, SomeProps { name: "world" });

        // 重建虚拟DOM
        dom.rebuild(&mut dioxus_core::NoOpMutations);
    }
}
