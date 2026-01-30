# 测试策略

本章介绍 Dioxus 组件的测试方法和策略。

## 单元测试

### 组件属性测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_blocks_components::{Button, ButtonType};

    #[test]
    fn test_button_creation() {
        let button = Button::new()
            .text("Test")
            .as_primary();
        
        // 测试属性
        assert_eq!(button.text, "Test");
        assert_eq!(button.btn_type, ButtonType::Primary);
    }

    #[test]
    fn test_button_default_values() {
        let button = Button::new();
        
        assert_eq!(button.text, "Button");
        assert_eq!(button.btn_type, ButtonType::Default);
    }
}
```

### 组件渲染测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use dioxus_core::Mutations;

    #[test]
    fn test_button_rendering() {
        let mut dom = VirtualDom::new(|| {
            Button::new()
                .id("test-button")
                .text("Test Button")
                .to_element()
        });
        
        let mut mutations = Mutations::default();
        dom.rebuild(&mut mutations);
        
        // 验证渲染结果
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("button"));
        assert!(html.contains("Test Button"));
        assert!(html.contains("t-button"));
    }

    #[test]
    fn test_card_rendering() {
        let mut dom = VirtualDom::new(|| {
            Card::new()
                .header(Text::h3("Header"))
                .body(Text::p("Body"))
                .to_element()
        });
        
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("t-card"));
        assert!(html.contains("Header"));
        assert!(html.contains("Body"));
    }
}
```

### 事件处理测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use std::{any::Any, rc::Rc};

    #[test]
    fn test_button_click() {
        let mut clicked = false;
        let mut dom = VirtualDom::new(|| {
            Button::new()
                .id("test-button")
                .text("Click me")
                .onclick(move |_| {
                    clicked = true;
                })
                .to_element()
        });
        
        // 初始化虚拟 DOM
        dom.rebuild(&mut dioxus_core::NoOpMutations);
        
        // 模拟点击事件
        let payload = Rc::new(dioxus_html::SerializedMouseData::default());
        let event = dioxus::core::Event::new(
            payload as Rc<dyn Any>,
            true
        );
        
        dom.runtime().handle_event("click", event, dioxus::core::ElementId(1));
        
        // 验证事件被触发
        assert!(clicked);
    }
}
```

### 快照测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_snapshot() {
        let mut dom = VirtualDom::new(|| {
            Button::new().text("Snapshot").as_primary().to_element()
        });
        
        let html = dioxus_ssr::render(&dom);
        
        // 简单的快照测试
        assert_eq!(
            html.trim(),
            r#"<button class="t-button t-button--primary" id="">Snapshot</button>"#
        );
    }
}
```

## 集成测试

### 多组件组合测试

```rust
#[test]
    fn test_form_integration() {
        let mut dom = VirtualDom::new(|| {
            Card::new()
                .header(Text::h3("用户注册"))
                .body(
                    View::new()
                        .style(|s| s.display("flex").flex_direction("column").gap("16px"))
                        .childrens2(vec![
                            Input::new()
                                .placeholder("用户名")
                                .to_element(),
                            Input::new()
                                .input_type(InputType::Password)
                                .placeholder("密码")
                                .to_element(),
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
        });
        
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("t-card"));
        assert!(html.contains("t-button"));
        assert!(html.contains("用户注册"));
    }
}
```

### 路由测试

```rust
#[test]
fn test_navigation() {
    let mut dom = VirtualDom::new(|| {
        Router::<Route> {}
    });
    
    // 测试初始路由
    assert_eq!(dom.root_layout(), Some(Route::Home {}));
    
    // 测试路由跳转
    let navigator = dom.reinterpret(&Route::About {});
    assert_eq!(navigator.current(), Route::About {});
}
```

### 状态管理测试

```rust
#[test]
fn test_counter_component() {
    let mut dom = VirtualDom::new(|| {
        Counter {}
    });
    
    dom.rebuild(&mut Mutations::default());
    
    // 初始状态
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Count: 0"));
    
    // 模拟点击
    let button_id = ElementId(1);
    let payload = Rc::new(SerializedMouseData::default());
    let event = Event::new(payload as Rc<dyn Any>, true);
    dom.runtime().handle_event("click", event, button_id);
    
    // 更新后的状态
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Count: 1"));
}
```

