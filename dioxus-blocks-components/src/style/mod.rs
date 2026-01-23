//! CSS 样式模块
//!
//! 提供组件库的样式定义和样式相关的辅助函数。
//!
//! ## 模块结构
//!
//! - `builder` - Style 结构体和核心实现
//! - `layout` - 布局样式（display、flex、position 等）
//! - `spacing` - 间距样式（margin、padding）
//! - `border` - 边框样式（border、border-radius）
//! - `background` - 背景样式（background-color、background-image 等）
//! - `text` - 文本样式（font、color、text-align 等）
//! - `visual` - 视觉效果样式（opacity、box-shadow、transition 等）

mod background;
mod border;
mod builder;
mod layout;
mod spacing;
mod text;
mod visual;

pub use builder::Style;
