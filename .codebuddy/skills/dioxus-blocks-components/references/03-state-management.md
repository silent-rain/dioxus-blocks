# 状态管理与路由

本章介绍 Dioxus 中的状态管理、Context API 和路由导航。

## Signal 响应式状态

Signal 是 Dioxus 0.7 中管理响应式状态的核心方式。

### 基础用法

```rust
use dioxus::prelude::*;
use dioxus_blocks_components::{Button, Text};

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    View::new()
        .style(|s| s.display("flex").gap("16px").align_items("center"))
        .children([
            Text::new(format!("Count: {}", count())).to_element(),
            Button::new()
                .text("+1")
                .onclick(move |_| *count.write() += 1)
                .to_element(),
        ])
        .to_element()
}
```

### Signal 读写

```rust
let mut count = use_signal(|| 0);

// 读取值
let value = count();           // 克隆值
let value = count.read();      // 获取引用

// 修改值
*count.write() += 1;         // 直接修改
count.with_mut(|c| *c += 1); // 使用闭包修改

// 条件修改
count.set(10);              // 设置新值
```

### 计算属性 use_memo

```rust
use dioxus::prelude::*;

#[component]
fn Calculator() -> Element {
    let mut a = use_signal(|| 0);
    let mut b = use_signal(|| 0);
    
    // 缓存计算结果
    let sum = use_memo(move || a() + b());
    
    rsx! {
        Input::new().value(a).to_element()
        Text::new(format!("+")).to_element()
        Input::new().value(b).to_element()
        Text::new(format!("= {}", sum())).to_element()
    }
}
```

### 副作用 use_effect

```rust
use dioxus::prelude::*;

#[component]
fn Timer() -> Element {
    let mut seconds = use_signal(|| 0);
    
    use_effect(move || {
        let mut s = seconds.clone();
        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                *s.write() += 1;
            }
        }
    });
    
    Text::new(format!("Elapsed: {}s", seconds())).to_element()
}
```

## Context API

Context API 用于在组件树中共享全局状态。

### 提供上下文

```rust
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    background_color: String,
    text_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
        }
    }
}

#[component]
fn App() -> Element {
    use_context_provider(|| Theme::default());
    
    rsx! {
        Router::<Route> {}
    }
}
```

### 消费上下文

```rust
#[component]
fn ThemedCard() -> Element {
    let theme: Signal<Theme> = use_context();
    
    Card::new()
        .style(move |s| {
            s.background_color(theme().background_color.clone())
                .color(theme().text_color.clone())
        })
        .to_element()
}
```

### 嵌套上下文

```rust
#[component]
fn Parent() -> Element {
    use_context_provider(|| ParentContext::default());
    
    rsx! {
        Child {}
    }
}

#[component]
fn Child() -> Element {
    // 可以访问父级上下文
    let parent: Signal<ParentContext> = use_context();
    // 也可以定义自己的子上下文
    use_context_provider(|| ChildContext::default());
    
    rsx! { GrandChild {} }
}
```

### 主题切换示例

```rust
#[derive(Clone, Debug, PartialEq)]
enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Debug, PartialEq)]
struct ThemeContext {
    mode: Signal<ThemeMode>,
}

impl ThemeContext {
    fn toggle(&self) {
        let mode = self.mode();
        self.mode.set(match mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        });
    }
    
    fn get_colors(&self) -> (&'static str, &'static str) {
        match self.mode() {
            ThemeMode::Light => ("#ffffff", "#333333"),
            ThemeMode::Dark => ("#121212", "#ffffff"),
        }
    }
}

#[component]
fn ThemeSwitcher() -> Element {
    let theme = ThemeContext {
        mode: use_signal(|| ThemeMode::Light),
    };
    
    use_context_provider(|| theme.clone());
    
    rsx! {
        Button::new()
            .text("切换主题")
            .onclick(move |_| theme.toggle())
            .to_element()
    }
}
```

## use_resource 异步数据

use_resource 用于管理异步数据加载。

### 基础用法

