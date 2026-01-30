# Dioxus Blocks Workspace

You are an expert [0.7 Dioxus](https://dioxuslabs.com/learn/0.7) assistant. Dioxus 0.7 changes every api in dioxus. Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone.

Provide concise code examples with detailed descriptions

## Project Structure

```
dioxus-blocks/
├── dioxus-blocks-components/    # 组件库 - 提供可复用的 UI 组件
├── dioxus-blocks-macro/         # 宏库 - 提供代码生成工具
└── dioxus-blocks-ui/            # UI 应用 - 使用组件库的实际应用
```

## dioxus-blocks-components

组件库层，提供可复用的 UI 组件和工具。

### 目录结构

```sh
dioxus-blocks-components/
├── src/
│   ├── common/           # 通用模块
│   ├── components/       # 组件实现
│   ├── traits.rs        # Trait 定义
│   ├── style.rs         # 样式构建器（支持伪类）
│   ├── constant.rs       # 常量定义
│   ├── lib.rs          # 库入口
│   └── outlet.rs       # 路由出口组件
├── assets/             # 静态资源
├── Cargo.toml
└── Dioxus.toml
```

### 核心组件

#### Style - 样式构建器

支持链式调用的 CSS 样式构建器，包含伪类支持。

```rust
use dioxus_blocks_components::Style;

// 基础样式
Style::default()
    .width("100px")
    .height("100px")
    .background_color("white")
    .padding("16px")
    .border_radius("8px");

// 生成的 CSS: "background-color: white; :hover { background-color: #f0f0f0; } :active { transform: scale(0.98); }"
```

**支持的伪类：**

- `hover()` - 鼠标悬停
- `active()` - 激活状态
- `focus()` - 获得焦点
- `visited()` - 已访问链接
- `checked()` - 选中状态
- `disabled()` - 禁用状态
- `enabled()` - 启用状态

#### View - 视图容器

通用的布局容器组件。

```rust
use dioxus_blocks_components::View;

View::new()
    .class("container")
    .style(|s| s.display("flex").gap("16px"))
    .children(child)
    .to_element();
```

#### Link - 链接组件

支持路由导航和外部链接。

```rust
use dioxus_blocks_components::Link;
use crate::Route;

// 路由导航
Link::default()
    .to(Route::HomeViewRoute {})
    .text("首页")
    .style(|s| s.color("blue"))
    .to_element();

// 外部链接
Link::default()
    .to("https://example.com")
    .text("访问示例")
    .to_element();
```

#### Text - 文本组件

提供不同级别的文本组件。

```rust
use dioxus_blocks_components::Text;

// h1 标题
Text::h1("大标题").style(|s| s.font_size("32px"));

// h2 标题
Text::h2("中标题").style(|s| s.font_size("24px"));

// p 段落
Text::p("这是一段文字");
```

#### Image - 图片组件

图片显示组件。

```rust
use dioxus_blocks_components::Image;
use dioxus::prelude::asset;

let logo = asset!("/assets/img/logo.svg");
Image::new(logo)
    .with_width("32px")
    .with_height("32px")
    .alt("Logo");
```

### Trait 定义

#### ToElement - 元素转换

将组件转换为 Dioxus Element。

```rust
impl ToElement for MyComponent {
    fn to_element(&self) -> Element {
        // 返回 Dioxus Element
    }
}
```

#### NavigationTarget - 导航目标

支持多种导航目标类型。

```rust
use dioxus_blocks_components::NavigationTarget;
use crate::Route;

// 路由枚举
let target: NavigationTarget = Route::HomeViewRoute {}.into();

// 字符串路径
let target: NavigationTarget = "/guide".into();

// 字符串包装
let target: NavigationTarget = NavigationTarget::<String>::from("/component");
```

### 文档示例规范

**涉及 `to_element` 的文档示例：**

```rust
/// 将组件转换为 Dioxus Element
///
/// # 返回值
///
/// 返回一个 Dioxus Element，表示渲染后的组件
///
/// # 示例
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_blocks_components::{ToElement};
/// # let mut dom = VirtualDom::new(|| {
///     MyComponent::new()
///         .text("Test")
///         .to_element()
/// # });
/// # let mut mutations = Mutations::default();
/// # dom.rebuild(&mut mutations);
/// ```
fn to_element(&self) -> Element;
```

**纯组件方法的文档示例：**

```rust
/// 设置组件显示的文本
///
/// # 参数
///
/// * `text` - 要显示的文本内容
///
/// # 返回值
///
/// 返回修改后的组件实例，支持链式调用
///
/// # 示例
///
/// ```rust
/// # use dioxus_blocks_components::MyComponent;
/// MyComponent::new().text("示例文本");
/// ```
pub fn text<T: Into<String>>(mut self, text: T) -> Self;
```

### 单元测试规范

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 纯组件方法测试
    #[test]
    fn test_component_creation() {
        let component = MyComponent::new().text("测试");
        assert_eq!(component.text, "测试");
    }

    // 涉及 to_element 的测试
    #[test]
    fn test_to_element() {
        let mut dom = VirtualDom::new(|| {
            MyComponent::new()
                .text("Test")
                .to_element()
        });
        let mut mutations = Mutations::default();
        dom.rebuild(&mut mutations);
    }
}
```

## dioxus-blocks-macro

代码生成宏库，提供组件和路由的代码生成能力。

### 目录结构

```sh
dioxus-blocks-macro/
├── src/
│   ├── component.rs    # 组件相关宏
│   ├── route.rs       # 路由相关宏
│   └── lib.rs        # 宏入口
├── Cargo.toml
└── README.md
```

### 使用方式

```toml
[dependencies]
dioxus-blocks-macro = { path = "../dioxus-blocks-macro" }
```

```rust
use dioxus_blocks_macro::component_macro;

#[derive(Component)]
struct MyComponent {
    text: String,
}
```

## dioxus-blocks-ui

实际应用层，使用组件库构建完整的 UI 应用。

### 目录结构

```sh
dioxus-blocks-ui/
├── src/
│   ├── main.rs           # 应用入口
│   ├── route.rs         # 路由定义
│   ├── layout/          # 布局组件
│   │   ├── mod.rs
│   │   ├── header.rs    # 头部
│   │   ├── navbar.rs    # 导航栏
│   │   ├── footer.rs    # 底部
│   │   ├── sidebar.rs   # 侧边栏
│   │   └── body.rs     # 主体
│   └── views/           # 页面视图
│       ├── mod.rs
│       ├── home.rs      # 首页
│       ├── blog.rs      # 博客页
│       ├── card.rs      # 卡片页
│       ├── grid.rs      # 网格页
│       ├── image.rs     # 图片页
│       └── text.rs     # 文本页
├── assets/            # 静态资源
│   ├── css/          # 样式文件
│   ├── img/          # 图片资源
│   └── tailwind.css
├── Cargo.toml
└── Dioxus.toml
```

### 应用入口

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
```

### 路由定义

```rust
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    HomeViewRoute {},

    #[route("/blog/:id")]
    BlogRoute { id: i32 },

    #[layout(Header)]
    #[route("/guide")]
    GuideRoute {},
}
```

### 布局组件示例

```rust
use crate::Route;
use dioxus_blocks_components::{Element, Link, View, ToElement};

