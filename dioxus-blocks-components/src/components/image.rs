//! Image 组件
//!
//! 提供一个可自定义的图片组件，支持加载状态、替代文本、尺寸等配置。
//!
//! # 示例
//!
//! ## 基本用法
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Image, ToElement};
//!
//! # let mut dom = VirtualDom::new(|| {
//!
//! #[component]
//! fn App() -> Element {
//!     let image = Image::new("https://example.com/image.jpg")
//!         .alt("示例图片")
//!         .width(300)
//!         .height(200)
//!         .to_element();
//!     rsx! {
//!         {image}
//!     }
//! }
//!
//! # App()
//! # });
//! # dom.rebuild(&mut dioxus_core::NoOpMutations);
//! ```
//!
//! ## 响应式图片
//!
//! ```rust
//! # use dioxus::prelude::*;
//! # use dioxus_blocks_components::{Image, ToElement};
//! #[component]
//! fn App() -> Element {
//!     let image = Image::new("https://example.com/image.jpg")
//!         .width("100%")
//!         .height("auto")
//!         .object_fit("cover")
//!         .to_element();
//!     rsx! {
//!         {image}
//!     }
//! }
//! ```
use std::sync::Arc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, traits::ToElement};

/// 对象适应方式枚举
///
/// 定义图片在容器中的适应方式
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ObjectFit {
    /// 拉伸以填满容器
    Fill,
    /// 保持纵横比，完整显示在容器内
    Contain,
    /// 保持纵横比，裁剪以填充容器
    Cover,
    /// 保持原始尺寸
    #[default]
    None,
    /// 缩放至最小尺寸
    ScaleDown,
}

impl From<String> for ObjectFit {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "fill" => ObjectFit::Fill,
            "contain" => ObjectFit::Contain,
            "cover" => ObjectFit::Cover,
            "scale-down" => ObjectFit::ScaleDown,
            _ => ObjectFit::None,
        }
    }
}

impl std::fmt::Display for ObjectFit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectFit::Fill => write!(f, "fill"),
            ObjectFit::Contain => write!(f, "contain"),
            ObjectFit::Cover => write!(f, "cover"),
            ObjectFit::None => write!(f, "none"),
            ObjectFit::ScaleDown => write!(f, "scale-down"),
        }
    }
}

/// 图片组件结构体
///
/// 提供一个可自定义的图片显示组件，支持丰富的图片配置。
#[derive(Debug, Clone, ComponentBase)]
pub struct Image {
    /// 图片组件的唯一标识符
    id: Option<String>,
    /// 图片组件的CSS类名
    class: String,
    /// 图片组件的内联样式
    style: Option<Style>,
    /// 图片组件的子元素列表
    childrens: Vec<Arc<dyn ToElement>>,
    /// 图片点击事件
    onclick: Option<EventHandler<MouseEvent>>,
    /// 图片的 URL 地址
    src: String,
    /// 图片的替代文本
    alt: String,
    /// 图片的宽度，可以是数字（像素）或字符串（如 "100%", "auto"）
    width: Option<String>,
    /// 图片的高度，可以是数字（像素）或字符串（如 "100%", "auto"）
    height: Option<String>,
    /// 图片对象适应方式
    object_fit: Option<ObjectFit>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_image".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,
            src: String::new(),
            alt: String::new(),
            width: None,
            height: None,
            object_fit: None,
        }
    }
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    /// 创建一个新的图片实例
    ///
    /// # 参数
    ///
    /// * `src` - 图片的 URL 地址
    ///
    /// # 返回值
    ///
    /// 返回一个新的图片实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Image;
    /// let image = Image::new("https://example.com/image.jpg");
    /// ```
    pub fn new<T: Into<String>>(src: T) -> Self {
        Self {
            src: src.into(),
            ..Default::default()
        }
    }

    /// 设置图片的替代文本
    ///
    /// # 参数
    ///
    /// * `alt` - 替代文本，用于图片加载失败时显示
    ///
    /// # 返回值
    ///
    /// 返回修改后的图片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Image;
    /// Image::new("https://example.com/image.jpg").alt("示例图片");
    /// ```
    pub fn alt<T>(mut self, alt: T) -> Self
    where
        T: Into<String>,
    {
        self.alt = alt.into();
        self
    }

    /// 设置图片的宽度（字符串值）
    ///
    /// # 参数
    ///
    /// * `width` - 宽度值，可以是 "100%", "auto", "50px" 等
    ///
    /// # 返回值
    ///
    /// 返回修改后的图片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Image;
    /// Image::new("https://example.com/image.jpg").with_width("100%");
    /// ```
    pub fn with_width<T>(mut self, width: T) -> Self
    where
        T: Into<String>,
    {
        self.width = Some(width.into());
        self
    }

    /// 设置图片的高度（字符串值）
    ///
    /// # 参数
    ///
    /// * `height` - 高度值，可以是 "100%", "auto", "50px" 等
    ///
    /// # 返回值
    ///
    /// 返回修改后的图片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Image;
    /// Image::new("https://example.com/image.jpg").with_height("auto");
    /// ```
    pub fn with_height<T>(mut self, height: T) -> Self
    where
        T: Into<String>,
    {
        self.height = Some(height.into());
        self
    }

    /// 设置图片的对象适应方式
    ///
    /// # 参数
    ///
    /// * `object_fit` - 对象适应方式
    ///
    /// # 返回值
    ///
    /// 返回修改后的图片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Image, ObjectFit};
    /// Image::new("https://example.com/image.jpg").with_object_fit(ObjectFit::Cover);
    /// ```
    pub fn with_object_fit(mut self, object_fit: ObjectFit) -> Self {
        self.object_fit = Some(object_fit);
        self
    }
}

impl ToElement for Image {
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let class = self.class.clone();
        let mut style = self
            .style
            .clone()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let onclick_handler = self.onclick;
        let src = self.src.clone();
        let alt = self.alt.clone();

        // 宽度
        if let Some(width) = &self.width {
            style.push_str(&format!("width: {};", width));
        }

        // 高度
        if let Some(height) = &self.height {
            style.push_str(&format!("height: {};", height));
        }

        // 对象适应方式
        if let Some(object_fit) = &self.object_fit {
            style.push_str(&format!("object-fit: {};", object_fit));
        }

        rsx! {
            img {
                id,
                class,
                style,
                src,
                alt,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let image = Image::new("https://example.com/image.jpg");
        assert_eq!(image.src, "https://example.com/image.jpg");
    }

    #[test]
    fn test_image_string_size() {
        let image = Image::new("https://example.com/image.jpg")
            .with_width("100%")
            .with_height("auto");

        assert_eq!(image.width, Some("100%".to_string()));
        assert_eq!(image.height, Some("auto".to_string()));
    }

    #[test]
    fn test_image_object_fit() {
        let image = Image::new("https://example.com/image.jpg").with_object_fit(ObjectFit::Cover);
        assert_eq!(image.object_fit, Some(ObjectFit::Cover));
    }
}
