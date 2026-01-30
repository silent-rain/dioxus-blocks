# 最佳实践与规范

本章介绍使用 `dioxus-blocks-components` 的最佳实践和编码规范。

## 命名规范

### 组件命名

- 使用 **PascalCase** 命名组件
- 名称应具有描述性
- 避免缩写

```rust
// ✅ 好的命名
struct UserProfile {}
struct ProductCard {}
struct NavigationBar {}

// ❌ 不好的命名
struct UserProf {}
struct ProdCard {}
struct Nav {}
```

### 方法命名

- 使用 **snake_case** 命名方法
- 动词 + 名词的形式
- 避免缩写

```rust
// ✅ 好的命名
pub fn get_user() {}
pub fn submit_form() {}
pub fn is_valid() {}

// ❌ 不好的命名
pub fn GetUser() {}
pub fn submitFrm() {}
pub fn valid() {}
```

### 布尔参数命名

- 使用 `is_` 或 `has_` 前缀
- 明确表示布尔语义

```rust
// ✅ 好的命名
struct MyComponent {
    is_enabled: bool,
    has_error: bool,
    is_loading: bool,
}

// ❌ 不好的命名
struct MyComponent {
    enabled: bool,
    error: bool,
    loading: bool,
}
```

### 变量命名

```rust
// ✅ 好的命名
let user_name = "Alice";
let is_logged_in = true;
let email_address = "alice@example.com";

// ❌ 不好的命名
let n = "Alice";
let log = true;
let ea = "alice@example.com";
```

## 样式组织

### 样式复用

将常用的样式提取为闭包或常量，避免重复。

```rust
// ❌ 不好的做法：重复样式
Card::new()
    .style(|s| s.padding("24px").border_radius("12px").background_color("#FAFAFA"))
    .to_element()

Card::new()
    .style(|s| s.padding("24px").border_radius("12px").background_color("#FAFAFA"))
    .to_element()

// ✅ 好的做法：样式复用
let card_style = |s: Style| {
    s.padding("24px")
        .border_radius("12px")
        .background_color("#FAFAFA")
};

Card::new().style(card_style).to_element()
Card::new().style(card_style).to_element()
```

### 样式模块化

```rust
// styles.rs
pub const CARD_PADDING: &str = "24px";
pub const CARD_BORDER_RADIUS: &str = "12px";
pub const CARD_BACKGROUND: &str = "#FAFAFA";

pub fn card_style() -> impl Fn(Style) -> Style {
    move |s: Style| {
        s.padding(CARD_PADDING)
            .border_radius(CARD_BORDER_RADIUS)
            .background_color(CARD_BACKGROUND)
    }
}

// 使用
Card::new().style(card_style()).to_element()
```

### 主题化管理

```rust
// theme.rs
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub text: String,
}

pub const THEME_LIGHT: Theme = Theme {
    primary: "#1976d2".to_string(),
    secondary: "#dc004e".to_string(),
    background: "#ffffff".to_string(),
    text: "#333333".to_string(),
};

pub const THEME_DARK: Theme = Theme {
    primary: "#90caf9".to_string(),
    secondary: "#f48fb1".to_string(),
    background: "#121212".to_string(),
    text: "#ffffff".to_string(),
};
```

### BEM 命名

```rust
// ✅ 好的做法：使用 BEM 命名
Card::new()
    .class("card")
    .style(|s| s.custom(".card__header {} .card__body {}"))
    .to_element()

// ❌ 不好的做法：无规律的类名
Card::new()
    .class("white-box")
    .to_element()
```

## 组件设计原则

### 单一职责原则

每个组件只负责一个功能。

```rust
// ✅ 好的做法：职责分离
#[component]
fn UserAvatar(user: User) -> Element {
    Image::new(user.avatar_url).to_element()
}

#[component]
fn UserInfo(user: User) -> Element {
    View::new()
        .childrens2(vec![
            Text::new(&user.name).to_element(),
            Text::new(&user.email).to_element(),
        ])
        .to_element()
}

#[component]
fn UserProfile(user: User) -> Element {
    Card::new()
        .children([
            UserAvatar { user }.to_element(),
            UserInfo { user }.to_element(),
        ])
        .to_element()
}

// ❌ 不好的做法：职责混乱
#[component]
fn UserProfileComplex(user: User) -> Element {
    Card::new()
        .body(rsx! {
            Image { src: user.avatar_url }
            Text { text: user.name }
            Text { text: user.email }
            // 其他不相关的功能...
        })
        .to_element()
}
```