#[derive(Debug, Default, Clone)]
pub struct Header {}

impl ToElement for Header {
    fn to_element(&self) -> Element {
        View::new()
            .style(|s| {
                s.display("flex")
                    .justify_content("space-between")
                    .align_items("center")
                    .padding("0 24px")
                    .height("64px")
                    .background_color("#ffffff")
            })
            .children(
                Link::default()
                    .to(Route::HomeViewRoute {})
                    .children(Text::h1("Dioxus Blocks"))
            )
            .children(Navbar::default())
            .to_element()
    }
}
```

### 页面组件示例

使用本地组件库 `dioxus-blocks-components` 构建页面。

```rust
use dioxus::prelude::*;
use dioxus_blocks_components::{
    Card, Element, Grid, GridCols, GridItem, Link, Style, Text, ToElement, View,
};
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
pub struct HomePage {}

impl ToElement for HomePage {
    fn to_element(&self) -> Element {
        // 定义样式闭包
        let card_style = |s: Style| {
            s.text_align("center")
                .border_radius("12px")
                .background_color("#FAFAFA")
                .padding("24px")
        };
        
        let text_style = |s: Style| s.font_size("16px").color("#333");
        
        let link_style = |s: Style| {
            s.height("100px")
                .line_height("100px")
                .text_decoration("none")
                .display("block")
        };

        // 使用 Grid 组件布局
        Grid::new(vec![
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Grid").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::GridViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Text").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::TextViewRoute {})
                    .style(link_style),
            ),
            GridItem::new(
                Link::default()
                    .children(
                        Card::new()
                            .children(Text::new("Image").style(text_style))
                            .style(card_style),
                    )
                    .to(crate::Route::ImageViewRoute {})
                    .style(link_style),
            ),
        ])
        .cols(GridCols::Col4)
        .gap(16)
        .to_element()
    }
}
```

**使用的主要组件：**

1. **Grid** - 网格布局容器

   ```rust
   Grid::new(items)
       .cols(GridCols::Col4)  // 4 列
       .gap(16)              // 间距 16px
   ```

2. **Card** - 卡片组件

   ```rust
   Card::new()
       .children(Text::new("内容"))
       .style(|s| s.padding("24px"))
   ```

3. **Link** - 链接组件（支持路由导航）

   ```rust
   Link::default()
       .to(Route::HomeViewRoute {})  // 路由导航
       .children(content)
       .style(|s| s.text_decoration("none"))
   ```

4. **Text** - 文本组件

   ```rust
   Text::new("标题").style(|s| s.font_size("20px"))
   ```

5. **Style** - 样式构建器

   ```rust
   Style::default()
       .padding("16px")
       .background_color("#fff")
   ```

**页面组件规范：**

- 使用 `#[derive(Route)]` 标记为路由组件
- 实现 `ToElement` trait
- 使用本地组件库而非原生 rsx
- 样式通过 `Style` 构建器定义
- 响应式状态使用 `Signal`
- 路由导航使用 `Link` 组件

