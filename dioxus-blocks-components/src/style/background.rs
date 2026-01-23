//! 背景相关样式
//!
//! 提供背景颜色和渐变相关的样式方法。

use super::Style;

impl Style {
    /// 背景颜色
    ///
    /// # 参数
    /// * `color` - 背景颜色值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background_color("#ffffff");
    /// ```
    ///
    pub fn background_color<T: Into<String>>(self, color: T) -> Self {
        self.insert_style("background-color", color.into())
    }

    /// 背景图像
    ///
    /// # 参数
    /// * `image` - 背景图像值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background_image("url('bg.jpg')");
    /// ```
    ///
    pub fn background_image<T: Into<String>>(self, image: T) -> Self {
        self.insert_style("background-image", image.into())
    }

    /// 背景位置
    ///
    /// # 参数
    /// * `position` - 背景位置值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background_position("center");
    /// ```
    ///
    pub fn background_position<T: Into<String>>(self, position: T) -> Self {
        self.insert_style("background-position", position.into())
    }

    /// 背景尺寸
    ///
    /// # 参数
    /// * `size` - 背景尺寸值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background_size("cover");
    /// ```
    ///
    pub fn background_size<T: Into<String>>(self, size: T) -> Self {
        self.insert_style("background-size", size.into())
    }

    /// 背景重复
    ///
    /// # 参数
    /// * `repeat` - 背景重复值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background_repeat("no-repeat");
    /// ```
    ///
    pub fn background_repeat<T: Into<String>>(self, repeat: T) -> Self {
        self.insert_style("background-repeat", repeat.into())
    }

    /// 背景渐变
    ///
    /// # 参数
    /// * `gradient` - 背景渐变值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background("linear-gradient(to right, #ff7e5f, #feb47b)");
    /// ```
    ///
    pub fn background<T: Into<String>>(self, background: T) -> Self {
        self.insert_style("background", background.into())
    }
}
