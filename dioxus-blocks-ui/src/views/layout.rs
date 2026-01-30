//! Layout 组件 (Row/Col)

use dioxus::prelude::*;
use dioxus_blocks_components::{Card, Col, Justify, Row, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct LayoutView {}

impl ToElement for LayoutView {
    fn to_element(&self) -> Element {
        View::new()
            .children(self.title())
            .children(self.content())
            .to_element()
    }
}

impl LayoutView {
    fn title(&self) -> View {
        View::new().childrens(vec![
            Text::h1("Layout 布局"),
            Text::p("基于 Flexbox 的布局组件，包含 Row（行）和 Col（列）两个组件。"),
        ])
    }

    fn content(&self) -> View {
        View::new().childrens(vec![
            self.basic_section(),
            self.gutter_section(),
            self.justify_section(),
            self.col_span_section(),
            self.responsive_section(),
        ])
    }

    fn basic_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("基础用法"))
                    .children(Text::p("使用 Row 和 Col 进行基础布局，支持 24 栅格系统。")),
            )
            .body(View::new().childrens(vec![
                self.row_24(),
                self.row_12_12(),
                self.row_8_8_8(),
                self.row_6_6_6_6(),
                self.row_4_4_4_4_4_4(),
            ]))
            .style(|s| s.margin_top("32px"))
    }

    fn row_24(&self) -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#7e57c2")
                    .border_radius("4px")
            }))
            .span(24),
        ])
        .style(|s| s.margin_bottom("20px"))
    }

    fn row_12_12(&self) -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(12),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(12),
        ])
        .style(|s| s.margin_bottom("20px"))
    }

    fn row_8_8_8(&self) -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(8),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(8),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(8),
        ])
        .style(|s| s.margin_bottom("20px"))
    }

    fn row_6_6_6_6(&self) -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .style(|s| s.margin_bottom("20px"))
    }

    fn row_4_4_4_4_4_4(&self) -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(4),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(4),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(4),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(4),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(4),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(4),
        ])
        .style(|s| s.margin_bottom("0"))
    }

    fn gutter_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("列间距 (Gutter)"))
                    .children(Text::p("通过 gap 属性设置列之间的间距，使布局更加美观。")),
            )
            .body(View::new().childrens(vec![Self::gutter_example_20()]))
            .style(|s| s.margin_top("32px"))
    }

    fn gutter_example_20() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#7e57c2")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#7e57c2")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#7e57c2")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#7e57c2")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .align_items("center")
        .gutter(20)
        .style(|s| s.margin_bottom("0"))
    }

    fn justify_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("对齐方式 (Justify)"))
                    .children(Text::p("通过 justify 属性设置子元素的水平对齐方式。")),
            )
            .body(View::new().childrens(vec![
                Self::justify_start(),
                Self::justify_center(),
                Self::justify_end(),
                Self::justify_between(),
                Self::justify_around(),
                Self::justify_evenly(),
            ]))
            .style(|s| s.margin_top("32px"))
    }

    fn justify_start() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .justify(Justify::Start)
        .style(|s| s.margin_bottom("20px"))
    }

    fn justify_center() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .justify(Justify::Center)
        .style(|s| s.margin_bottom("20px"))
    }

    fn justify_end() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .justify(Justify::End)
        .style(|s| s.margin_bottom("20px"))
    }

    fn justify_between() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .justify(Justify::SpaceBetween)
        .style(|s| s.margin_bottom("20px"))
    }

    fn justify_around() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .justify(Justify::SpaceAround)
        .style(|s| s.margin_bottom("20px"))
    }

    fn justify_evenly() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#b39ddb")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
        ])
        .justify(Justify::SpaceEvenly)
        .style(|s| s.margin_bottom("0"))
    }

    fn col_span_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("列偏移 (Col Offset)"))
                    .children(Text::p(
                        "通过 margin-left 设置列的偏移量，实现更灵活的布局。",
                    )),
            )
            .body(View::new().childrens(vec![
                Self::col_offset_example_1(),
                Self::col_offset_example_2(),
                Self::col_offset_example_3(),
            ]))
            .style(|s| s.margin_top("32px"))
    }

    fn col_offset_example_1() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6)
            .offset(6),
        ])
        .style(|s| s.margin_bottom("20px"))
    }

    fn col_offset_example_2() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6)
            .offset(6),
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(6)
            .offset(6),
        ])
        .style(|s| s.margin_bottom("20px"))
    }

    fn col_offset_example_3() -> Row {
        Row::new(vec![
            Col::new(View::new().style(|s| {
                s.min_height("36px")
                    .background_color("#9575cd")
                    .border_radius("4px")
            }))
            .span(12)
            .offset(6),
        ])
        .style(|s| s.margin_bottom("0"))
    }

    fn responsive_section(&self) -> Card {
        Card::new()
            .header(
                View::new()
                    .children(Text::h3("复杂布局示例"))
                    .children(Text::p("嵌套使用 Row 和 Col 创建复杂的响应式布局。")),
            )
            .body(self.nested_layout_example())
            .style(|s| s.margin_top("32px"))
    }

    fn nested_layout_example(&self) -> Row {
        Row::new(vec![
            Col::new(
                View::new()
                    .childrens(vec![
                        Text::h3("侧边栏"),
                        Text::p("导航菜单"),
                        Text::p("菜单项 1"),
                        Text::p("菜单项 2"),
                        Text::p("菜单项 3"),
                    ])
                    .style(|s| {
                        s.padding("20px")
                            .background_color("#e8eaf6")
                            .border_radius("4px")
                    }),
            )
            .span(6)
            .style(|s| s.border_radius("8px")),
            Col::new(
                View::new()
                    .childrens(vec![
                        Text::h3("主内容区"),
                        Text::p("主要内容展示"),
                        Text::p("卡片 1"),
                        Text::p("卡片 2"),
                        Text::p("卡片 3"),
                    ])
                    .style(|s| {
                        s.padding("20px")
                            .background_color("#f3e5f5")
                            .border_radius("4px")
                    }),
            )
            .span(18)
            .style(|s| s.border_radius("8px")),
        ])
        .gutter(12)
    }
}
