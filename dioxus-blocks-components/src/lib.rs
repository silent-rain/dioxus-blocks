//! Dioxus Blocks 组件库
//!
//! 这是一个基于 Dioxus 的组件库，提供常用的 UI 组件和过程宏自动生成路由组件。
//!
//! ## 特性
//!
//! - 提供常用的 UI 组件（Card、Button 等）
//! - 过程宏自动为组件生成对应的路由组件
//! - 支持多平台（Web、Desktop、Mobile）
//!
//! ## 组件
//!
//! - [`Card`][]: 卡片容器组件，支持头部、主体和底部内容
//! - [`Button`][]: 按钮组件，支持文本和点击事件
//! - [`Grid`][]: 网格布局组件，支持自定义列数、行数和间距
//! - [`GridItem`][]: 网格项组件，支持控制网格项在网格中的位置和跨度
//!
//! ## 宏
//!
//! - [`ComponentBase`]: 为组件提供基础方法（id、class、style 等）
//! - [`Route`][]: 为组件自动生成对应的路由组件

mod constant;
pub use constant::{MAIN_CSS, TAILWIND_CSS};

pub use dioxus::prelude::Element;

mod style;
pub use style::Style;

mod outlet;
pub use outlet::Outlet;

mod traits;
pub use traits::ToElement;

mod components;
pub use components::*;

// mod common;
// pub use common::*;
