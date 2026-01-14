//! # Navbar
use crate::Route;
use dioxus_blocks_components::{Element, Link, NavigationTarget, ToElement, View};

#[derive(Debug, Default, Clone)]
pub struct Navbar {}

impl ToElement for Navbar {
    fn to_element(&self) -> Element {
        View::new()
            .class("t_navbar")
            .childrens2(vec![
                Link::default().to(Route::HomeViewRoute {}).text("Home"),
                Link::default().to(Route::Blog { id: 1 }).text("Blog"),
                Link::default().to(Route::TextViewRoute {}).text("Text"),
                Link::default().to("/xxx1").text("Text1"),
                Link::default()
                    .to(NavigationTarget::<String>::from("/xxx2"))
                    .text("Text2"),
                Link::default().to("https://www.baidu.com/").text("Text3"),
            ])
            .to_element()
    }
}