```rust
use dioxus::prelude::*;

async fn fetch_user(id: i32) -> Result<User, reqwest::Error> {
    reqwest::get(format!("https://api.example.com/users/{}", id))
        .await?
        .json()
        .await
}

#[component]
fn UserProfile(id: i32) -> Element {
    let data = use_resource(move || fetch_user(id));
    
    match data() {
        Some(Ok(user)) => Card::new()
            .header(Text::h3(&user.name))
            .body(Text::p(&user.email))
            .to_element(),
        Some(Err(e)) => Text::new(format!("错误: {}", e)).to_element(),
        None => Text::new("加载中...").to_element(),
    }
}
```

### 依赖更新

```rust
#[component]
fn SearchResults(query: String) -> Element {
    let results = use_resource(move || async move {
        search_api(query.clone()).await
    });
    
    // query 变化时自动重新加载
    match results() {
        Some(data) => render_results(data),
        None => Text::new("搜索中...").to_element(),
    }
}
```

### 错误处理

```rust
let data = use_resource(move || async move {
    fetch_data().await.map_err(|e| {
        eprintln!("Failed to fetch data: {}", e);
        e
    })
});

match data() {
    Some(Ok(value)) => render_data(value),
    Some(Err(_)) => Text::new("加载失败，请重试").to_element(),
    None => Text::new("加载中...").to_element(),
}
```

## 路由导航

### 路由定义

```rust
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/about")]
    About {},
    
    #[route("/users/:id")]
    User { id: i32 },
    
    #[route("/products/:id/reviews/:review_id")]
    Review { id: i32, review_id: i32 },
}
```

### Link 组件导航

```rust
use dioxus_blocks_components::Link;

// 基础导航
Link::new(NavigationTarget::from(Route::Home {}))
    .text("首页")
    .to_element()

// 带参数的导航
Link::new(NavigationTarget::from(Route::User { id: 123 }))
    .text("用户123")
    .to_element()

// 外部链接
Link::new(NavigationTarget::from("https://example.com"))
    .text("外部链接")
    .to_element()
```

### 编程式导航

```rust
#[component]
fn NavigationButton() -> Element {
    let navigator = use_navigator();
    
    Button::new()
        .text("跳转到关于页面")
        .onclick(move |_| navigator.push(Route::About {}))
        .to_element()
}
```

### 导航方法

```rust
let navigator = use_navigator();

// push - 添加新路由到历史
navigator.push(Route::Home {});

// replace - 替换当前路由
navigator.replace(Route::About {});

// go - 前进或后退
navigator.go(-1);  // 后退一页
navigator.go(1);   // 前进一页

// back - 后退
navigator.back();

// forward - 前进
navigator.forward();
```

### 路由参数

```rust
#[derive(Params, PartialEq, Clone)]
struct UserParams {
    id: i32,
}

#[component]
fn UserPage() -> Element {
    let params = use_route::<UserParams>();
    
    Card::new()
        .header(Text::h3(format!("用户 ID: {}", params.id)))
        .to_element()
}
```

### 嵌套路由

```rust
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/dashboard")]
    #[layout(DashboardLayout)]
    Dashboard {},
    
    #[route("/dashboard/settings")]
    #[layout(DashboardLayout)]
    DashboardSettings {},
}

#[component]
fn DashboardLayout() -> Element {
    rsx! {
        Sidebar {}
        Outlet::<Route> {}
    }
}
```

## 状态管理最佳实践

### 1. 状态提升

将共享状态提升到最近的公共父组件。

```rust
// ❌ 不好的做法：状态分散
#[component]
fn Parent() -> Element {
    rsx! {
        Child1 {}
        Child2 {}
    }
}

#[component]
fn Child1() -> Element {
    let mut value = use_signal(|| 0);
    // value 无法被 Child2 访问
}

// ✅ 好的做法：状态提升
#[component]
fn Parent() -> Element {
    let mut value = use_signal(|| 0);
    rsx! {
        Child1 { value }
        Child2 { value }
    }
}

#[component]
fn Child1(value: Signal<i32>) -> Element {
    // value 可以共享
}
```

