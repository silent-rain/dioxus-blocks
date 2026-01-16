//! CSS 样式模块
//!
//! 提供组件库的样式定义和样式相关的辅助函数。
//!
//! 此模块目前为空，未来可以添加样式相关的辅助函数和常量。

use indexmap::IndexMap;

#[derive(Debug, Default, Clone)]
pub struct Style {
    styles: IndexMap<String, String>,
}

impl Style {
    /// 创建一个新的样式实例
    ///
    /// # 参数
    ///
    /// * `style` - 样式字符串，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    ///
    /// 返回一个新的样式实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Style;
    /// Style::new("width: 100px; height: 100px;");
    /// ```
    pub fn new<T: Into<String>>(styles: T) -> Self {
        let style_str = styles.into();
        let mut m: IndexMap<String, String> = IndexMap::new();
        style_str.split(";").for_each(|style| {
            if style.is_empty() {
                return;
            }
            let parts: Vec<&str> = style.split(":").collect();
            if parts.len() >= 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                m.insert(key.to_string(), value.to_string());
            }
        });

        Self { styles: m }
    }

    /// 自定义样式
    ///
    /// # 返回值
    ///
    /// 返回样式字符串
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Style;
    /// let style = Style::default().custom("width: 100px; height: 100px;");
    /// let css_string = style.to_string();
    /// ```
    pub fn custom<T: Into<String>>(mut self, styles: T) -> Self {
        let style_str = styles.into();
        let mut m: IndexMap<String, String> = IndexMap::new();
        style_str.split(";").for_each(|style| {
            if style.is_empty() {
                return;
            }
            let parts: Vec<&str> = style.split(":").collect();
            if parts.len() >= 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                m.insert(key.to_string(), value.to_string());
            }
        });
        self.styles.extend(m);
        self
    }
}

impl Style {
    /// 宽度
    ///
    /// # 参数
    /// * `width` - 宽度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().width("100px");
    /// ```
    ///
    pub fn width<T: Into<String>>(mut self, width: T) -> Self {
        self.styles.insert("width".to_string(), width.into());
        self
    }

