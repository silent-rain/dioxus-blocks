# 组件 API 参考

本文档提供 `dioxus-blocks-components` 库中所有组件的详细 API 文档。

## 布局组件

### View - 视图容器

通用的布局容器，类似 HTML 的 div，支持裸露渲染模式。

#### 基本用法

```rust
use dioxus_blocks_components::View;

View::new()
    .class("container")
    .style(|s| s.display("flex").gap("16px").padding("24px"))
    .children(/* 子组件 */)
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
| ------ | ------ | ------ |
| `new()` | - | 创建新的 View 实例 |
| `id()` | `String` | 设置组件唯一标识符 |
| `class()` | `String` | 添加 CSS 类名 |
| `style()` | `Fn(Style) -> Style` | 设置内联样式 |
| `children()` | `impl ToElement` | 添加子组件 |
| `childrens()` | `Vec<impl ToElement>` | 批量添加同类型子组件 |
| `childrens2()` | `Vec<impl ToElement>` | 批量添加不同类型子组件 |
| `onclick()` | `EventHandler<MouseEvent>` | 设置点击事件 |
| `bare()` | `bool` | 是否裸露渲染（不使用 div 包装） |

#### 特殊功能

**裸露渲染模式：**

```rust
// 不使用 div 包装
View::new()
    .bare(true)
    .children(Text::new("直接渲染"))
    .to_element()
```

#### 完整示例

```rust
use dioxus_blocks_components::{View, Text};

#[component]
fn Container() -> Element {
    View::new()
        .class("main-container")
        .style(|s| s
            .display("flex")
            .flex_direction("column")
            .gap("24px")
            .padding("32px")
        )
        .children(Text::h1("标题"))
        .childrens([
            Text::h2("二级标题").to_element(),
            Text::p("内容").to_element(),
        ])
        .to_element()
}
```

---

### Grid - 网格布局

提供类似 Tailwind CSS 的网格系统，支持自定义列数、行数和间距。

#### 基本用法

```rust
use dioxus_blocks_components::{Grid, GridItem, GridCols};

Grid::new(vec![
    GridItem::new(Card::new().body(Text::new("卡片1"))),
    GridItem::new(Card::new().body(Text::new("卡片2"))),
    GridItem::new(Card::new().body(Text::new("卡片3"))),
])
.cols(GridCols::Col3)  // 3列
.gap(16)              // 间距16px
.to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
| ------ | ------ | ------ |
| `new()` | `Vec<GridItem>` | 创建新的 Grid 实例 |
| `id()` | `String` | 设置组件唯一标识符 |
| `class()` | `String` | 添加 CSS 类名 |
| `style()` | `Fn(Style) -> Style` | 设置内联样式 |
| `cols()` | `GridCols` | 设置列数（1-12） |
| `rows()` | `GridRows` | 设置行数（1-12） |
| `gap()` | `u16` | 设置间距（像素） |

#### GridCols 枚举

```rust
pub enum GridCols {
    Col1,   // 1列
    Col2,   // 2列
    Col3,   // 3列
    Col4,   // 4列（默认）
    Col5,   // 5列
    Col6,   // 6列
    Col7,   // 7列
    Col8,   // 8列
    Col9,   // 9列
    Col10,  // 10列
    Col11,  // 11列
    Col12,  // 12列
}
```

#### GridRows 枚举

```rust
pub enum GridRows {
    Row1,   // 1行（默认）
    Row2,   // 2行
    Row3,   // 3行
    Row4,   // 4行
    Row5,   // 5行
    Row6,   // 6行
    Row7,   // 7行
    Row8,   // 8行
    Row9,   // 9行
    Row10,  // 10行
    Row11,  // 11行
    Row12,  // 12行
}
```

#### GridItem

包装网格项目的容器。

```rust
GridItem::new(/* 子组件 */)
```

#### 完整示例

```rust
use dioxus_blocks_components::{Grid, GridCols, GridItem, Card, Text};

#[component]
fn ProductGrid() -> Element {
    let products = vec!["产品1", "产品2", "产品3", "产品4"];
    
    Grid::new(
        products.into_iter()
            .map(|name| 
                GridItem::new(
                    Card::new()
                        .header(Text::h3(name))
                        .body(Text::p("产品描述"))
                )
            )
            .collect()
    )
    .cols(GridCols::Col4)
    .gap(16)
    .to_element()
}
```

