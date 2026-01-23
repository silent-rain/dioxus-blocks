//! 文本相关样式
//!
//! 提供字体、颜色、对齐等文本相关的样式方法。

use super::Style;

impl Style {
    /// 颜色
    ///
    /// # 参数
    /// * `color` - 颜色值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().color("#000000");
    /// ```
    ///
    pub fn color<T: Into<String>>(self, color: T) -> Self {
        self.insert_style("color", color.into())
    }

    /// 字体大小
    ///
    /// # 参数
    /// * `size` - 字体大小值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().font_size("16px");
    /// ```
    ///
    pub fn font_size<T: Into<String>>(self, size: T) -> Self {
        self.insert_style("font-size", size.into())
    }

    /// 字体粗细
    ///
    /// # 参数
    /// * `weight` - 字体粗细值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().font_weight("bold");
    /// ```
    ///
    pub fn font_weight<T: Into<String>>(self, weight: T) -> Self {
        self.insert_style("font-weight", weight.into())
    }

    /// 字体样式
    ///
    /// # 参数
    /// * `style` - 字体样式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().font_style("italic");
    /// ```
    ///
    pub fn font_style<T: Into<String>>(self, style: T) -> Self {
        self.insert_style("font-style", style.into())
    }

    /// 文本对齐
    ///
    /// # 参数
    /// * `align` - 文本对齐值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().text_align("center");
    /// ```
    ///
    pub fn text_align<T: Into<String>>(self, align: T) -> Self {
        self.insert_style("text-align", align.into())
    }

    /// 文本装饰
    ///
    /// # 参数
    /// * `decoration` - 文本装饰值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().text_decoration("underline");
    /// ```
    ///
    pub fn text_decoration<T: Into<String>>(self, decoration: T) -> Self {
        self.insert_style("text-decoration", decoration.into())
    }

    /// 文本转换
    ///
    /// # 参数
    /// * `transform` - 文本转换值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().text_transform("uppercase");
    /// ```
    ///
    pub fn text_transform<T: Into<String>>(self, transform: T) -> Self {
        self.insert_style("text-transform", transform.into())
    }

    /// 行高
    ///
    /// # 参数
    /// * `line_height` - 行高值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().line_height("1.5");
    /// ```
    ///
    pub fn line_height<T: Into<String>>(self, line_height: T) -> Self {
        self.insert_style("line-height", line_height.into())
    }

    /// 字体族
    ///
    /// # 参数
    /// * `family` - 字体族值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().font_family("Arial, sans-serif");
    /// ```
    ///
    pub fn font_family<T: Into<String>>(self, family: T) -> Self {
        self.insert_style("font-family", family.into())
    }

    /// 字母间距
    ///
    /// # 参数
    /// * `spacing` - 字母间距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().letter_spacing("2px");
    /// ```
    ///
    pub fn letter_spacing<T: Into<String>>(self, spacing: T) -> Self {
        self.insert_style("letter-spacing", spacing.into())
    }

    /// 单词间距
    ///
    /// # 参数
    /// * `spacing` - 单词间距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().word_spacing("5px");
    /// ```
    ///
    pub fn word_spacing<T: Into<String>>(self, spacing: T) -> Self {
        self.insert_style("word-spacing", spacing.into())
    }

    /// 文本缩进
    ///
    /// # 参数
    /// * `indent` - 文本缩进值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().text_indent("2em");
    /// ```
    ///
    pub fn text_indent<T: Into<String>>(self, indent: T) -> Self {
        self.insert_style("text-indent", indent.into())
    }

    /// 文本溢出
    ///
    /// # 参数
    /// * `overflow` - 文本溢出值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().text_overflow("ellipsis");
    /// ```
    ///
    pub fn text_overflow<T: Into<String>>(self, overflow: T) -> Self {
        self.insert_style("text-overflow", overflow.into())
    }

    /// 文本阴影
    ///
    /// # 参数
    /// * `shadow` - 文本阴影值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().text_shadow("2px 2px 4px rgba(0,0,0,0.5)");
    /// ```
    ///
    pub fn text_shadow<T: Into<String>>(self, shadow: T) -> Self {
        self.insert_style("text-shadow", shadow.into())
    }
}
