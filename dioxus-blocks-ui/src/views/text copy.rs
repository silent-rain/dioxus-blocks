//! Text 组件

use dioxus::prelude::*;

use dioxus_blocks_components::{Style, Text};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct TextView {}

impl TextView {
    pub fn element(&self) -> Element {
        rsx! {
            div { class: "text-section",
                h1 { class: "section-title", "Text 组件示例" }
                p { class: "section-description", "演示如何使用 Text 组件的各种功能" }
            }
            {self.basic_text()}
            {self.text_sizes()}
            {self.text_colors()}
            {self.text_alignments()}
            {self.text_weights()}
            {self.text_decorations()}
            {self.text_transforms()}
            {self.text_line_heights()}
            {self.text_families()}
            {self.comprehensive_example()}
        }
    }
}

impl TextView {
    /// 基本文本
    pub fn basic_text(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "基本文本" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("这是基本的文本组件")
                            .style(Style::default().font_size("1rem").color("#374151"))
                            .element()
                    }
                }
            }
        }
    }

    /// 不同大小的文本
    pub fn text_sizes(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "不同大小的文本" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("小号文本 (12px)")
                            .style(Style::default().font_size("12px").color("#6b7280"))
                            .element()
                    }
                    {
                        Text::new()
                            .content("中号文本 (16px)")
                            .style(Style::default().font_size("16px").color("#6b7280"))
                            .element()
                    }
                    {
                        Text::new()
                            .content("大号文本 (20px)")
                            .style(Style::default().font_size("20px").color("#6b7280"))
                            .element()
                    }
                    {
                        Text::new()
                            .content("特大号文本 (24px)")
                            .style(Style::default().font_size("24px").color("#6b7280"))
                            .element()
                    }
                }
            }
        }
    }

    /// 不同颜色的文本
    pub fn text_colors(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "不同颜色的文本" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("红色文本")
                            .style(Style::default().font_size("1rem").color("#ef4444"))
                            .element()
                    }
                    {
                        Text::new()
                            .content("绿色文本")
                            .style(Style::default().font_size("1rem").color("#10b981"))
                            .element()
                    }
                    {
                        Text::new()
                            .content("蓝色文本")
                            .style(Style::default().font_size("1rem").color("#3b82f6"))
                            .element()
                    }
                    {
                        Text::new()
                            .content("紫色文本")
                            .style(Style::default().font_size("1rem").color("#8b5cf6"))
                            .element()
                    }
                }
            }
        }
    }

    /// 不同对齐方式的文本
    pub fn text_alignments(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "不同对齐方式的文本" }
                div { class: "example-content",
                    div { class: "alignment-demo",
                        {
                            Text::new()
                                .content("左对齐文本 (默认)")
                                .style(
                                    Style::default().font_size("1rem").color("#374151").text_align("left"),
                                )
                                .element()
                        }
                    }
                    div { class: "alignment-demo",
                        {
                            Text::new()
                                .content("居中对齐文本")
                                .style(
                                    Style::default().font_size("1rem").color("#374151").text_align("center"),
                                )
                                .element()
                        }
                    }
                    div { class: "alignment-demo",
                        {
                            Text::new()
                                .content("右对齐文本")
                                .style(
                                    Style::default().font_size("1rem").color("#374151").text_align("right"),
                                )
                                .element()
                        }
                    }
                }
            }
        }
    }

    /// 不同字体粗细的文本
    pub fn text_weights(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "不同字体粗细的文本" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("细体文本 (300)")
                            .style(
                                Style::default().font_size("1rem").color("#374151").font_weight("300"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("正常文本 (400)")
                            .style(
                                Style::default().font_size("1rem").color("#374151").font_weight("400"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("中等文本 (500)")
                            .style(
                                Style::default().font_size("1rem").color("#374151").font_weight("500"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("粗体文本 (700)")
                            .style(
                                Style::default().font_size("1rem").color("#374151").font_weight("700"),
                            )
                            .element()
                    }
                }
            }
        }
    }

    /// 文本装饰示例
    pub fn text_decorations(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "文本装饰" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("无装饰文本")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_decoration("none"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("下划线文本")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_decoration("underline"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("删除线文本")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_decoration("line-through"),
                            )
                            .element()
                    }
                }
            }
        }
    }

    /// 文本转换示例
    pub fn text_transforms(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "文本转换" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("原始文本: hello world")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_transform("none"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("大写转换: hello world")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_transform("uppercase"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("小写转换: HELLO WORLD")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_transform("lowercase"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("首字母大写: hello world")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .text_transform("capitalize"),
                            )
                            .element()
                    }
                }
            }
        }
    }

    /// 行高示例
    pub fn text_line_heights(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "行高" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content(
                                "这是一段行高为1.2的文本。这是一段行高为1.2的文本。这是一段行高为1.2的文本。",
                            )
                            .style(
                                Style::default().font_size("1rem").color("#374151").line_height("1.2"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content(
                                "这是一段行高为1.5的文本。这是一段行高为1.5的文本。这是一段行高为1.5的文本。",
                            )
                            .style(
                                Style::default().font_size("1rem").color("#374151").line_height("1.5"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content(
                                "这是一段行高为2.0的文本。这是一段行高为2.0的文本。这是一段行高为2.0的文本。",
                            )
                            .style(
                                Style::default().font_size("1rem").color("#374151").line_height("2.0"),
                            )
                            .element()
                    }
                }
            }
        }
    }

    /// 字体族示例
    pub fn text_families(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "字体族" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("这是使用 Arial 字体的文本")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .font_family("Arial, sans-serif"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("这是使用 Georgia 字体的文本")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .font_family("Georgia, serif"),
                            )
                            .element()
                    }
                    {
                        Text::new()
                            .content("这是使用 Courier New 字体的文本")
                            .style(
                                Style::default()
                                    .font_size("1rem")
                                    .color("#374151")
                                    .font_family("Courier New, monospace"),
                            )
                            .element()
                    }
                }
            }
        }
    }

    /// 综合示例
    pub fn comprehensive_example(&self) -> Element {
        rsx! {
            div { class: "example-section",
                h2 { class: "example-title", "综合样式示例" }
                div { class: "example-content",
                    {
                        Text::new()
                            .content("这是一个综合了多种样式的文本示例")
                            .style(
                                Style::default()
                                    .font_size("1.125rem")
                                    .color("#4f46e5")
                                    .text_align("center")
                                    .font_weight("600")
                                    .line_height("1.6")
                                    .text_decoration("underline")
                                    .font_family("Arial, sans-serif"),
                            )
                            .element()
                    }
                }
            }
        }
    }
}
