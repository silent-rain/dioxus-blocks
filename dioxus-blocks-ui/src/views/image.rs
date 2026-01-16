use dioxus::prelude::*;
use dioxus_blocks_components::{Card, Image, ObjectFit, Style, Text, ToElement, View};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct ImageView {}

impl ToElement for ImageView {
    fn to_element(&self) -> Element {
        View::new().children(self.content()).to_element()
    }
}

impl ImageView {
    fn content(&self) -> Card {
        let text_img_style = |s: Style| {
            s.display("flex")
                .flex_direction("column")
                .align_items("center")
        };
        let img_none = View::new()
            .children(Text::span("None"))
            .children(
                Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100px")
                .with_height("100px")
                .with_object_fit(ObjectFit::None),
            )
            .style(text_img_style);
        let img_fill = View::new()
            .children(Text::span("Fill"))
            .children(
                Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100px")
                .with_height("100px")
                .with_object_fit(ObjectFit::Fill),
            )
            .style(text_img_style);
        let img_contain = View::new()
            .children(Text::span("Contain"))
            .children(
                Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100px")
                .with_height("100px")
                .with_object_fit(ObjectFit::Contain),
            )
            .style(text_img_style);
        let img_cover = View::new()
            .children(Text::span("Cover"))
            .children(
                Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100px")
                .with_height("100px")
                .with_object_fit(ObjectFit::Cover),
            )
            .style(text_img_style);
        let img_scale_down = View::new()
            .children(Text::span("ScaleDown"))
            .children(
                Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100px")
                .with_height("100px")
                .with_object_fit(ObjectFit::ScaleDown),
            )
            .style(text_img_style);

        Card::new()
            .header(View::new().children(Text::h2("基础用法")).children(Text::p(
                "可通过fit确定图片如何适应到容器框，同原生 object-fit 。",
            )))
            .body(
                View::new()
                    .childrens2(vec![
                        img_none,
                        img_fill,
                        img_contain,
                        img_cover,
                        img_scale_down,
                    ])
                    .style(|s| s.display("flex").gap("16px")),
            )
    }
}
