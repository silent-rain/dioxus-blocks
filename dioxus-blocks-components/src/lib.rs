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
//! - [`View`][]: 容器组件，类似 HTML 的 div 或 Vue 的 template，支持裸露渲染
//! - [`Card`][]: 卡片容器组件，支持头部、主体和底部内容
//! - [`Button`][]: 按钮组件，支持文本和点击事件
//! - [`Grid`][]: 网格布局组件，支持自定义列数、行数和间距
//! - [`GridItem`][]: 网格项组件，支持控制网格项在网格中的位置和跨度
//! - [`Link`][]: 链接组件，支持路由跳转、字符串路径
//! - [`LinkDestination`][]: 链接目标类型枚举
//! - [`LinkTarget`][]: 链接打开目标枚举
//! - [`Image`][]: 图片组件，支持加载状态、替代文本、尺寸等配置
//! - [`ObjectFit`][]: 图片对象适应方式枚举
//!
//! ## 宏
//!
//! - [`ComponentBase`]: 为组件提供基础方法（id、class、style 等）
//! - [`Route`][]: 为组件自动生成对应的路由组件

mod constant;
pub use constant::{MAIN_CSS, TAILWIND_CSS};

pub use dioxus::prelude::{Element, NavigationTarget};

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
