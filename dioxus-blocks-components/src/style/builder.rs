//! Style 构建器
//!
//! 提供 Style 结构体及其核心实现。

use indexmap::IndexMap;

use crate::style::css_generator::{CssGenerator, PseudoClass};

/// CSS 样式构建器
///
/// 支持链式调用的 CSS 样式构建器，包含伪类支持。
#[derive(Debug, Default, Clone)]
pub struct Style {
    styles: IndexMap<String, String>,
    pseudo_styles: IndexMap<PseudoClass, IndexMap<String, String>>,
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

        Self {
            styles: m,
            pseudo_styles: IndexMap::new(),
        }
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

    /// 插入样式属性
    pub(crate) fn insert_style(mut self, key: &str, value: String) -> Self {
        self.styles.insert(key.to_string(), value);
        self
    }

    /// 悬停伪类
    ///
    /// 定义鼠标悬停在元素上时的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义悬停样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().background_color("white")
    ///     .hover(|s| s.background_color("#f0f0f0"));
    /// ```
    pub fn hover<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let hover_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Hover, hover_style.styles);
        self
    }

    /// 激活伪类
    ///
    /// 定义元素被激活（如鼠标按下）时的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义激活样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().color("blue")
    ///     .active(|s| s.color("darkblue"));
    /// ```
    pub fn active<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let active_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Active, active_style.styles);
        self
    }

    /// 焦点伪类
    ///
    /// 定义元素获得焦点时的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义焦点样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border("1px solid #ccc")
    ///     .focus(|s| s.border_color("blue"));
    /// ```
    pub fn focus<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let focus_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Focus, focus_style.styles);
        self
    }

    /// 已访问伪类
    ///
    /// 定义已访问链接的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义已访问样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().color("blue")
    ///     .visited(|s| s.color("purple"));
    /// ```
    pub fn visited<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let visited_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Visited, visited_style.styles);
        self
    }

    /// 选中伪类
    ///
    /// 定义复选框、单选框等被选中时的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义选中样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().border("1px solid #ccc")
    ///     .checked(|s| s.border_color("green"));
    /// ```
    pub fn checked<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let checked_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Checked, checked_style.styles);
        self
    }

    /// 禁用伪类
    ///
    /// 定义元素被禁用时的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义禁用样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().color("black")
    ///     .disabled(|s| s.color("gray"));
    /// ```
    pub fn disabled<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let disabled_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Disabled, disabled_style.styles);
        self
    }

    /// 启用伪类
    ///
    /// 定义元素启用时的样式。
    ///
    /// # 参数
    ///
    /// * `f` - 定义启用样式的闭包
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().color("gray")
    ///     .enabled(|s| s.color("black"));
    /// ```
    pub fn enabled<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let enabled_style = f(Style::default());
        self.pseudo_styles
            .insert(PseudoClass::Enabled, enabled_style.styles);
        self
    }

    /// 生成 CSS 类选择器规则
    ///
    /// 将样式和伪类样式转换为 CSS 规则字符串，可用于注入 `<style>` 标签。
    ///
    /// # 参数
    ///
    /// * `class_name` - CSS 类名
    ///
    /// # 返回值
    ///
    /// 返回 CSS 规则字符串
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// let style = Style::default()
    ///     .background_color("white")
    ///     .hover(|s| s.background_color("#f0f0f0"));
    /// let css = style.to_css_class("my-card");
    /// assert!(css.contains(".my-card"));
    /// ```
    pub fn to_css_class(&self, class_name: &str) -> String {
        CssGenerator::to_css_class(self, class_name)
    }

    /// 生成 ID 选择器规则
    ///
    /// 将样式和伪类样式转换为 CSS 规则字符串，使用 ID 选择器。
    ///
    /// # 参数
    ///
    /// * `id_name` - CSS ID 名
    ///
    /// # 返回值
    ///
    /// 返回 CSS 规则字符串
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// let style = Style::default()
    ///     .background_color("white")
    ///     .hover(|s| s.background_color("#f0f0f0"));
    /// let css = style.to_css_id("unique-id");
    /// assert!(css.contains("#unique-id"));
    /// ```
    pub fn to_css_id(&self, id_name: &str) -> String {
        CssGenerator::to_css_id(self, id_name)
    }
}

// CSS 生成相关实现
impl CssGenerator for Style {
    fn to_css_class(&self, class_name: &str) -> String {
        let mut rules = Vec::new();

        if !self.styles.is_empty() {
            let styles = self
                .styles
                .iter()
                .map(|(k, v)| format!("  {}: {};", k, v))
                .collect::<Vec<String>>()
                .join("\n");
            rules.push(format!(".{} {{\n{}\n}}", class_name, styles));
        }

        for (pseudo, styles) in self.pseudo_styles.iter() {
            if !styles.is_empty() {
                let pseudo_styles = styles
                    .iter()
                    .map(|(k, v)| format!("  {}: {};", k, v))
                    .collect::<Vec<String>>()
                    .join("\n");
                rules.push(format!(
                    ".{}:{} {{\n{}\n}}",
                    class_name,
                    pseudo.to_css_string(),
                    pseudo_styles
                ));
            }
        }

        rules.join("\n")
    }

    fn to_css_id(&self, id_name: &str) -> String {
        let mut rules = Vec::new();

        if !self.styles.is_empty() {
            let styles = self
                .styles
                .iter()
                .map(|(k, v)| format!("  {}: {};", k, v))
                .collect::<Vec<String>>()
                .join("\n");
            rules.push(format!("#{} {{\n{}\n}}", id_name, styles));
        }

        for (pseudo, styles) in self.pseudo_styles.iter() {
            if !styles.is_empty() {
                let pseudo_styles = styles
                    .iter()
                    .map(|(k, v)| format!("  {}: {};", k, v))
                    .collect::<Vec<String>>()
                    .join("\n");
                rules.push(format!(
                    "#{}:{} {{\n{}\n}}",
                    id_name,
                    pseudo.to_css_string(),
                    pseudo_styles
                ));
            }
        }

        rules.join("\n")
    }
}

impl From<Style> for String {
    fn from(style: Style) -> Self {
        let mut parts = Vec::new();

        if !style.styles.is_empty() {
            let normal_styles = style
                .styles
                .iter()
                .map(|(k, v)| format!("{}: {};", k, v))
                .collect::<Vec<String>>()
                .join(" ");
            parts.push(normal_styles);
        }

        for (pseudo_class, styles) in style.pseudo_styles.iter() {
            if !styles.is_empty() {
                let pseudo_str = styles
                    .iter()
                    .map(|(k, v)| format!("{}: {};", k, v))
                    .collect::<Vec<String>>()
                    .join(" ");
                parts.push(format!(
                    "{} {{ {} }}",
                    pseudo_class.to_css_string(),
                    pseudo_str
                ));
            }
        }

        parts.join(" ")
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
