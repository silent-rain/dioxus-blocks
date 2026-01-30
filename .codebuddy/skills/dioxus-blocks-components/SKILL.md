---
name: dioxus-blocks-components
description: |
  dioxus-blocks-components 组件库专家技能，提供完整的组件 API 文档和最佳实践。
  
  Use when:
  - 开发 Dioxus Web/桌面应用
  - 需要构建现代化 UI 界面
  - 处理表单、布局、导航等场景
  
  Features:
  - 15+ 丰富 UI 组件
  - 链式样式构建器
  - 完整的状态管理指南
  - 响应式布局系统
---

# Dioxus Blocks Components 快速开始

你是一位精通 `dioxus-blocks-components` 组件库的专家，能够熟练使用所有组件构建现代化的UI应用。

## 核心原则

本技能基于以下核心原则：

1. **所有组件实现 `ToElement` trait** - 提供统一的 `to_element()` 方法
2. **使用 `ComponentBase` 宏** - 自动生成基础方法（id、class、style、children、onclick）
3. **响应式状态管理** - 使用 Signal、use_resource、use_context
4. **类型安全优先** - 使用枚举而非字符串
5. **组件组合模式** - 灵活组合构建复杂界面

## 快速示例

```rust
use dioxus::prelude::*;
use dioxus_blocks_components::{Button, Card, Grid, GridCols, GridItem, Text, View};

#[derive(Debug, Default, Clone)]
pub struct App {}

impl ToElement for App {
    fn to_element(&self) -> Element {
        HomePage::default().to_element()
    }
}

#[derive(Debug, Default, Clone)]
pub struct HomePage {}

impl ToElement for HomePage {
    fn to_element(&self) -> Element {
        Grid::new(vec![
            GridItem::new(Card::new()
                .header(Text::h3("卡片1"))
                .body(Text::p("这是卡片1的内容"))),
            GridItem::new(Card::new()
                .header(Text::h3("卡片2"))
                .body(Text::p("这是卡片2的内容"))),
            GridItem::new(Card::new()
                .header(Text::h3("卡片3"))
                .body(Text::p("这是卡片3的内容"))),
        ])
        .cols(GridCols::Col3)
        .gap(16)
        .to_element()
    }
}

fn main() {
    dioxus::launch(App::default().to_element());
}
```

## 组件速查表

| 组件 | 用途 | 复杂度 | 适用场景 |
| ------ | ------ | -------- | ---------- |
| **布局组件** | | | |
| View | 通用容器 | ⭐ | 基础布局、包装器 |
| Grid | 网格布局 | ⭐⭐⭐ | 卡片网格、响应式布局 |
| Layout | Flex 布局 | ⭐⭐ | 水平/垂直布局 |
| **基础组件** | | | |
| Button | 按钮 | ⭐⭐ | 操作触发、表单提交 |
| Link | 链接 | ⭐⭐ | 路由导航、外部链接 |
| Text | 文本 | ⭐ | 标题、段落、标签 |
| Image | 图片 | ⭐ | 图片展示、头像 |
| **数据展示** | | | |
| Card | 卡片 | ⭐⭐ | 内容容器、信息展示 |
| **表单组件** | | | |
| Input | 输入框 | ⭐⭐ | 文本输入、搜索 |
| InputNumber | 数字输入 | ⭐⭐⭐ | 数值输入、计数器 |
| TextArea | 多行文本 | ⭐⭐ | 长文本、描述输入 |
| Select | 下拉选择 | ⭐⭐⭐ | 单选/多选 |
| Radio | 单选框 | ⭐⭐ | 单选选项 |
| Checkbox | 复选框 | ⭐⭐ | 多选选项 |

## 核心概念速览

### ToElement Trait

所有组件都实现了 `ToElement` trait：

```rust
use dioxus_blocks_components::ToElement;

// 在 rsx! 中使用
rsx! {
    {MyComponent::new().to_element()}
}

// 作为子组件
View::new()
    .children(MyComponent::new())
    .to_element()
```

### ComponentBase 宏

自动生成的基础方法：

```rust
MyComponent::new()
    .id("my-id")                    // 设置 ID
    .class("my-class")              // 添加类名
    .style(|s| s.padding("16px"))    // 设置样式
    .children(child_component)        // 添加子组件
    .onclick(|_| println!("Clicked")) // 添加点击事件
    .to_element()
```

### 样式构建器

```rust
use dioxus_blocks_components::Style;

Style::default()
    .width("100px")
    .height("100px")
    .background_color("white")
    .active(|s| s.transform("scale(0.98)"))
```

