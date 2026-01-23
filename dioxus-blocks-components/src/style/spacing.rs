//! 间距相关样式
//!
//! 提供 margin 和 padding 相关的样式方法。

use super::Style;

impl Style {
    /// 边距
    ///
    /// # 参数
    /// * `margin` - 边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().margin("10px");
    /// ```
    ///
    pub fn margin<T: Into<String>>(self, margin: T) -> Self {
        self.insert_style("margin", margin.into())
    }

    /// 上边距
    ///
    /// # 参数
    /// * `margin_top` - 上边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().margin_top("10px");
    /// ```
    ///
    pub fn margin_top<T: Into<String>>(self, margin_top: T) -> Self {
        self.insert_style("margin-top", margin_top.into())
    }

    /// 下边距
    ///
    /// # 参数
    /// * `margin_bottom` - 下边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().margin_bottom("10px");
    /// ```
    ///
    pub fn margin_bottom<T: Into<String>>(self, margin_bottom: T) -> Self {
        self.insert_style("margin-bottom", margin_bottom.into())
    }

    /// 左边距
    ///
    /// # 参数
    /// * `margin_left` - 左边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().margin_left("10px");
    /// ```
    ///
    pub fn margin_left<T: Into<String>>(self, margin_left: T) -> Self {
        self.insert_style("margin-left", margin_left.into())
    }

    /// 右边距
    ///
    /// # 参数
    /// * `margin_right` - 右边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().margin_right("10px");
    /// ```
    ///
    pub fn margin_right<T: Into<String>>(self, margin_right: T) -> Self {
        self.insert_style("margin-right", margin_right.into())
    }

    /// 内边距
    ///
    /// # 参数
    /// * `padding` - 内边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().padding("10px");
    /// ```
    ///
    pub fn padding<T: Into<String>>(self, padding: T) -> Self {
        self.insert_style("padding", padding.into())
    }

    /// 上内边距
    ///
    /// # 参数
    /// * `padding_top` - 上内边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().padding_top("10px");
    /// ```
    ///
    pub fn padding_top<T: Into<String>>(self, padding_top: T) -> Self {
        self.insert_style("padding-top", padding_top.into())
    }

    /// 下内边距
    ///
    /// # 参数
    /// * `padding_bottom` - 下内边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().padding_bottom("10px");
    /// ```
    ///
    pub fn padding_bottom<T: Into<String>>(self, padding_bottom: T) -> Self {
        self.insert_style("padding-bottom", padding_bottom.into())
    }

    /// 左内边距
    ///
    /// # 参数
    /// * `padding_left` - 左内边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().padding_left("10px");
    /// ```
    ///
    pub fn padding_left<T: Into<String>>(self, padding_left: T) -> Self {
        self.insert_style("padding-left", padding_left.into())
    }

    /// 右内边距
    ///
    /// # 参数
    /// * `padding_right` - 右内边距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().padding_right("10px");
    /// ```
    ///
    pub fn padding_right<T: Into<String>>(self, padding_right: T) -> Self {
        self.insert_style("padding-right", padding_right.into())
    }
}
