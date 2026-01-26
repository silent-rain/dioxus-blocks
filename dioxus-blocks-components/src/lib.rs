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
//! - [`Button`][]: 按钮组件，支持多种类型、形状和尺寸
//! - [`ButtonType`][]: 按钮类型枚举（default、primary、success、info、warning、danger）
//! - [`ButtonShape`][]: 按钮形状枚举（default、plain、round、circle、link、text）
//! - [`ButtonSize`][]: 按钮尺寸枚举（small、medium、large）
//! - [`Grid`][]: 网格布局组件，支持自定义列数、行数和间距
//! - [`GridItem`][]: 网格项组件，支持控制网格项在网格中的位置和跨度
//! - [`Link`][]: 链接组件，支持路由跳转、字符串路径、多种类型和下划线样式
//! - [`LinkType`][]: 链接类型枚举（default、primary、success、info、warning、danger）
//! - [`LinkUnderline`][]: 下划线样式枚举（always、hover、never）
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

pub mod prelude;

// mod common;
// pub use common::*;
