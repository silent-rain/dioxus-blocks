//! Style 构建器
//!
//! 提供 Style 结构体及其核心实现。

use indexmap::IndexMap;

/// CSS 样式构建器
///
/// 支持链式调用的 CSS 样式构建器，包含伪类支持。
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

    /// 生成内联样式字符串
    ///
    /// 将样式属性转换为内联样式格式的字符串
    ///
    /// # 返回值
    ///
    /// 返回格式化的内联样式字符串
    pub(crate) fn to_inline_style(&self) -> String {
        let mut parts = Vec::new();
        if !self.styles.is_empty() {
            let normal_styles = self
                .styles
                .iter()
                .map(|(k, v)| format!("{}: {};", k, v))
                .collect::<Vec<String>>()
                .join(" ");
            parts.push(normal_styles);
        }
        parts.join(" ")
    }

    /// 插入样式属性
    pub(crate) fn insert_style(mut self, key: &str, value: String) -> Self {
        self.styles.insert(key.to_string(), value);
        self
    }
}

impl From<Style> for String {
    fn from(style: Style) -> Self {
        style.to_inline_style()
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

    #[test]
    fn test_inline_style_format() {
        // 没有 class 或 id，返回内联样式
        let style = Style::default()
            .width("100px")
            .height("100px")
            .background_color("white");

        let result: String = style.into();
        assert_eq!(
            result,
            "width: 100px; height: 100px; background-color: white;"
        );
        assert!(!result.contains("."));
        assert!(!result.contains("#"));
    }

    #[test]
    fn test_empty_style_returns_empty_string() {
        // 空 style 对象
        let style = Style::default();
        let result: String = style.into();
        assert_eq!(result, "");
    }
}