### 2. 避免不必要的状态

```rust
// ❌ 不好的做法：可计算的状态
#[component]
fn FullName() -> Element {
    let first = use_signal(|| String::new());
    let last = use_signal(|| String::new());
    let mut full = use_signal(String::new);
    
    // 需要手动更新
    full.set(format!("{} {}", first(), last()));
}

// ✅ 好的做法：使用 use_memo
#[component]
fn FullName() -> Element {
    let first = use_signal(|| String::new());
    let last = use_signal(|| String::new());
    
    let full = use_memo(move || format!("{} {}", first(), last()));
    
    Text::new(full()).to_element()
}
```

### 3. 合理拆分状态

```rust
// ✅ 好的做法：相关状态分组
#[derive(Clone, Default)]
struct FormState {
    username: String,
    email: String,
    password: String,
}

#[component]
fn Form() -> Element {
    let mut form = use_signal(FormState::default);
    
    // 修改相关字段
    form.with_mut(|f| f.username = "new_value".to_string());
}
```

## 实战示例

### 示例1：购物车

```rust
#[derive(Clone, Default, PartialEq)]
struct CartItem {
    id: i32,
    name: String,
    price: f64,
    quantity: i32,
}

#[derive(Clone, Default, PartialEq)]
struct CartState {
    items: Vec<CartItem>,
}

impl CartState {
    fn add_item(&mut self, item: CartItem) {
        self.items.push(item);
    }
    
    fn remove_item(&mut self, id: i32) {
        self.items.retain(|item| item.id != id);
    }
    
    fn total(&self) -> f64 {
        self.items.iter().map(|item| item.price * item.quantity as f64).sum()
    }
}

#[component]
fn ShoppingCart() -> Element {
    let mut cart = use_signal(CartState::default);
    
    View::new()
        .children([
            render_cart_items(cart),
            render_total(cart),
        ])
        .to_element()
}

fn render_cart_items(cart: Signal<CartState>) -> Element {
    Grid::new(
        cart().items.iter().map(|item| {
            GridItem::new(
                Card::new()
                    .header(Text::h3(&item.name))
                    .body(Text::p(format!("￥{}", item.price)))
            )
        }).collect()
    )
    .cols(GridCols::Col3)
    .to_element()
}
```

### 示例2：用户认证

```rust
#[derive(Clone, Debug, PartialEq)]
enum AuthState {
    NotAuthenticated,
    Authenticating,
    Authenticated(User),
    Error(String),
}

#[component]
fn AuthProvider() -> Element {
    let mut auth_state = use_signal(|| AuthState::NotAuthenticated);
    
    use_context_provider(|| auth_state.clone());
    
    rsx! {
        if matches!(auth_state(), AuthState::Authenticated(_)) {
            Dashboard {}
        } else {
            LoginForm {}
        }
    }
}

#[component]
fn ProtectedRoute() -> Element {
    let auth: Signal<AuthState> = use_context();
    
    match auth() {
        AuthState::Authenticated(user) => {
            Text::new(format!("欢迎, {}", user.name)).to_element()
        }
        AuthState::NotAuthenticated => {
            // 跳转到登录页
            let navigator = use_navigator();
            navigator.push(Route::Login {});
            Text::new("请先登录").to_element()
        }
        _ => Text::new("加载中...").to_element(),
    }
}
```

### 示例3：实时搜索

```rust
#[component]
fn SearchBox() -> Element {
    let mut query = use_signal(String::new);
    let results = use_resource(move || {
        let q = query();
        async move {
            if q.len() >= 2 {
                search_api(q).await.unwrap_or_default()
            } else {
                Vec::new()
            }
        }
    });
    
    View::new()
        .children([
            Input::new()
                .value(query)
                .placeholder("搜索...")
                .to_element(),
            render_results(results),
        ])
        .to_element()
}
```

## 下一章

掌握了状态管理后，继续学习：

→ [组件组合模式](./04-patterns.md) - 学习如何灵活组合组件