---

### Layout - 布局组件

提供基于 Row 和 Col 的水平和垂直布局系统。

#### 基本用法

```rust
use dioxus_blocks_components::{Row, Col, ColSpan, Justify};

Row::new()
    .cols(vec![
        Col::new().span(ColSpan::Col12),
    ])
    .justify(Justify::Center)
    .to_element()
```

#### 方法

**Layout 方法：**

- `new()` - 创建新的 Layout 实例
- `rows()` - 设置行
- `justify()` - 设置水平对齐
- `align()` - 设置垂直对齐

**Row 方法：**

- `new()` - 创建新的 Row
- `cols()` - 设置列

**Col 方法：**

- `new()` - 创建新的 Col
- `span()` - 设置跨度

#### 枚举类型

**ColSpan：**

```rust
pub enum ColSpan {
    Col1, Col2, Col3, Col4, Col5, Col6,
    Col7, Col8, Col9, Col10, Col11, Col12,
}
```

**Justify：**

```rust
pub enum Justify {
    Start, Center, End, SpaceBetween, SpaceAround,
}
```

---

## 基础组件

### Button - 按钮

支持多种类型、形状和尺寸的按钮组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Button, ButtonType, ButtonShape, ButtonSize};

Button::new()
    .text("点击我")
    .btn_type(ButtonType::Primary)
    .shape(ButtonShape::Round)
    .size(ButtonSize::Medium)
    .onclick(|_| println!("Clicked!"))
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | - | 创建新的 Button 实例 |
| `text()` | `impl Into<String>` | 设置按钮文本 |
| `btn_type()` | `ButtonType` | 设置按钮类型 |
| `shape()` | `ButtonShape` | 设置按钮形状 |
| `size()` | `ButtonSize` | 设置按钮尺寸 |
| `disabled()` | `bool` | 是否禁用 |
| `loading()` | `bool` | 是否显示加载状态 |
| `onclick()` | `EventHandler<MouseEvent>` | 点击事件 |

#### ButtonType 枚举

```rust
pub enum ButtonType {
    Default,   // 默认按钮
    Primary,   // 主要按钮（蓝色）
    Success,   // 成功按钮（绿色）
    Info,      // 信息按钮（青色）
    Warning,   // 警告按钮（橙色）
    Danger,    // 危险按钮（红色）
}
```

#### ButtonShape 枚举

```rust
pub enum ButtonShape {
    Default,  // 默认圆角
    Plain,    // 朴素按钮
    Round,    // 椭圆按钮
    Circle,   // 圆形按钮
    Link,     // 链接按钮
    Text,     // 文字按钮
}
```

#### ButtonSize 枚举

```rust
pub enum ButtonSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

#### 便捷方法

```rust
// 类型快捷方法
Button::new().text("Save").as_primary()
Button::new().text("Delete").as_danger()
Button::new().text("Success").as_success()

// 形状快捷方法
Button::new().text("Round").as_round()
Button::new().text("Circle").as_circle()
Button::new().text("Link").as_link()

// 尺寸快捷方法
Button::new().text("Small").as_small()
Button::new().text("Large").as_large()

// 组合使用
Button::new()
    .text("Submit")
    .as_primary()
    .as_round()
    .as_large()
```

#### 完整示例

```rust
use dioxus_blocks_components::{Button, ButtonType, View};

#[component]
fn ButtonGroup() -> Element {
    View::new()
        .style(|s| s.display("flex").gap("12px"))
        .childrens2(vec![
            Button::new().text("取消").as_text(),
            Button::new().text("确定").as_primary(),
            Button::new().text("删除").as_danger(),
        ])
        .to_element()
}
```

---

### Link - 链接

支持路由导航和外部链接的组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Link, LinkType, LinkUnderline};
use dioxus::prelude::NavigationTarget;

// 路由导航
Link::new(NavigationTarget::from(Route::Home {}))
    .text("首页")
    .as_primary()
    .to_element()

// 外部链接
Link::new(NavigationTarget::from("https://example.com"))
    .text("访问网站")
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | `NavigationTarget` | 创建新的 Link 实例 |
| `text()` | `impl Into<String>` | 设置链接文本 |
| `link_type()` | `LinkType` | 设置链接类型 |
| `underline()` | `LinkUnderline` | 设置下划线样式 |
| `disabled()` | `bool` | 是否禁用 |

#### LinkType 枚举

```rust
pub enum LinkType {
    Default,  // 默认链接
    Primary,  // 主要链接
    Success,  // 成功链接
    Info,     // 信息链接
    Warning,  // 警告链接
    Danger,   // 危险链接
}
```

#### LinkUnderline 枚举

```rust
pub enum LinkUnderline {
    Always,  // 始终显示下划线
    Hover,   // 悬停时显示下划线
    None,    // 不显示下划线
}
```

#### 完整示例

```rust
use dioxus_blocks_components::{Link, LinkType, View};

