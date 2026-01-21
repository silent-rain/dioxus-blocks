//! Header 头部组件
//!
//! 提供网站顶部导航栏，包含 logo、项目名称和导航菜单。
use std::rc::Rc;

use dioxus::prelude::{asset, manganis};
use dioxus_blocks_components::{Element, Image, Link, Text, ToElement, View};

use crate::Route;
use crate::layout::Navbar;

#[derive(Debug, Default, Clone)]
pub struct Header {}

impl ToElement for Header {
    fn to_element(&self) -> Element {
        let logo = asset!("/assets/img/logo.svg");
        View::new()
            .class("t_header")
            .style(|s| {
                s.display("flex")
                    .justify_content("space-between")
                    .align_items("center")
                    .padding("0 24px")
                    .height("64px")
                    .background_color("#ffffff")
                    .border_bottom("1px solid var(--t-border-color-light)")
                    .box_shadow("0 2px 8px rgba(0, 0, 0, 0.06)")
            })
            .children(
                Link::default()
                    .to(Route::HomeViewRoute {})
                    .style(|s| s.display("flex").align_items("center").gap("12px").text_decoration("none"))
                    .children(
                        Image::new(logo)
                            .with_width("32px")
                            .with_height("32px")
                            .alt("Logo"),
                    )
                    .children(Text::h1("Dioxus Blocks").style(|s| {
                        s.font_size("20px")
                            .font_weight("600")
                            .color("var(--t-text-color-primary)")
                            .margin("0")
                            .line_height("64px")
                    })),
            )
            .children(
                View::new()
                    .style(|s| s.display("flex").align_items("center"))
                    .childrens(vec![Rc::new(Navbar::default())]),
            )
            .to_element()
    }
}
