//! Card View
use dioxus::prelude::*;

use dioxus_blocks_components::{Card, CardShadow};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct CardView {}

impl CardView {
    pub fn element(&self) -> Element {
        rsx! {
            div { class: "card-section",
                h1 { class: "section-title", "极简卡片" }
                p { class: "section-description", "仅有内容的卡片形式。" }
            }
            {self.basic_card()}
            div { class: "card-section",
                h1 { class: "section-title", "带 header 的卡片" }
                p { class: "section-description",
                    "由极简卡片上方的标题栏组成，标题栏中可包含标题、图片、操作区、状态等内容。顶部栏可以定义所有的内容，以用户的自定义元素为准。"
                }
            }
            {self.header_and_body_card()}
            div { class: "card-section",
                h1 { class: "section-title", "边框与分割线控制" }
                p { class: "section-description",
                    "通过 border 属性控制卡片是否显示边框，通过 header_divider 属性控制标题与内容之间是否显示分割线。"
                }
            }
            {self.border_divider_card()}
            div { class: "card-section",
                h1 { class: "section-title", "带 footer 的卡片" }
                p { class: "section-description",
                    "由极简卡片下方的操作栏组成，操作栏中可包含按钮、链接、操作区等内容。底部栏可以定义所有的内容，以用户的自定义元素为准。"
                }
            }
            {self.footer_and_body_card()}
            div { class: "card-section",
                h1 { class: "section-title", "同时带 header 和 footer 的卡片" }
                p { class: "section-description",
                    "由顶部栏、底部栏和极简卡片组成的复杂卡片，三个区域内容可根据需要对内容进行配置。"
                }
            }
            {self.header_body_footer_card()}
            div { class: "card-section",
                h1 { class: "section-title", "带 children 的卡片" }
                p { class: "section-description",
                    "使用 children 属性替代 body 属性，可以使用任意元素作为卡片内容。"
                }
            }
            {self.children_card()}
            div { class: "card-section",
                h1 { class: "section-title", "阴影效果控制" }
                p { class: "section-description",
                    "通过 shadow 属性设置卡片阴影出现的时机。该属性的值可以是：always、hover 或 never。"
                }
            }
            {self.shadow_card()}
        }
    }
}

impl CardView {
    /// Basic card with only body content
    pub fn basic_card(&self) -> Element {
        Card::new()
            .class("basic-card")
            .body(rsx! {
                p { "This is a basic card with only body content." }
            })
            .into()
    }

    /// Card with header and body
    pub fn header_and_body_card(&self) -> Element {
        Card::new()
            .class("header-card")
            .header(rsx! {
                div {
                    span { "Card name" }
                    button { "More" }
                }
            })
            .body(rsx! {
                for i in 1..=4 {
                    p { class: "text item", key: "{i}", "List item {i}" }
                }
            })
            .into()
    }

    /// Border and divider card
    pub fn border_divider_card(&self) -> Element {
        Card::new()
            .class("header-card")
            .header(rsx! {
                div {
                    span { "Card name" }
                    button { "More" }
                }
            })
            .header_divider(false)
            .body(rsx! {
                for i in 1..=4 {
                    p { class: "text item", key: "{i}", "List item {i}" }
                }
            })
            .into()
    }

    /// Card with footer and body
    pub fn footer_and_body_card(&self) -> Element {
        Card::new()
            .class("footer-card")
            .footer(rsx! {
                div {
                    span { "Card name" }
                    button { "More" }
                }
            })
            .body(rsx! {
                for i in 1..=4 {
                    p { class: "text item", key: "{i}", "List item {i}" }
                }
            })
            .into()
    }

    // Card with header, body, and footer
    pub fn header_body_footer_card(&self) -> Element {
        Card::new()
            .class("full-card")
            .header(rsx! {
                div { class: "card-header",
                    span { "Complete Card" }
                }
            })
            .body(rsx! {
                for i in 1..=4 {
                    p { class: "text item", key: "{i}", "List item {i}" }
                }
            })
            .footer(rsx! {
                div { style: "display: flex; justify-content: flex-end; gap: 10px;",
                    button { "Cancel" }
                    button { "OK" }
                }
            })
            .into()
    }

    /// Card with children (alternative to body)
    pub fn children_card(&self) -> Element {
        Card::new()
            .class("children-card")
            .header(rsx! {
                div { class: "card-header",
                    span { "Card with Children" }
                }
            })
            .children(rsx! {
                p { "This card uses children instead of body prop." }
                p { "You can add multiple children elements." }
            })
            .footer(rsx! {
                span { "Footer content" }
            })
            .into()
    }

    /// Shadow control card
    pub fn shadow_card(&self) -> Element {
        // Always shadow
        let always_shadow: Element = Card::new()
            .class("shadow-always")
            .shadow(CardShadow::Always)
            .header(rsx! {
                span { "Always Shadow" }
            })
            .body(rsx! {
                p { "This card always has shadow." }
            })
            .into();

        // Hover shadow
        let hover_shadow: Element = Card::new()
            .class("shadow-hover")
            .shadow(CardShadow::Hover)
            .header(rsx! {
                span { "Hover Shadow" }
            })
            .body(rsx! {
                p { "This card has shadow on hover." }
            })
            .into();
        // Never shadow
        let never_shadow: Element = Card::new()
            .class("shadow-never")
            .shadow(CardShadow::Never)
            .header(rsx! {
                span { "No Shadow" }
            })
            .body(rsx! {
                p { "This card never has shadow." }
            })
            .into();
        rsx! {
            div { style: "display: flex; gap: 20px; flex-wrap: wrap;",

                // Always shadow
                {always_shadow}

                // Hover shadow
                {hover_shadow}

                // Never shadow
                {never_shadow}
            }
        }
    }
}