#[component]
fn Navigation() -> Element {
    View::new()
        .style(|s| s.display("flex").gap("24px"))
        .childrens2(vec![
            Link::new(NavigationTarget::from("/")).text("首页"),
            Link::new(NavigationTarget::from("/about")).text("关于").as_primary(),
            Link::new(NavigationTarget::from("/contact")).text("联系").as_underline_hover(),
        ])
        .to_element()
}
```

---

### Text - 文本

支持多种文本标签和样式的文本组件。

#### 基本用法

```rust
use dioxus_blocks_components::Text;

// 标题
Text::h1("主标题").style(|s| s.font_size("32px").font_weight("bold"))
Text::h2("副标题").style(|s| s.color("#666"))

// 段落
Text::p("这是一段文本内容")

// 普通文本
Text::new("普通文本").style(|s| s.color("blue"))
```

#### 构造方法

| 方法 | 说明 |
|------|------|
| `new(content)` | 创建普通文本（span 标签） |
| `h1(content)` | 创建 h1 标题 |
| `h2(content)` | 创建 h2 标题 |
| `h3(content)` | 创建 h3 标题 |
| `h4(content)` | 创建 h4 标题 |
| `h5(content)` | 创建 h5 标题 |
| `h6(content)` | 创建 h6 标题 |
| `p(content)` | 创建段落（p 标签） |

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `style()` | `Fn(Style) -> Style` | 设置内联样式 |
| `id()` | `String` | 设置组件 ID |
| `class()` | `String` | 添加 CSS 类名 |
| `onclick()` | `EventHandler<MouseEvent>` | 点击事件 |

#### TextTag 枚举

```rust
pub enum TextTag {
    H1, H2, H3, H4, H5, H6,  // 标题标签
    P,                            // 段落标签
    Span,                         // 普通文本（默认）
}
```

#### 完整示例

```rust
use dioxus_blocks_components::{Text, Card, View};

#[component]
fn Typography() -> Element {
    Card::new()
        .body(
            View::new()
                .childrens2(vec![
                    Text::h1("大标题").to_element(),
                    Text::h2("二级标题").to_element(),
                    Text::h3("三级标题").to_element(),
                    Text::p("这是一段段落文本").to_element(),
                    Text::new("普通文本内容").to_element(),
                ])
                .to_element()
        )
        .to_element()
}
```

---

### Image - 图片

图片显示组件，支持加载状态和多种适配模式。

#### 基本用法

```rust
use dioxus_blocks_components::Image;
use dioxus::prelude::asset;

let logo = asset!("/assets/img/logo.svg");
Image::new(logo)
    .with_width("100px")
    .with_height("100px")
    .alt("Logo")
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new(source)` | `AssetPath` | 创建新的 Image 实例 |
| `with_width()` | `String` | 设置宽度 |
| `with_height()` | `String` | 设置高度 |
| `alt()` | `String` | 设置替代文本 |
| `object_fit()` | `ObjectFit` | 设置适配模式 |

#### ObjectFit 枚举

```rust
pub enum ObjectFit {
    Fill,       // 填充容器
    Contain,    // 保持比例，完整显示
    Cover,      // 保持比例，填充容器
    None,       // 不调整
    ScaleDown,  // 缩小到完整显示
}
```

#### 完整示例

```rust
use dioxus_blocks_components::{Image, Card};

