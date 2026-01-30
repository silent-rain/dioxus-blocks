# 问题排查指南

本章提供常见问题的解决方案和调试技巧。

## 样式问题

### 样式不生效

**症状：** 设置的样式没有应用到组件上。

**原因：**

1. CSS 文件未引入
2. 样式优先级低
3. 类名拼写错误

**解决方案：**

```rust
// 1. 确保引入 CSS
document::Stylesheet {
    href: asset!("/assets/css/index.css"),
}

// 2. 检查类名拼写
Card::new()
    .class("t-card")  // ✅ 正确的类名
    // .class("t-Card")  // ❌ 错误的大小写
    .to_element()

// 3. 提高优先级（谨慎使用）
Card::new()
    .style(|s| s.custom("background-color: #fff !important;"))
    .to_element()
```

### 样式冲突

**症状：** 某些样式被其他样式覆盖。

**解决方案：**

```rust
// 方案1：使用更具体的选择器
Card::new()
    .style(|s| s.custom(
        ".my-card .card__body {
            background-color: #fff;
        }"
    ))
    .to_element()

// 方案2：使用内联样式（优先级高）
Card::new()
    .style(|s| s.background_color("#ffffff"))
    .to_element()
```

### 伪类不响应

**症状：** hover、active 等伪类样式没有效果。

**原因：** 伪类语法错误

**解决方案：**

```rust
// ✅ 正确的伪类语法
Button::new()
    .style(|s| s
        .background_color("white")
    )
    .to_element()

// ❌ 错误的伪类语法
Button::new()
    .style(|s| s
        .background_color("white")
    )
    .to_element()
```

## 事件问题

### 事件不触发

**症状：** 点击或其他事件没有响应。

**原因：**

1. 组件未调用 `to_element()`
2. 事件处理器设置错误

**解决方案：**

```rust
// ✅ 确保调用 to_element()
Button::new()
    .text("Click me")
    .onclick(|_| println!("Clicked"))
    .to_element()  // 必须！

// ❌ 忘记调用 to_element()
Button::new()
    .text("Click me")
    .onclick(|_| println!("Clicked"))
    // 缺少 .to_element()
```

### 事件冒泡问题

**症状：** 子组件的事件触发了父组件的事件。

**解决方案：**

```rust
Button::new()
    .text("Inner")
    .onclick(|e| {
        e.stop_propagation();  // 阻止事件冒泡
        println!("Inner clicked");
    })
    .to_element()
```

### 事件类型错误

**症状：** 事件处理器接收到了错误的事件类型。

**解决方案：**

```rust
// ✅ 使用正确的事件类型
Button::new()
    .onclick(|e: MouseEvent| {  // MouseEvent
        println!("Clicked at: {:?}", e.client_coordinates());
    })
    .to_element()

Input::new()
    .oninput(|e: FormData| {  // FormData
        println!("Input: {}", e.value());
    })
    .to_element()
```

## 渲染问题

### 组件不显示

**症状：** 组件渲染后看不到。

**原因：**

1. 子组件未调用 `to_element()`
2. 条件渲染逻辑错误
3. 样式导致隐藏（display: none, opacity: 0）

**解决方案：**

```rust
// 1. 检查子组件是否调用 to_element()
View::new()
    .children([
        Text::new("Visible").to_element(),  // ✅ 调用了
        Text::new("Invisible"),               // ❌ 未调用
    ])
    .to_element()

// 2. 检查条件渲染
View::new()
    .children([
        Text::new("Always visible").to_element(),
        if condition {
            Some(Text::new("Conditional").to_element())
        } else {
            None
        },
    ])
    .to_element()

// 3. 检查样式
Card::new()
    .style(|s| {
        // 避免使用 display: none
        // s.display("none")
        // s.opacity("0")
    })
    .to_element()
```

### 白屏

**症状：** 页面完全空白。

**解决方案：**

```rust
// 1. 检查 App 组件
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // ✅ 确保有 Router 或根组件
        Router::<Route> {}
    }
}

// 2. 检查路由配置
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},  // 确保至少有一个路由
}

// 3. 检查错误边界
// 在开发模式下，Dioxus 会显示错误信息
```

### 闪烁

**症状：** 组件渲染时出现闪烁。

**解决方案：**

```rust
// 使用条件渲染避免空状态
View::new()
    .children([
        data().is_some().then(|| {
            Card::new().body(render_data()).to_element()
        }),
    ])
    .to_element()
```

## 状态问题

### 状态不同步

**症状：** 状态更新后视图没有变化。

**原因：**

1. 未使用 Signal
2. 信号引用问题

**解决方案：**