    /// 高度
    ///
    /// # 参数
    /// * `height` - 高度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().height("100px");
    /// ```
    ///
    pub fn height<T: Into<String>>(mut self, height: T) -> Self {
        self.styles.insert("height".to_string(), height.into());
        self
    }

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
    pub fn background_color<T: Into<String>>(mut self, color: T) -> Self {
        self.styles
            .insert("background-color".to_string(), color.into());
        self
    }

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
    pub fn color<T: Into<String>>(mut self, color: T) -> Self {
        self.styles.insert("color".to_string(), color.into());
        self
    }

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
    pub fn margin<T: Into<String>>(mut self, margin: T) -> Self {
        self.styles.insert("margin".to_string(), margin.into());
        self
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
    pub fn margin_top<T: Into<String>>(mut self, margin_top: T) -> Self {
        self.styles
            .insert("margin-top".to_string(), margin_top.into());
        self
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
    pub fn margin_bottom<T: Into<String>>(mut self, margin_bottom: T) -> Self {
        self.styles
            .insert("margin-bottom".to_string(), margin_bottom.into());
        self
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
    pub fn margin_left<T: Into<String>>(mut self, margin_left: T) -> Self {
        self.styles
            .insert("margin-left".to_string(), margin_left.into());
        self
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
    pub fn margin_right<T: Into<String>>(mut self, margin_right: T) -> Self {
        self.styles
            .insert("margin-right".to_string(), margin_right.into());
        self
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
    pub fn padding<T: Into<String>>(mut self, padding: T) -> Self {
        self.styles.insert("padding".to_string(), padding.into());
        self
    }

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
    pub fn border<T: Into<String>>(mut self, border: T) -> Self {
        self.styles.insert("border".to_string(), border.into());
        self
    }

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
    pub fn border_radius<T: Into<String>>(mut self, radius: T) -> Self {
        self.styles
            .insert("border-radius".to_string(), radius.into());
        self
    }

    /// 显示方式
    ///
    /// # 参数
    /// * `display` - 显示方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().display("flex");
    /// ```
    ///
    pub fn display<T: Into<String>>(mut self, display: T) -> Self {
        self.styles.insert("display".to_string(), display.into());
        self
    }

    /// flex 方向
    ///
    /// # 参数
    /// * `direction` - flex 方向值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().flex_direction("row");
    /// ```
    ///
    pub fn flex_direction<T: Into<String>>(mut self, direction: T) -> Self {
        self.styles
            .insert("flex-direction".to_string(), direction.into());
        self
    }

    /// 对齐方式
    ///
    /// # 参数
    /// * `align` - 对齐方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().align_items("center");
    /// ```
    ///
    pub fn align_items<T: Into<String>>(mut self, align: T) -> Self {
        self.styles.insert("align-items".to_string(), align.into());
        self
    }

    /// 间距
    ///
    /// # 参数
    /// * `gap` - 间距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().gap("16px");
    /// ```
    ///
    pub fn gap<T: Into<String>>(mut self, gap: T) -> Self {
        self.styles.insert("gap".to_string(), gap.into());
        self
    }

    /// 定位方式
    ///
    /// # 参数
    /// * `position` - 定位方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().position("relative");
    /// ```
    ///
    pub fn position<T: Into<String>>(mut self, position: T) -> Self {
        self.styles.insert("position".to_string(), position.into());
        self
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
    pub fn font_size<T: Into<String>>(mut self, size: T) -> Self {
        self.styles.insert("font-size".to_string(), size.into());
        self
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
    pub fn font_weight<T: Into<String>>(mut self, weight: T) -> Self {
        self.styles.insert("font-weight".to_string(), weight.into());
        self
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
    pub fn text_align<T: Into<String>>(mut self, align: T) -> Self {
        self.styles.insert("text-align".to_string(), align.into());
        self
    }

    /// 透明度
    ///
    /// # 参数
    /// * `opacity` - 透明度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().opacity("0.5");
    /// ```
    ///
    pub fn opacity<T: Into<String>>(mut self, opacity: T) -> Self {
        self.styles.insert("opacity".to_string(), opacity.into());
        self
    }

    /// 阴影
    ///
    /// # 参数
    /// * `shadow` - 阴影值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().box_shadow("0 2px 4px rgba(0,0,0,0.1)");
    /// ```
    ///
    pub fn box_shadow<T: Into<String>>(mut self, shadow: T) -> Self {
        self.styles.insert("box-shadow".to_string(), shadow.into());
        self
    }

    /// Z轴层级
    ///
    /// # 参数
    /// * `index` - Z轴层级值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().z_index("10");
    /// ```
    ///
    pub fn z_index<T: Into<String>>(mut self, index: T) -> Self {
        self.styles.insert("z-index".to_string(), index.into());
        self
    }

    /// 溢出处理
    ///
    /// # 参数
    /// * `overflow` - 溢出处理值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().overflow("hidden");
    /// ```
    ///
    pub fn overflow<T: Into<String>>(mut self, overflow: T) -> Self {
        self.styles.insert("overflow".to_string(), overflow.into());
        self
    }

    /// 光标样式
    ///
    /// # 参数
    /// * `cursor` - 光标样式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().cursor("pointer");
    /// ```
    ///
    pub fn cursor<T: Into<String>>(mut self, cursor: T) -> Self {
        self.styles.insert("cursor".to_string(), cursor.into());
        self
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
    pub fn line_height<T: Into<String>>(mut self, line_height: T) -> Self {
        self.styles
            .insert("line-height".to_string(), line_height.into());
        self
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
    pub fn text_decoration<T: Into<String>>(mut self, decoration: T) -> Self {
        self.styles
            .insert("text-decoration".to_string(), decoration.into());
        self
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
    pub fn text_transform<T: Into<String>>(mut self, transform: T) -> Self {
        self.styles
            .insert("text-transform".to_string(), transform.into());
        self
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
    pub fn font_family<T: Into<String>>(mut self, family: T) -> Self {
        self.styles.insert("font-family".to_string(), family.into());
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
    /// # use dioxus_blocks_components::Style;
    /// Style::default().object_fit("cover");
    /// ```
    pub fn object_fit<T: Into<String>>(mut self, object_fit: T) -> Self {
        self.styles
            .insert("object-fit".to_string(), object_fit.into());
        self
    }
}

impl From<Style> for String {
    fn from(style: Style) -> Self {
        let styles = style
            .styles
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect::<Vec<String>>();

        styles.join(" ")
    }
}

impl From<&Style> for String {
    fn from(style: &Style) -> Self {
        let s: String = style.clone().into();
        s
    }
}

impl From<String> for Style {
    fn from(str: String) -> Self {
        Style::new(str)
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style() {
        let style = Style::new("width: 100px; height: 100px; color: red;");
        assert_eq!(
            style.to_string(),
            "width: 100px; height: 100px; color: red;"
        );
    }

    #[test]
    fn test_style_methods() {
        let style = Style::default()
            .width("200px")
            .height("150px")
            .background_color("#ffffff")
            .color("#333333")
            .margin("10px")
            .padding("15px")
            .border("1px solid #ccc")
            .border_radius("5px")
            .display("flex")
            .font_size("16px")
            .text_align("center");

        assert_eq!(
            style.to_string(),
            "width: 200px; height: 150px; background-color: #ffffff; color: #333333; margin: 10px; padding: 15px; border: 1px solid #ccc; border-radius: 5px; display: flex; font-size: 16px; text-align: center;"
        );
    }
}
