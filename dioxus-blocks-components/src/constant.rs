//! 常量定义
//!
//! 提供项目中使用的常量，主要是 CSS 资源文件。

use dioxus::prelude::*;

/// 主样式文件
///
/// 包含组件库的主要样式定义，使用 SCSS 格式
pub const MAIN_CSS: Asset = asset!("/assets/css/index.scss");

/// Tailwind CSS 样式文件
///
/// 包含 Tailwind CSS 框架的样式定义
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