#[component]
fn ProductImage() -> Element {
    Card::new()
        .body(
            Image::new(asset!("/assets/product.jpg"))
                .with_width("100%")
                .with_height("200px")
                .alt("产品图片")
        )
        .to_element()
}
```

---

## 数据展示组件

### Card - 卡片

支持头部、主体、底部和阴影效果的卡片组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Card, CardShadow};

Card::new()
    .header(Text::h3("卡片标题"))
    .body(Text::p("卡片内容"))
    .footer(Button::new().text("确定").as_primary())
    .shadow(CardShadow::Hover)
    .border(true)
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | - | 创建新的 Card 实例 |
| `header()` | `impl ToElement + Clone + 'static` | 设置头部内容 |
| `body()` | `impl ToElement + Clone + 'static` | 设置主体内容 |
| `footer()` | `impl ToElement + Clone + 'static` | 设置底部内容 |
| `shadow()` | `CardShadow` | 设置阴影效果 |
| `border()` | `bool` | 是否显示边框 |
| `header_divider()` | `bool` | 头部和主体之间是否有分隔线 |

#### CardShadow 枚举

```rust
pub enum CardShadow {
    Always,  // 始终显示阴影（默认）
    Hover,   // 悬停时显示阴影
    Never,   // 从不显示阴影
}
```

#### 完整示例

```rust
use dioxus_blocks_components::{Card, Text, Button, View};

#[component]
fn UserCard() -> Element {
    Card::new()
        .header(Text::h3("用户信息"))
        .body(
            View::new()
                .childrens2(vec![
                    Text::p("用户名: Alice").to_element(),
                    Text::p("邮箱: alice@example.com").to_element(),
                ])
                .to_element()
        )
        .footer(
            View::new()
                .style(|s| s.display("flex").justify_content("flex-end").gap("12px"))
                .childrens2(vec![
                    Button::new().text("编辑").as_text(),
                    Button::new().text("删除").as_danger(),
                ])
                .to_element()
        )
        .shadow(CardShadow::Hover)
        .to_element()
}
```

---

## 表单组件

### Input - 输入框

单行文本输入框组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Input, InputType, InputSize};

Input::new()
    .input_type(InputType::Text)
    .placeholder("请输入用户名")
    .size(InputSize::Medium)
    .disabled(false)
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | - | 创建新的 Input 实例 |
| `input_type()` | `InputType` | 设置输入类型 |
| `placeholder()` | `String` | 设置占位文本 |
| `value()` | `Signal<String>` | 设置值（响应式） |
| `size()` | `InputSize` | 设置尺寸 |
| `disabled()` | `bool` | 是否禁用 |
| `readonly()` | `bool` | 是否只读 |
| `max_length()` | `usize` | 最大长度 |

#### InputType 枚举

```rust
pub enum InputType {
    Text,     // 文本
    Password, // 密码
    Email,    // 邮箱
    Number,   // 数字
    Tel,      // 电话
    Url,      // URL
    Search,   // 搜索
}
```

#### InputSize 枚举

```rust
pub enum InputSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

#### 完整示例

```rust
use dioxus::prelude::*;
use dioxus_blocks_components::{Input, InputType, Text};

#[component]
fn LoginForm() -> Element {
    let mut username = use_signal(String::new);
    
    rsx! {
        Text::p("用户名").to_element()
        Input::new()
            .input_type(InputType::Text)
            .value(username)
            .placeholder("请输入用户名")
            .to_element()
    }
}
```

---

### InputNumber - 数字输入框

支持精度控制、步进的数字输入框组件。

#### 基本用法

```rust
use dioxus_blocks_components::{InputNumber, InputNumberSize};

InputNumber::new()
    .min(0.0)
    .max(100.0)
    .step(1.0)
    .precision(2)
    .size(InputNumberSize::Medium)
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | - | 创建新的 InputNumber 实例 |
| `min()` | `f64` | 最小值 |
| `max()` | `f64` | 最大值 |
| `step()` | `f64` | 步进值 |
| `precision()` | `usize` | 小数位数 |
| `size()` | `InputNumberSize` | 设置尺寸 |
| `disabled()` | `bool` | 是否禁 |

#### InputNumberSize 枚举

```rust
pub enum InputNumberSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

#### InputNumberStep 枚举

```rust
pub enum InputNumberStep {
    Default,  // 默认步进
    Strict,   // 严格步进
}
```

---

### TextArea - 多行文本输入

多行文本输入框组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Textarea, TextareaSize};

Textarea::new()
    .placeholder("请输入描述")
    .rows(4)
    .size(TextareaSize::Medium)
    .to_element()
```

