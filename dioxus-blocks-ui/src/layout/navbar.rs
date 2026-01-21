//! Navbar 导航栏组件
//!
//! 提供网站主导航菜单，包含多个导航链接。
use crate::Route;
use dioxus_blocks_components::{Element, Link, NavigationTarget, ToElement, View};

#[derive(Debug, Default, Clone)]
pub struct Navbar {}

impl ToElement for Navbar {
    fn to_element(&self) -> Element {
        View::new()
            .style(|s| s.display("flex").align_items("center").gap("8px"))
            .childrens2(vec![
                self.create_nav_link(Route::HomeViewRoute {}, "首页"),
                self.create_nav_link(Route::BlogRoute { id: 1 }, "博客"),
                self.create_nav_link("/guide", "指南"),
                self.create_nav_link(NavigationTarget::<String>::from("/component"), "组件"),
                self.create_external_link("https://github.com/silent-rain/dioxus-blocks", "GitHub"),
            ])
            .to_element()
    }
}

impl Navbar {
    /// 创建导航链接
    ///
    /// # 参数
    ///
    /// * `target` - 导航目标
    /// * `text` - 链接文本
    ///
    /// # 返回值
    ///
    /// 返回一个导航链接组件
    fn create_nav_link<T>(&self, target: T, text: &str) -> Link
    where
        T: Into<NavigationTarget>,
    {
        Link::default()
            .to(target)
            .text(text)
            .class("nav-link")  // 使用 CSS 类处理伪类
            .style(|s| {
                s.padding("8px 16px")
                    .color("var(--t-text-color-primary)")
                    .text_decoration("none")
                    .border_radius("4px")
                    .transition("all var(--t-transition-duration)")
            })
    }

    /// 创建外部链接
    ///
    /// # 参数
    ///
    /// * `url` - 外部 URL
    /// * `text` - 链接文本
    ///
    /// # 返回值
    ///
    /// 返回一个外部链接组件
    fn create_external_link(&self, url: &str, text: &str) -> Link {
        Link::default()
            .to(url)
            .text(text)
            .class("nav-link external-link")  // 使用 CSS 类处理伪类
            .style(|s| {
                s.padding("8px 16px")
                    .color("var(--t-text-color-primary)")
                    .text_decoration("none")
                    .border_radius("4px")
                    .transition("all var(--t-transition-duration)")
            })
    }
}
