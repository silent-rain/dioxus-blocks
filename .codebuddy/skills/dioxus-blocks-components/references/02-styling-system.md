# 样式系统深度指南

`dioxus-blocks-components` 提供了一个强大的链式调用样式构建器 `Style`，支持内联样式和伪类样式。

## Style 构建器基础

### 基本用法

```rust
use dioxus_blocks_components::Style;

Style::default()
    .width("100px")
    .height("100px")
    .background_color("white")
    .padding("16px")
    .border_radius("8px")
```

### 应用到组件

```rust
Card::new()
    .style(|s| s
        .padding("24px")
        .border_radius("12px")
        .background_color("#FAFAFA")
    )
    .to_element()
```

## 常用样式方法

### 布局相关

| 方法 | 说明 | 示例 |
|------|------|------|
| `display()` | 显示类型 | `s.display("flex")` |
| `position()` | 定位方式 | `s.position("relative")` |
| `top()` / `left()` | 定位位置 | `s.top("10px")` |
| `z_index()` | 层级 | `s.z_index("100")` |

### Flexbox

| 方法 | 说明 | 示例 |
|------|------|------|
| `flex_direction()` | 方向 | `s.flex_direction("row")` |
| `justify_content()` | 水平对齐 | `s.justify_content("center")` |
| `align_items()` | 垂直对齐 | `s.align_items("center")` |
| `flex_wrap()` | 换行 | `s.flex_wrap("wrap")` |
| `gap()` | 间距 | `s.gap("16px")` |
| `flex()` | flex 属性 | `s.flex("1")` |

### 尺寸

| 方法 | 说明 | 示例 |
|------|------|------|
| `width()` | 宽度 | `s.width("100px")` |
| `height()` | 高度 | `s.height("100px")` |
| `min_width()` | 最小宽度 | `s.min_width("200px")` |
| `max_width()` | 最大宽度 | `s.max_width("500px")` |
| `min_height()` | 最小高度 | `s.min_height("100px")` |
| `max_height()` | 最大高度 | `s.max_height("300px")` |

### 间距

| 方法 | 说明 | 示例 |
|------|------|------|
| `padding()` | 内边距 | `s.padding("16px")` |
| `margin()` | 外边距 | `s.margin("16px")` |
| `padding_top()` | 上内边距 | `s.padding_top("8px")` |
| `padding_bottom()` | 下内边距 | `s.padding_bottom("8px")` |
| `padding_left()` | 左内边距 | `s.padding_left("8px")` |
| `padding_right()` | 右内边距 | `s.padding_right("8px")` |
| `margin_top()` | 上外边距 | `s.margin_top("8px")` |
| `margin_bottom()` | 下外边距 | `s.margin_bottom("8px")` |
| `margin_left()` | 左外边距 | `s.margin_left("8px")` |
| `margin_right()` | 右外边距 | `s.margin_right("8px")` |

### 颜色和背景

| 方法 | 说明 | 示例 |
|------|------|------|
| `background_color()` | 背景色 | `s.background_color("#ffffff")` |
| `color()` | 文字颜色 | `s.color("#333333")` |
| `border()` | 边框 | `s.border("1px solid #e0e0e0")` |
| `border_radius()` | 圆角 | `s.border_radius("8px")` |
| `border_color()` | 边框颜色 | `s.border_color("#ff0000")` |
| `border_width()` | 边框宽度 | `s.border_width("2px")` |
| `border_style()` | 边框样式 | `s.border_style("dashed")` |
| `opacity()` | 透明度 | `s.opacity("0.8")` |

### 字体和文本

| 方法 | 说明 | 示例 |
|------|------|------|
| `font_size()` | 字号 | `s.font_size("16px")` |
| `font_weight()` | 字重 | `s.font_weight("bold")` |
| `font_family()` | 字体 | `s.font_family("Arial")` |
| `line_height()` | 行高 | `s.line_height("1.5")` |
| `text_align()` | 对齐 | `s.text_align("center")` |
| `text_decoration()` | 装饰 | `s.text_decoration("underline")` |
| `text_transform()` | 转换 | `s.text_transform("uppercase")` |
| `letter_spacing()` | 字间距 | `s.letter_spacing("1px")` |
| `word_spacing()` | 词间距 | `s.word_spacing("2px")` |

### 其他

| 方法 | 说明 | 示例 |
|------|------|------|
| `cursor()` | 鼠标样式 | `s.cursor("pointer")` |
| `overflow()` | 溢出处理 | `s.overflow("hidden")` |
| `box_shadow()` | 阴影 | `s.box_shadow("0 2px 4px rgba(0,0,0,0.1)")` |
| `transform()` | 变换 | `s.transform("scale(1.1)")` |
| `transition()` | 过渡 | `s.transition("all 0.3s ease")` |
| `custom()` | 自定义 CSS | `s.custom("user-select: none;")` |

## 伪类支持

Style 构建器支持 7 种伪类，可以响应不同状态。

### hover - 鼠标悬停

```rust
Button::new()
    .style(|s| s
        .background_color("white")
    )
    .to_element()
```

### active - 激活状态

```rust
Button::new()
    .style(|s| s
        .transform("scale(1)")
        .active(|s| s.transform("scale(0.98)"))
    )
    .to_element()
```

### focus - 获得焦点

```rust
Input::new()
    .style(|s| s
        .border_color("#e0e0e0")
        .focus(|s| s.border_color("blue"))
    )
    .to_element()
```

### visited - 已访问链接

```rust
Link::new(NavigationTarget::from("/page"))
    .style(|s| s
        .color("blue")
        .visited(|s| s.color("purple"))
    )
    .to_element()
```

### checked - 选中状态