### 可复用性

通过 Props 参数提高组件可复用性。

```rust
// ✅ 好的做法：参数化
#[component]
fn Button(
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
) -> Element {
    dioxus_blocks_components::Button::new()
        .text(text)
        .btn_type(variant)
        .size(size)
        .to_element()
}

// 使用
Button { text: "提交".to_string(), variant: ButtonVariant::Primary, size: ButtonSize::Medium }

// ❌ 不好的做法：硬编码
#[component]
fn SubmitButton() -> Element {
    dioxus_blocks_components::Button::new()
        .text("提交")
        .btn_type(ButtonType::Primary)
        .size(ButtonSize::Medium)
        .to_element()
}
```

### 可组合性

设计组件使其易于组合使用。

```rust
// ✅ 好的做法：组合友好
#[component]
fn Card(
    header: Option<Element>,
    body: Element,
    footer: Option<Element>,
) -> Element {
    dioxus_blocks_components::Card::new()
        .header_option(header)
        .body(body)
        .footer_option(footer)
        .to_element()
}

// 使用
Card {
    header: Some(Text::h3("标题").to_element()),
    body: Text::p("内容").to_element(),
    footer: None,
}
```

## 事件处理最佳实践

### 事件冒泡控制

```rust
#[component]
fn NestedButtons() -> Element {
    View::new()
        .onclick(|e| {
            e.stop_propagation();  // 阻止事件冒泡
            println!("Outer clicked");
        })
        .children([
            Button::new()
                .text("Inner")
                .onclick(|e| {
                    e.stop_propagation();
                    println!("Inner clicked");
                })
                .to_element(),
        ])
        .to_element()
}
```

### 事件性能优化

```rust
// ✅ 好的做法：避免频繁创建闭包
#[component]
fn ButtonList() -> Element {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    let onclick = |item: &str| {
        println!("Clicked: {}", item);
    };
    
    View::new()
        .childrens2(
            items.iter()
                .map(|item| 
                    Button::new()
                        .text(item)
                        .onclick(move |_| onclick(item))
                        .to_element()
                )
                .collect()
        )
        .to_element()
}

// ❌ 不好的做法：每次渲染创建新闭包
Button::new()
    .text(item)
    .onclick(|_| println!("Clicked: {}", item))  // item 可能被错误捕获
    .to_element()
```

### 事件处理模式

```rust
// 模式1：传递事件处理器
#[component]
fn FormField(
    label: String,
    on_change: EventHandler<String>,
) -> Element {
    View::new()
        .childrens2(vec![
            Text::new(&label).to_element(),
            Input::new()
                .oninput(move |e| on_change.call(e.value()))
                .to_element(),
        ])
        .to_element()
}

// 使用
FormField {
    label: "用户名".to_string(),
    on_change: move |value| {
        println!("Input: {}", value);
    }
}
```

## 类型安全

### 使用枚举而非字符串

```rust
// ✅ 好的做法：类型安全
Button::new()
    .btn_type(ButtonType::Primary)
    .shape(ButtonShape::Round)
    .to_element()

// ❌ 不好的做法：字符串
Button::new()
    .type_("primary")  // 容易拼写错误
    .shape("round")     // IDE 无法补全
```

### 避免使用 any

```rust
// ✅ 好的做法：明确类型
#[component]
fn DisplayItem(item: Displayable) -> Element {
    match item {
        Displayable::Text(text) => Text::new(text).to_element(),
        Displayable::Number(num) => Text::new(num.to_string()).to_element(),
    }
}

#[derive(Clone, PartialEq)]
enum Displayable {
    Text(String),
    Number(i32),
}

// ❌ 不好的做法：使用 any
#[component]
fn DisplayItem(item: Box<dyn Any>) -> Element {
    // 类型不安全，容易出错
}
```

### 泛型使用