## 详细文档导航

### 📚 核心文档

1. **[组件 API 参考](./references/01-components-api.md)** - 所有组件的详细文档
   - 布局组件：View、Grid、Layout
   - 基础组件：Button、Link、Text、Image
   - 数据展示：Card
   - 表单组件：Input、InputNumber、TextArea、Select、Radio、Checkbox
   - 枚举类型和参数说明

2. **[样式系统](./references/02-styling-system.md)** - 样式构建器深度指南
   - Style 构建器基础
   - 常用样式模式
   - 主题定制

3. **[状态管理与路由](./references/03-state-management.md)** - 状态和导航
   - Signal 响应式状态
   - Context API 全局状态
   - use_resource 异步数据
   - 路由导航和编程式导航

### 🎨 开发指南

1. **[组件组合模式](./references/04-patterns.md)** - 设计模式和最佳实践
   - 嵌套组件
   - 条件渲染
   - 列表渲染
   - 高阶组件

2. **[最佳实践与规范](./references/05-best-practices.md)** - 编码规范
   - 命名规范
   - 样式组织
   - 事件处理
   - 类型安全
   - 代码复用

### 🔧 故障排查与测试

1. **[问题排查指南](./references/06-troubleshooting.md)** - 常见问题解决方案
   - 样式问题
   - 事件问题
   - 渲染问题
   - 状态问题
   - 路由问题
   - 性能问题

2. **[测试策略](./references/07-testing.md)** - 测试指南
   - 单元测试
   - 集成测试
   - 端到端测试
   - 测试工具和 Mock 策略

### 📦 项目资源

1. **[项目开发指南](./references/08-project-guide.md)** - 项目结构和工作流
   - 推荐项目结构
   - 开发流程
   - 构建与部署
   - Git 工作流
   - 性能优化

2. **[完整示例代码](./assets/examples.rs)** - 可运行的完整示例
   - 基础布局示例
   - 表单组件示例
   - 状态管理示例
   - 路由导航示例

3. **[代码片段集合](./assets/snippets.md)** - 常用代码片段
    - 布局相关
    - 表单相关
    - 状态相关
    - 样式相关

## 常见使用场景

### 场景1：构建管理后台

```text
[Header]
├── [Sidebar]
│   ├── Dashboard
│   ├── Users
│   └── Settings
└── [Main Content]
    ├── Grid Cards
    └── Data Tables
```

**参考：** [布局组件 API](./references/01-components-api.md#布局组件)、[Grid 系统](./references/01-components-api.md#grid-网格布局)

### 场景2：表单页面

```text
[Form Container]
├── [Card]
│   ├── [Header] - 表单标题
│   ├── [Body]
│   │   ├── Input - 用户名
│   │   ├── Input - 邮箱
│   │   ├── Select - 角色
│   │   └── TextArea - 描述
│   └── [Footer]
│       ├── Button - 取消
│       └── Button - 提交
```

**参考：** [表单组件 API](./references/01-components-api.md#表单组件)、[组合模式](./references/04-patterns.md)

### 场景3：产品列表

```text
[Grid cols=4 gap=16]
├── [Card]
│   ├── [Image] - 产品图片
│   └── [Text] - 产品信息
├── [Card]
│   ├── [Image]
│   └── [Text]
└── ...
```

**参考：** [Grid 组件](./references/01-components-api.md#grid-网格布局)、[列表渲染](./references/04-patterns.md#列表渲染)

## 获取帮助

遇到问题时，请按以下顺序查找解决方案：

1. **组件使用问题** → [组件 API 参考](./references/01-components-api.md)
2. **样式问题** → [样式系统](./references/02-styling-system.md) 或 [问题排查](./references/06-troubleshooting.md#样式问题)
3. **状态管理问题** → [状态管理与路由](./references/03-state-management.md)
4. **常见错误** → [问题排查指南](./references/06-troubleshooting.md)
5. **需要示例** → [完整示例代码](./assets/examples.rs) 或 [代码片段集合](./assets/snippets.md)

## 记住要点

✅ **所有组件必须调用 `.to_element()` 方法**
✅ **使用枚举而非字符串保持类型安全**
✅ **Signal 管理响应式状态**
✅ **NavigationTarget 处理路由导航**
✅ **Style 构建器支持伪类**
✅ **ComponentBase 宏提供基础方法**

---

**开始探索**：从 [组件 API 参考](./references/01-components-api.md) 开始深入学习！