#### 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | - | 创建新的 Textarea 实例 |
| `placeholder()` | `String` | 占位文本 |
| `value()` | `Signal<String>` | 值（响应式） |
| `rows()` | `usize` | 行数 |
| `size()` | `TextareaSize` | 尺寸 |
| `disabled()` | `bool` | 是否禁用 |
| `max_length()` | `usize` | 最大长度 |

#### TextareaSize 枚举

```rust
pub enum TextareaSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

---

### Select - 下拉选择器

单选/多选下拉框组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Select, SelectOption, SelectSize};

let options = vec![
    SelectOption::new("option1", "选项1"),
    SelectOption::new("option2", "选项2"),
    SelectOption::new("option3", "选项3"),
];

Select::new(options)
    .placeholder("请选择一个选项")
    .size(SelectSize::Medium)
    .multiple(false)
    .to_element()
```

#### SelectOption

```rust
SelectOption::new(value, label)
```

#### Select 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | `Vec<SelectOption>` | 创建新的 Select 实例 |
| `placeholder()` | `String` | 占位文本 |
| `value()` | `Signal<Option<String>>` | 选中值 |
| `multiple()` | `bool` | 是否多选 |
| `size()` | `SelectSize` | 尺寸 |
| `disabled()` | `bool` | 是否禁用 |

#### SelectSize 枚举

```rust
pub enum SelectSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

---

### Radio - 单选框

单选框组组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Radio, RadioGroup, RadioSize};

RadioGroup::new(vec![
    Radio::new("option1", "选项1"),
    Radio::new("option2", "选项2"),
])
.size(RadioSize::Medium)
.to_element()
```

#### Radio 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new(value, label)` | `String, String` | 创建 Radio 实例 |
| `disabled()` | `bool` | 是否禁用 |

#### RadioGroup 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | `Vec<Radio>` | 创建 RadioGroup |
| `value()` | `Signal<Option<String>>` | 选中值 |
| `size()` | `RadioSize` | 尺寸 |
| `disabled()` | `bool` | 是否禁用 |

#### RadioSize 枚举

```rust
pub enum RadioSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

---

### Checkbox - 复选框

复选框组组件。

#### 基本用法

```rust
use dioxus_blocks_components::{Checkbox, CheckboxGroup, CheckboxSize};

CheckboxGroup::new(vec![
    Checkbox::new("option1", "选项1"),
    Checkbox::new("option2", "选项2"),
])
.size(CheckboxSize::Medium)
.to_element()
```

#### Checkbox 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new(value, label)` | `String, String` | 创建 Checkbox 实例 |
| `disabled()` | `bool` | 是否禁用 |

#### CheckboxGroup 方法

| 方法 | 参数 | 说明 |
|------|------|------|
| `new()` | `Vec<Checkbox>` | 创建 CheckboxGroup |
| `value()` | `Signal<Vec<String>>` | 选中值列表 |
| `size()` | `CheckboxSize` | 尺寸 |
| `disabled()` | `bool` | 是否禁用 |

#### CheckboxSize 枚举

```rust
pub enum CheckboxSize {
    Small,   // 小尺寸
    Medium,  // 中尺寸（默认）
    Large,   // 大尺寸
}
```

---

## ComponentBase 宏自动生成方法

所有组件都通过 `#[derive(ComponentBase)]` 宏自动生成以下基础方法：

| 方法 | 参数 | 说明 |
|------|------|------|
| `id()` | `String` | 设置组件 ID |
| `class()` | `String` | 添加 CSS 类名 |
| `style()` | `Fn(Style) -> Style` | 设置内联样式（接受闭包） |
| `children()` | `impl ToElement` | 添加子组件 |
| `childrens2()` | `Vec<impl ToElement>` | 批量添加子组件 |
| `onclick()` | `EventHandler<MouseEvent>` | 设置点击事件 |

---

## 通用属性

所有组件都支持以下基础属性：

```rust
MyComponent::new()
    .id("unique-id")                    // HTML id 属性
    .class("class1 class2")             // CSS class
    .style(|s| s.padding("16px"))       // 内联样式
    .onclick(|event| {                  // 点击事件
        println!("Clicked: {:?}", event);
    })
    .to_element()
```

---

## 下一章

详细了解了所有组件的 API 后，继续学习：

→ [样式系统](./02-styling-system.md) - 深入了解 Style 构建器和伪类
