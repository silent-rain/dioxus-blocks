use dioxus::prelude::*;
use dioxus_blocks_components::{Card, Image, ObjectFit, Text, ToElement, View};
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
        Card::new()
            .header(Text::h2("基础用法"))
            .body(View::new().childrens2(vec![
                Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100%")
                .with_height("auto")
                .with_object_fit(ObjectFit::None),
                 Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100%")
                .with_height("auto")
                .with_object_fit(ObjectFit::Fill),
                 Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100%")
                .with_height("auto")
                .with_object_fit(ObjectFit::Contain),
                 Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100%")
                .with_height("auto")
                .with_object_fit(ObjectFit::Cover),
                 Image::new(
                    "https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg",
                )
                .alt("响应式图片")
                .with_width("100%")
                .with_height("auto")
                .with_object_fit(ObjectFit::ScaleDown),
            ]))
    }
}