## 端到端测试

### 使用 Playwright

```rust
// e2e_test.rs
use dioxus::prelude::*;

#[tokio::test]
async fn test_user_flow() {
    let app = dioxus_cli_config::app::builder()
        .app(App {})
        .build();
    
    let mut browser = playwright::Browser::launch(playwright::LaunchOptions {
        headless: true,
        ..Default::default()
    }).await.unwrap();
    
    let page = browser.new_page().await.unwrap();
    page.goto("http://localhost:8080").await.unwrap();
    
    // 测试用户登录流程
    page.click("#username-input").await.unwrap();
    page.fill("#username-input", "testuser").await.unwrap();
    page.click("#password-input").await.unwrap();
    page.fill("#password-input", "password123").await.unwrap();
    page.click("#login-button").await.unwrap();
    
    // 验证登录成功
    let welcome_text = page.text_content(".welcome-message").await.unwrap();
    assert!(welcome_text.contains("欢迎"));
}
```

## 测试工具和 Mock

### API Mock

```rust
// 测试中使用 mock 数据
#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_user_api() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        ]
    }

    #[test]
    fn test_user_list() {
        let users = mock_user_api();
        
        let mut dom = VirtualDom::new(|| {
            UserList { users: users.clone() }
        });
        
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Alice"));
        assert!(html.contains("Bob"));
    }
}
```

### 时间 Mock

```rust
// 测试异步操作
#[test]
fn test_async_data() {
    let mut dom = VirtualDom::new(|| {
        AsyncComponent {}
    });
    
    // 模拟异步数据加载
    let mut mutations = Mutations::default();
    dom.rebuild(&mut mutations);
    
    // 等待资源加载
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        });
    
    // 验证数据已加载
    let html = dioxus_ssr::render(&dom);
    assert!(!html.contains("Loading..."));
}
```

## 测试覆盖率

### 设置覆盖率目标

```toml
# Cargo.toml
[dev-dependencies]
cargo-tarpaulin = "0.27"
```

```bash
# 运行覆盖率测试
cargo tarpaulin --out Html --output-dir ./coverage
```

### 关键路径覆盖

```rust
#[test]
fn test_user_login_flow() {
    // 测试登录成功路径
    test_login_success();
    
    // 测试登录失败路径
    test_login_failure();
    
    // 测试表单验证路径
    test_form_validation();
}
```

## 测试最佳实践

### 1. 测试命名

```rust
// ✅ 好的命名
#[test]
fn test_button_renders_correctly() {}

#[test]
fn test_button_click_triggers_handler() {}

// ❌ 不好的命名
#[test]
fn test1() {}
#[test]
fn test_button() {}
```

### 2. 测试隔离

```rust
#[test]
fn test_component_independent() {
    // 每个测试独立，不依赖其他测试
    let component = MyComponent::new();
    assert_eq!(component.value, "test");
}
```

### 3. 使用辅助函数

```rust
fn render_component<C: ToElement>(component: C) -> String {
    let mut dom = VirtualDom::new(|| component.to_element());
    dioxus_ssr::render(&dom)
}

#[test]
fn test_helper() {
    let html = render_component(
        Button::new().text("Test")
    );
    assert!(html.contains("Test"));
}
```

### 4. 测试边界情况

```rust
#[test]
fn test_empty_list() {
    let items = Vec::new();
    let component = List { items };
    assert!(render_component(component).contains("暂无数据"));
}

#[test]
fn test_long_text() {
    let long_text = "A".repeat(1000);
    let component = Text::new(&long_text);
    let html = render_component(component);
    assert!(html.contains("..."));  // 应该被截断
}
```

## 持续集成

### GitHub Actions 配置

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all
      - name: Run linter
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt -- --check
```

## 下一章

掌握了测试策略后，继续学习：

→ [项目开发指南](./08-project-guide.md) - 学习项目结构和开发流程
