//! 边框相关样式
//!
//! 提供 border 和 border-radius 相关的样式方法。

use super::Style;

impl Style {
    /// 边框
    ///
    /// # 参数
    /// * `border` - 边框值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border("1px solid #000");
    /// ```
    ///
    pub fn border<T: Into<String>>(self, border: T) -> Self {
        self.insert_style("border", border.into())
    }

    /// 边框颜色
    ///
    /// # 参数
    /// * `border_color` - 边框颜色值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_color("#000");
    /// ```
    ///
    pub fn border_color<T: Into<String>>(self, border_color: T) -> Self {
        self.insert_style("border-color", border_color.into())
    }

    /// 上边框
    ///
    /// # 参数
    /// * `border_top` - 上边框值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_top("1px solid #000");
    /// ```
    ///
    pub fn border_top<T: Into<String>>(self, border_top: T) -> Self {
        self.insert_style("border-top", border_top.into())
    }

    /// 底部边框
    ///
    /// # 参数
    /// * `border_bottom` - 底部边框值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_bottom("1px solid #000");
    /// ```
    ///
    pub fn border_bottom<T: Into<String>>(self, border_bottom: T) -> Self {
        self.insert_style("border-bottom", border_bottom.into())
    }

    /// 左边框
    ///
    /// # 参数
    /// * `border_left` - 左边框值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_left("1px solid #000");
    /// ```
    ///
    pub fn border_left<T: Into<String>>(self, border_left: T) -> Self {
        self.insert_style("border-left", border_left.into())
    }

    /// 右边框
    ///
    /// # 参数
    /// * `border_right` - 右边框值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_right("1px solid #000");
    /// ```
    ///
    pub fn border_right<T: Into<String>>(self, border_right: T) -> Self {
        self.insert_style("border-right", border_right.into())
    }

    /// 圆角
    ///
    /// 圆角
    ///
    /// # 参数
    /// * `radius` - 圆角值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_radius("5px");
    /// ```
    ///
    pub fn border_radius<T: Into<String>>(self, radius: T) -> Self {
        self.insert_style("border-radius", radius.into())
    }

    /// 左上圆角
    ///
    /// # 参数
    /// * `radius` - 左上圆角值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_top_left_radius("5px");
    /// ```
    ///
    pub fn border_top_left_radius<T: Into<String>>(self, radius: T) -> Self {
        self.insert_style("border-top-left-radius", radius.into())
    }

    /// 右上圆角
    ///
    /// # 参数
    /// * `radius` - 右上圆角值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_top_right_radius("5px");
    /// ```
    ///
    pub fn border_top_right_radius<T: Into<String>>(self, radius: T) -> Self {
        self.insert_style("border-top-right-radius", radius.into())
    }

    /// 左下圆角
    ///
    /// # 参数
    /// * `radius` - 左下圆角值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_bottom_left_radius("5px");
    /// ```
    ///
    pub fn border_bottom_left_radius<T: Into<String>>(self, radius: T) -> Self {
        self.insert_style("border-bottom-left-radius", radius.into())
    }

    /// 右下圆角
    ///
    /// # 参数
    /// * `radius` - 右下圆角值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border_bottom_right_radius("5px");
    /// ```
    ///
    pub fn border_bottom_right_radius<T: Into<String>>(self, radius: T) -> Self {
        self.insert_style("border-bottom-right-radius", radius.into())
    }
}
