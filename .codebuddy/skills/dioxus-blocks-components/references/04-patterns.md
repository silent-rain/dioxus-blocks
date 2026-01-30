# 组件组合模式

本章介绍 Dioxus 中的组件组合模式和设计模式。

## 嵌套组件

### 基础嵌套

```rust
use dioxus_blocks_components::{Card, View, Text, Button};

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
    .to_element()
```

### 多层嵌套

```rust
View::new()
    .style(|s| s.display("flex").gap("24px"))
    .childrens2(vec![
        Card::new()
            .header(Text::h3("左侧卡片"))
            .body(Text::p("左侧内容"))
            .to_element(),
        View::new()
            .style(|s| s.display("flex").flex_direction("column").gap("16px"))
            .childrens2(vec![
                Card::new()
                    .header(Text::h3("右上卡片"))
                    .body(Text::p("右上内容"))
                    .to_element(),
                Card::new()
                    .header(Text::h3("右下卡片"))
                    .body(Text::p("右下内容"))
                    .to_element(),
            ])
            .to_element(),
    ])
    .to_element()
```

## 条件渲染

### if 表达式

```rust
use dioxus::prelude::*;
use dioxus_blocks_components::{View, Text, Button};

#[component]
fn ConditionalComponent(show_detail: bool) -> Element {
    View::new()
        .children([
            Text::new("基本信息").to_element(),
            if show_detail {
                Some(Text::new("详细信息").to_element())
            } else {
                None
            },
        ])
        .to_element()
}
```

### Option 和 Then

```rust
#[component]
fn OptionalData(data: Option<String>) -> Element {
    View::new()
        .children([
            Text::new("标题").to_element(),
            data.map(|d| Text::p(d).to_element()),
        ])
        .to_element()
}
```

### match 表达式

```rust
#[derive(PartialEq, Clone)]
enum Status {
    Loading,
    Success(String),
    Error(String),
}

#[component]
fn StatusDisplay(status: Status) -> Element {
    match status {
        Status::Loading => Text::new("加载中...").to_element(),
        Status::Success(msg) => Text::new(msg).style(|s| s.color("green")).to_element(),
        Status::Error(err) => Text::new(err).style(|s| s.color("red")).to_element(),
    }
}
```

### 多条件组合

```rust
#[component]
fn ConditionalForm(user: Option<User>, is_edit_mode: bool) -> Element {
    View::new()
        .children([
            // 已登录且编辑模式
            if user.is_some() && is_edit_mode {
                Some(render_edit_form())
            } else {
                None
            },
            // 已登录且非编辑模式
            if user.is_some() && !is_edit_mode {
                Some(render_view_mode())
            } else {
                None
            },
            // 未登录
            if user.is_none() {
                Some(render_login_prompt())
            } else {
                None
            },
        ])
        .to_element()
}
```

## 列表渲染

### 基础列表

```rust
use dioxus_blocks_components::{View, Card, Text};

#[component]
fn UserList(users: Vec<User>) -> Element {
    View::new()
        .style(|s| s.display("flex").flex_direction("column").gap("16px"))
        .childrens2(
            users.into_iter()
                .map(|user| 
                    Card::new()
                        .header(Text::h3(&user.name))
                        .body(Text::p(&user.email))
                        .to_element()
                )
                .collect()
        )
        .to_element()
}
```

### Grid 列表

```rust
use dioxus_blocks_components::{Grid, GridCols, GridItem, Card, Text};

#[component]
fn ProductGrid(products: Vec<Product>) -> Element {
    Grid::new(
        products.into_iter()
            .map(|product| 
                GridItem::new(
                    Card::new()
                        .header(Text::h3(&product.name))
                        .body(Text::p(&product.description))
                )
            )
            .collect()
    )
    .cols(GridCols::Col4)
    .gap(16)
    .to_element()
}
```

### 索引列表

```rust
#[component]
fn NumberedItems(items: Vec<String>) -> Element {
    View::new()
        .style(|s| s.display("flex").flex_direction("column").gap("8px"))
        .childrens2(
            items.iter()
                .enumerate()
                .map(|(i, item)| 
                    Text::new(format!("{}. {}", i + 1, item)).to_element()
                )
                .collect()
        )
        .to_element()
}
```

### 动态列表

```rust
#[component]
fn DynamicList() -> Element {
    let mut items = use_signal(|| vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
    ]);
    
    View::new()
        .children([
            Button::new()
                .text("添加项")
                .onclick(move |_| {
                    let len = items().len();
                    items.write().push(format!("Item {}", len + 1));
                })
                .to_element(),
            // 列表渲染
            View::new()
                .style(|s| s.display("flex").flex_direction("column").gap("8px"))
                .childrens2(
                    items().iter()
                        .map(|item| Text::new(item).to_element())
                        .collect()
                )
                .to_element(),
        ])
        .to_element()
}
```

## 高阶组件模式

### 布局包装器

```rust
// 布局包装器组件
#[component]
fn WithLayout(title: String, children: Element) -> Element {
    Card::new()
        .header(Text::h3(&title))
        .body(children)
        .to_element()
}

// 使用
WithLayout {
    title: "用户信息".to_string(),
    children: rsx! {
        Text::p("用户: Alice").to_element()
    }
}
```

### 样式包装器

```rust
fn with_card_style<T: ToElement>(component: T) -> Element {
    Card::new()
        .style(|s| s
            .padding("24px")
            .border_radius("12px")
            .shadow("0 2px 8px rgba(0,0,0,0.1)")
        )
        .body(component)
        .to_element()
}

// 使用
with_card_style(Text::p("内容"))
```

### 权限包装器