```rust
Checkbox::new("agree", "同意条款")
    .style(|s| s
        .border_color("#e0e0e0")
        .checked(|s| s.border_color("blue"))
    )
    .to_element()
```

### disabled - 禁用状态

```rust
Button::new()
    .style(|s| s
        .opacity("1")
        .disabled(|s| s.opacity("0.5"))
    )
    .disabled(true)
    .to_element()
```

### enabled - 启用状态

```rust
Button::new()
    .style(|s| s
        .opacity("0.5")
        .enabled(|s| s.opacity("1"))
    )
    .disabled(false)
    .to_element()
```

## 常用样式模式

### Flex 居中

```rust
View::new()
    .style(|s| s
        .display("flex")
        .justify_content("center")
        .align_items("center")
    )
    .to_element()
```

### 固定高度滚动容器

```rust
View::new()
    .style(|s| s
        .height("400px")
        .overflow_y("auto")
    )
    .to_element()
```

### 绝对定位覆盖

```rust
View::new()
    .style(|s| s
        .position("absolute")
        .top("0")
        .left("0")
        .width("100%")
        .height("100%")
    )
    .to_element()
```

### 渐变背景

```rust
Card::new()
    .style(|s| s.custom(
        "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);"
    ))
    .to_element()
```

### 卡片悬浮效果

```rust
Card::new()
    .style(|s| s
        .transition("all 0.3s ease")
        .box_shadow("0 2px 8px rgba(0,0,0,0.1)")
    )
    .to_element()
```

### 文本溢出省略

```rust
Text::new("这是一段很长的文本内容...")
    .style(|s| s
        .white_space("nowrap")
        .overflow("hidden")
        .text_overflow("ellipsis")
    )
    .to_element()
```

### 按钮按下动画

```rust
Button::new()
    .style(|s| s
        .transition("transform 0.1s")
        .active(|s| s.transform("scale(0.95)"))
    )
    .to_element()
```

## 样式复用策略

### 提取样式闭包

```rust
let card_style = |s: Style| {
    s.padding("24px")
        .border_radius("12px")
        .background_color("#FAFAFA")
};

// 复用样式
Card::new().style(card_style).to_element()
Card::new().style(card_style).to_element()
```

### 样式常量

```rust
// 定义常用样式常量
const PRIMARY_BUTTON_STYLE: &str = "background-color: #1976d2; color: white;";
const DANGER_BUTTON_STYLE: &str = "background-color: #d32f2f; color: white;";

Button::new().style(|s| s.custom(PRIMARY_BUTTON_STYLE))
```

### 样式工厂函数

```rust
fn button_style(is_primary: bool) -> impl Fn(Style) -> Style {
    move |s: Style| {
        if is_primary {
            s.background_color("#1976d2").color("white")
        } else {
            s.background_color("#ffffff").color("#333333")
        }
    }
}

Button::new().style(button_style(true))
```

## 响应式设计

### 使用 CSS 变量

```rust
View::new()
    .style(|s| s.custom("--primary-color: #1976d2;"))
    .to_element()
```

### 媒体查询（通过自定义样式）

```rust
View::new()
    .style(|s| s.custom(
        "@media (max-width: 768px) {
            .container { padding: 16px; }
        }"
    ))
    .to_element()
```

### 流式宽度

```rust
Card::new()
    .style(|s| s
        .width("100%")
        .max_width("600px")
    )
    .to_element()
```

## 主题定制

### 颜色主题

```rust
// 定义主题颜色
const THEME_LIGHT: ThemeColors = ThemeColors {
    primary: "#1976d2",
    secondary: "#dc004e",
    background: "#ffffff",
    surface: "#f5f5f5",
    text: "#333333",
};

struct ThemeColors {
    primary: &'static str,
    secondary: &'static str,
    background: &'static str,
    surface: &'static str,
    text: &'static str,
}

// 使用主题
Card::new()
    .style(|s| s
        .background_color(THEME_LIGHT.background)
        .color(THEME_LIGHT.text)
    )
    .to_element()
```

### 深色模式

```rust
// 深色主题
const THEME_DARK: ThemeColors = ThemeColors {
    primary: "#90caf9",
    secondary: "#f48fb1",
    background: "#121212",
    surface: "#1e1e1e",
    text: "#ffffff",
};
```

## 性能优化

### 避免重复计算

```rust
// 好的方式
let style = Style::default()
    .width("100px")
    .height("100px");

// 在多处复用
Card::new().style(|s| style.clone()).to_element()
```

### 使用类名而非内联样式

```rust
// 对于大量重复的样式，使用 CSS 类
View::new()
    .class("container")
    // .style(...) // 避免复杂内联样式
    .to_element()
```

### 按需加载样式

```rust
// 仅在需要时加载样式
if needs_custom_style {
    Card::new().style(|s| s.custom("...")).to_element()
} else {
    Card::new().to_element()
}
```

## 样式调试技巧

### 检查样式应用

```rust
let style = Style::default()
    .width("100px")
    .height("100px")
    .background_color("white");

println!("Generated CSS: {}", style.to_string());
```

## 常见问题

### 1. 样式不生效

**原因：** CSS 文件未引入或样式优先级低

**解决：**

```rust
// 确保引入 CSS
document::Stylesheet {
    href: asset!("/assets/css/index.css"),
}

// 使用 !important 提高优先级（不推荐）
s.custom("color: red !important;")
```

### 3. 动态样式不更新

**原因：** 闭包捕获的值不是响应式的

**解决：**

```rust
// 使用 Signal 或移动语义
let color = use_signal(|| "#ffffff");

Card::new()
    .style(|s| s.background_color(color()))  // ✅ 每次调用获取最新值
    .to_element()
```

## 下一章

掌握了样式系统后，继续学习：

→ [状态管理与路由](./03-state-management.md) - 学习响应式状态和路由导航
