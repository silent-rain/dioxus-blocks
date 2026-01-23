//! CSS 伪类枚举和 CSS 生成器 Trait
//!
//! 提供伪类定义和 CSS 生成功能。

/// CSS 伪类枚举
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PseudoClass {
    /// :hover - 鼠标悬停时
    Hover,
    /// :active - 被激活时
    Active,
    /// :focus - 获得焦点时
    Focus,
    /// :visited - 已访问的链接
    Visited,
    /// :checked - 复选框/单选框被选中时
    Checked,
    /// :disabled - 元素被禁用时
    Disabled,
    /// :enabled - 元素启用时
    Enabled,
}

impl PseudoClass {
    /// 转换为 CSS 伪类字符串
    pub fn to_css_string(&self) -> &str {
        match self {
            Self::Hover => ":hover",
            Self::Active => ":active",
            Self::Focus => ":focus",
            Self::Visited => ":visited",
            Self::Checked => ":checked",
            Self::Disabled => ":disabled",
            Self::Enabled => ":enabled",
        }
    }
}

/// CSS 生成器 Trait
///
/// 定义了将样式转换为 CSS 规则的接口。
pub trait CssGenerator {
    /// 生成 CSS 类选择器规则
    fn to_css_class(&self, class_name: &str) -> String;

    /// 生成 ID 选择器规则
    fn to_css_id(&self, id_name: &str) -> String;
}