```rust
#[component]
fn RequireAuth(required: bool, children: Element) -> Element {
    if required {
        children
    } else {
        Card::new()
            .body(Text::p("您没有权限访问此内容").style(|s| s.color("red")))
            .to_element()
    }
}
```

## Render Props 模式

```rust
// 接受渲染函数的组件
#[derive(PartialEq, Props, Clone)]
struct ListProps<T: Clone + PartialEq> {
    items: Vec<T>,
    render: fn(&T) -> Element,
}

fn List<T: Clone + PartialEq>(cx: ListProps<T>) -> Element {
    rsx! {
        View::new()
            .style(|s| s.display("flex").flex_direction("column").gap("8px"))
            .childrens2(
                cx.items.iter()
                    .map(cx.render)
                    .collect()
            )
            .to_element()
    }
}

// 使用
#[component]
fn UserListRenderProps() -> Element {
    let users = vec![
        User { name: "Alice".to_string() },
        User { name: "Bob".to_string() },
    ];
    
    List {
        items: users,
        render: |user| Text::new(&user.name).to_element()
    }
}
```

## Slot 模式

### 单插槽

```rust
#[derive(PartialEq, Props, Clone)]
struct CardWithHeaderProps {
    title: String,
    children: Element,
}

fn CardWithHeader(cx: CardWithHeaderProps) -> Element {
    Card::new()
        .header(Text::h3(&cx.title))
        .body(cx.children)
        .to_element()
}

// 使用
CardWithHeader {
    title: "标题".to_string(),
    children: rsx! {
        Text::p("内容").to_element()
    }
}
```

### 多插槽

```rust
#[derive(PartialEq, Props, Clone)]
struct ModalProps {
    header: Element,
    body: Element,
    footer: Element,
}

fn Modal(cx: ModalProps) -> Element {
    View::new()
        .style(|s| s
            .position("fixed")
            .top("0")
            .left("0")
            .width("100%")
            .height("100%")
            .background_color("rgba(0,0,0,0.5)")
            .display("flex")
            .justify_content("center")
            .align_items("center")
        )
        .children([
            Card::new()
                .header(cx.header)
                .body(cx.body)
                .footer(cx.footer)
                .style(|s| s.width("500px"))
                .to_element()
        ])
        .to_element()
}
```

## 表单组合

### 表单字段组合

```rust
#[component]
fn FormField(label: String, children: Element) -> Element {
    View::new()
        .style(|s| s.display("flex").flex_direction("column").gap("8px"))
        .children([
            Text::new(&label).style(|s| s.font_weight("bold")).to_element(),
            children,
        ])
        .to_element()
}

// 使用
FormField {
    label: "用户名".to_string(),
    children: rsx! {
        Input::new()
            .placeholder("请输入用户名")
            .to_element()
    }
}
```

### 完整表单

```rust
#[component]
fn UserForm() -> Element {
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    
    Card::new()
        .header(Text::h3("用户注册"))
        .body(
            View::new()
                .style(|s| s.display("flex").flex_direction("column").gap("16px"))
                .childrens2(vec![
                    FormField {
                        label: "用户名".to_string(),
                        children: rsx! {
                            Input::new()
                                .value(username)
                                .placeholder("请输入用户名")
                                .to_element()
                        }
                    }.to_element(),
                    FormField {
                        label: "邮箱".to_string(),
                        children: rsx! {
                            Input::new()
                                .input_type(InputType::Email)
                                .value(email)
                                .placeholder("请输入邮箱")
                                .to_element()
                        }
                    }.to_element(),
                    FormField {
                        label: "密码".to_string(),
                        children: rsx! {
                            Input::new()
                                .input_type(InputType::Password)
                                .value(password)
                                .placeholder("请输入密码")
                                .to_element()
                        }
                    }.to_element(),
                ])
                .to_element()
        )
        .footer(
            View::new()
                .style(|s| s.display("flex").justify_content("flex-end").gap("12px"))
                .childrens2(vec![
                    Button::new().text("取消").as_text().to_element(),
                    Button::new().text("注册").as_primary().to_element(),
                ])
                .to_element()
        )
        .to_element()
}
```

## 列表项组合

### 可复用列表项

```rust
#[derive(Clone, PartialEq)]
struct ListItemData {
    id: i32,
    title: String,
    subtitle: String,
}

#[component]
fn ListItem(item: ListItemData) -> Element {
    Card::new()
        .style(|s| s
            .padding("16px")
        )
        .body(
            View::new()
                .style(|s| s.display("flex").justify_content("space_between"))
                .children([
                    View::new()
                        .style(|s| s.display("flex").flex_direction("column"))
                        .childrens2(vec![
                            Text::new(&item.title).style(|s| s.font_weight("bold")).to_element(),
                            Text::new(&item.subtitle).style(|s| s.color("#666")).to_element(),
                        ])
                        .to_element(),
                    Button::new().text("详情").as_text().to_element(),
                ])
                .to_element()
        )
        .to_element()
}
```

### 列表容器

```rust
#[component]
fn ListContainer<T: Clone + PartialEq>(
    items: Vec<T>,
    render_item: impl Fn(T) -> Element + 'static
) -> Element {
    View::new()
        .style(|s| s.display("flex").flex_direction("column").gap("8px"))
        .childrens2(
            items.into_iter()
                .map(render_item)
                .collect()
        )
        .to_element()
}

// 使用
ListContainer {
    items: vec![
        ListItemData {
            id: 1,
            title: "项目1".to_string(),
            subtitle: "描述1".to_string()
        }
    ],
    render_item: |item| ListItem { item }.to_element()
}
```

## 下一章

掌握了组件组合模式后，继续学习：

→ [最佳实践与规范](./05-best-practices.md) - 学习编码规范和最佳实践