```rust
// ✅ 使用 Signal
let mut count = use_signal(|| 0);

Button::new()
    .text("+1")
    .onclick(move |_| *count.write() += 1)
    .to_element()

Text::new(format!("Count: {}", count())).to_element()

// ❌ 使用普通变量
let mut count = 0;  // 不会触发重渲染

Button::new()
    .text("+1")
    .onclick(move |_| {
        count += 1;  // 修改不会更新视图
    })
    .to_element()
```

### 状态丢失

**症状：** 页面刷新后状态丢失。

**解决方案：**

```rust
// 使用 localStorage 持久化状态
let mut count = use_signal(|| {
    // 从 localStorage 读取
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .and_then(|s| s.get_item("count").ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
});

use_effect(move || {
    // 保存到 localStorage
    let value = count().to_string();
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .and_then(|s| s.set_item("count", &value).ok());
});
```

### 重复渲染

**症状：** 组件频繁重新渲染，影响性能。

**解决方案：**

```rust
// ✅ 使用 use_memo 缓存计算结果
let filtered_items = use_memo(move || {
    items().iter()
        .filter(|item| item.name.contains(&search_query()))
        .cloned()
        .collect::<Vec<_>>()
});

// ❌ 每次渲染都重新计算
let filtered_items = items().iter()
    .filter(|item| item.name.contains(&search_query()))
    .cloned()
    .collect::<Vec<_>>();
```

## 路由问题

### 路由跳转失败

**症状：** 点击链接或编程式导航无效果。

**原因：** NavigationTarget 类型不匹配

**解决方案：**

```rust
// ✅ 使用正确的 NavigationTarget
Link::new(NavigationTarget::from(Route::Home {}))
    .text("首页")
    .to_element()

// ❌ 错误的类型
Link::new(Route::Home {})  // 错误：未转换为 NavigationTarget
    .text("首页")
    .to_element()
```

### 参数丢失

**症状：** 路由参数没有正确传递。

**解决方案：**

```rust
// 定义带参数的路由
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/users/:id")]
    User { id: i32 },  // 确保参数类型匹配
}

// 使用路由参数
#[component]
fn UserPage() -> Element {
    let params = use_route::<UserParams>();
    Text::new(format!("User ID: {}", params.id)).to_element()
}

#[derive(Params, PartialEq, Clone)]
struct UserParams {
    id: i32,
}
```

### 404 错误

**症状：** 访问路由时显示 404。

**解决方案：**

```rust
// 添加 404 路由
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    
    #[route("/about")]
    About {},
    
    // 404 路由
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound() -> Element {
    Card::new()
        .body(Text::p("页面未找到").style(|s| s.color("red")))
        .to_element()
}
```

## 性能问题

### 渲染慢

**症状：** 页面响应迟缓，滚动卡顿。

**解决方案：**

```rust
// 1. 虚拟化长列表（第三方库）
// 2. 使用 lazy loading
#[component]
fn LazyList() -> Element {
    let show_all = use_signal(|| false);
    
    View::new()
        .children([
            items().iter().take(if show_all() { 1000 } else { 50 })
                .map(|item| Text::new(item).to_element())
                .collect::<Vec<_>>(),
            Button::new()
                .text(if show_all() { "显示全部" } else { "显示更多" })
                .onclick(move |_| *show_all.write() = !show_all())
                .to_element(),
        ])
        .to_element()
}

// 3. 避免不必要的组件嵌套
```

### 内存泄漏

**症状：** 长时间使用后内存占用持续增长。

**解决方案：**

```rust
// 1. 清理副作用
use_effect(move || {
    let cleanup = || {
        // 清理资源
        println!("Cleaning up");
    };
    
    cleanup
});

// 2. 使用弱引用避免循环引用
// 3. 及时释放资源
```

### 加载时间长

**症状：** 首屏加载慢。

**解决方案：**

```rust
// 1. 代码分割
// 2. 懒加载组件
use dioxus::prelude::*;
use dioxus_lazy::lazy;

let LazyComponent = lazy(|| {
    // 懒加载的组件
});

// 3. 资源预加载
// 4. 使用 CDN 加速
```

## 调试技巧

### 使用浏览器开发者工具

```rust
// 添加调试信息
Card::new()
    .style(|s| s.border("1px solid red"))  // 临时边框便于调试
    .to_element()

// 查看控制台输出
Button::new()
    .onclick(|_| {
        console::log_1(&"Button clicked".into());
    })
    .to_element()
```

### 条件渲染调试

```rust
// 显示调试信息
let show_debug = true;

if show_debug {
    Text::new(format!("Debug: count = {}", count())).to_element()
}

// 或者使用环境变量
if cfg!(debug_assertions) {
    Text::new("Debug mode").to_element()
}
```

### 组件树检查

```rust
// 在开发模式下，Dioxus 提供了调试工具
// 可以查看组件树和状态
```

## 下一章

掌握了问题排查后，继续学习：

→ [测试策略](./07-testing.md) - 学习如何编写测试