## Dioxus 0.7 核心概念

### RSX 语法

```rust
rsx! {
    div {
        class: "container",           // 属性
        style: "color: red;",        // 内联样式
        "Hello, Dioxus!"            // 子元素
    }

    // 条件属性
    div {
        class: if condition { "active" },
    }

    // 循环
    for i in 0..5 {
        div { "{i}" }
    }

    // 条件渲染
    if condition {
        div { "Condition is true" }
    }

    // 表达式
    {some_variable}
    {(0..5).map(|i| rsx! { span { "{i}" } })}
}
```

### 组件定义

```rust
#[component]
fn MyComponent(
    mut count: Signal<i32>,
    on_click: EventHandler<i32>,
) -> Element {
    rsx! {
        div {
            class: "my-component",
            p { "Count: {count}" }
            button {
                onclick: move |_| *count.write() += 1,
                "Increment"
            }
        }
    }
}
```

### 状态管理

```rust
// 本地状态
let mut count = use_signal(|| 0);

// 计算/记忆值
let doubled = use_memo(move || count() * 2);

// 读取信号
count()          // 克隆值
count.read()      // 获取引用
count.write()      // 获取可变引用
count.with_mut(|c| *c += 1)  // 使用闭包修改
```

### Context API

```rust
// 提供上下文
use_context_provider(|| theme);

// 消费上下文
let theme: Signal<String> = use_context();
```

### 异步资源

```rust
let data = use_resource(move || async move {
    // 异步操作
    fetch_data().await
});

match data() {
    Some(value) => rsx! { Data { value } },
    None => rsx! { "Loading..." },
}
```

### 路由导航

```rust
use dioxus::prelude::*;

// 使用 Link 组件导航
Link::default()
    .to(Route::BlogRoute { id: 1 })
    .text("博客");

// 使用 navigator 导航
let navigator = use_navigator();
navigator.push(Route::HomeViewRoute {});
```

### 静态资源

```rust
use dioxus::prelude::{asset, manganis};

// 图片
let image = asset!("/assets/img/logo.png");
img { src: image, alt: "Logo" }

// CSS 样式表
document::Stylesheet {
    href: asset!("/assets/css/style.css"),
}
```

## 开发命令

```sh
# 启动开发服务器
cd dioxus-blocks-ui
dx serve

# 构建项目
cargo build

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 检查代码
cargo clippy
```

## 依赖配置

### Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "dioxus-blocks-components",
    "dioxus-blocks-macro",
    "dioxus-blocks-ui",
]

[workspace.dependencies]
dioxus = "0.7"
```

### dioxus-blocks-ui/Cargo.toml

```toml
[package]
name = "dioxus-blocks-ui"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = { workspace = true }
dioxus-blocks-components = { path = "../dioxus-blocks-components" }

[features]
default = []
server = ["dioxus/axum"]
web = ["dioxus/web"]
```

## 编码规范

- 遵循 Rust 官方 API 指南（RFC 0343）
- 所有公共 API 必须包含完整文档注释
- 使用 `#[component]` 宏定义组件
- 组件名称使用 PascalCase
- 方法名使用 snake_case
- Props 必须实现 `PartialEq` 和 `Clone`
- 使用 `Signal` 处理响应式状态
- 异步操作使用 `use_resource`
- 遵循 `CLAUDE.md` 中的编码标准

## 测试规范

### 单元测试

每个模块应包含单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let result = function_to_test();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### 组件测试

使用 Dioxus 的测试框架：

```rust
#[test]
fn test_component_rendering() {
    let mut dom = VirtualDom::new(|| {
        MyComponent::new().text("Test").to_element()
    });
    let mutations = dom.rebuild();
    // 验证渲染结果
}
```

## 部署

```sh
# 构建生产版本
cargo build --release

# 静态部署
dx build --release
```