```rust
// ✅ 好的做法：泛型提高复用性
#[component]
fn List<T: Clone + PartialEq + 'static>(
    items: Vec<T>,
    render: impl Fn(T) -> Element + 'static,
) -> Element {
    View::new()
        .style(|s| s.display("flex").flex_direction("column").gap("8px"))
        .childrens2(items.into_iter().map(render).collect())
        .to_element()
}

// 使用
List {
    items: vec!["Alice", "Bob"],
    render: |name| Text::new(name).to_element()
}
```

## 代码复用策略

### 自定义 Hook

```rust
// hooks.rs
pub fn use_local_storage<T: Clone + serde::Serialize + serde::de::DeserializeOwned>(
    key: String,
    initial: T,
) -> Signal<T> {
    let mut signal = use_signal(|| {
        if let Ok(Some(value)) = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .get_item(&key)
        {
            serde_json::from_str(&value).unwrap_or(initial)
        } else {
            initial
        }
    });
    
    use_effect(move || {
        if let Ok(storage) = web_sys::window().unwrap().local_storage() {
            storage.set_item(&key, &serde_json::to_string(&signal()).unwrap());
        }
    });
    
    signal
}

// 使用
let username = use_local_storage("username".to_string(), String::new());
```

### 工具函数

```rust
// utils.rs
pub fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn truncate(text: &str, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[..max_length])
    } else {
        text.to_string()
    }
}
```

### 高阶组件

```rust
// 高阶组件：添加加载状态
#[component]
fn WithLoading(
    is_loading: bool,
    children: Element,
) -> Element {
    Card::new()
        .body(
            if is_loading {
                Text::new("加载中...").to_element()
            } else {
                children
            }
        )
        .to_element()
}

// 使用
WithLoading {
    is_loading: data().is_none(),
    children: render_data(),
}
```

## 组件文档

### 完整文档示例

```rust
/// 用户卡片组件
///
/// 显示用户的基本信息和操作按钮。
///
/// # 示例
///
/// ```rust
/// use dioxus_blocks_components::ToElement;
/// UserCard {
///     user: User { name: "Alice".to_string() },
///     on_edit: |_| {},
///     on_delete: |_| {},
/// }.to_element()
/// ```
#[derive(Debug, Clone)]
pub struct UserCard {
    /// 用户数据
    pub user: User,
    /// 编辑回调
    pub on_edit: EventHandler<MouseEvent>,
    /// 删除回调
    pub on_delete: EventHandler<MouseEvent>,
}
```

### 参数文档

```rust
/// 表单组件
///
/// # 参数
///
/// * `title` - 表单标题
/// * `on_submit` - 提交回调，接收表单数据
/// * `is_loading` - 是否显示加载状态
///
/// # 示例
///
/// ```rust
/// Form {
///     title: "用户注册".to_string(),
///     on_submit: |data| println!("{:?}", data),
///     is_loading: false,
/// }.to_element()
/// ```
```

## 性能优化

### 避免不必要的重渲染

```rust
// ✅ 好的做法：使用 use_memo
#[component]
fn ExpensiveList(items: Vec<Item>) -> Element {
    let expensive_result = use_memo(move || {
        items.iter().map(|item| item.compute_expensive()).collect()
    });
    
    render_items(expensive_result())
}

// ❌ 不好的做法：每次渲染都计算
#[component]
fn ExpensiveList(items: Vec<Item>) -> Element {
    let result = items.iter().map(|item| item.compute_expensive()).collect();
    render_items(result)
}
```

### 组件懒加载

```rust
// ✅ 好的做法：条件渲染
#[component]
fn LazyComponent(show_detail: bool) -> Element {
    View::new()
        .children([
            Button::new()
                .text("显示详情")
                .onclick(move |_| *show_detail.write() = true)
                .to_element(),
            // 仅在需要时渲染
            if *show_detail() {
                Some(DetailComponent {}.to_element())
            } else {
                None
            },
        ])
        .to_element()
}
```

### Key 优化

```rust
// ✅ 好的做法：提供 key
Grid::new(
    items.into_iter()
        .map(|item| 
            GridItem::new(Card::new().body(Text::new(&item.name)))
                .key(item.id)  // 使用唯一标识作为 key
        )
        .collect()
)
.cols(GridCols::Col3)
.to_element()
```

## 下一章

掌握了最佳实践后，继续学习：

→ [问题排查指南](./06-troubleshooting.md) - 解决常见问题
