# 项目开发指南

本章介绍使用 `dioxus-blocks-components` 开发项目的完整指南。

## 推荐项目结构

```
my-dioxus-app/
├── Cargo.toml              # 项目依赖配置
├── Dioxus.toml            # Dioxus 配置
├── assets/                 # 静态资源
│   ├── css/              # 样式文件
│   │   ├── index.css     # 主样式
│   │   └── tailwind.css  # Tailwind CSS
│   ├── img/              # 图片资源
│   │   ├── logo.svg
│   │   └── favicon.ico
│   └── fonts/            # 字体文件
│       └── inter.woff2
├── src/
│   ├── main.rs            # 应用入口
│   ├── route.rs           # 路由定义
│   ├── components/        # 可复用组件
│   │   ├── mod.rs
│   │   ├── user_card.rs
│   │   ├── product_grid.rs
│   │   └── navbar.rs
│   ├── layouts/           # 布局组件
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   ├── sidebar.rs
│   │   └── footer.rs
│   ├── views/             # 页面视图
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── products.rs
│   │   ├── about.rs
│   │   └── not_found.rs
│   ├── hooks/             # 自定义 Hooks
│   │   ├── mod.rs
│   │   ├── use_local_storage.rs
│   │   └── use_debounce.rs
│   ├── utils/             # 工具函数
│   │   ├── mod.rs
│   │   ├── date.rs
│   │   └── format.rs
│   ├── types/             # 类型定义
│   │   ├── mod.rs
│   │   └── user.rs
│   └── services/          # API 服务
│       ├── mod.rs
│       └── api.rs
└── tests/                # 测试文件
    ├── components/
    └── integration/
```

## 开发流程

### 环境搭建

```bash
# 1. 创建新项目
cargo new my-dioxus-app
cd my-dioxus-app

# 2. 添加依赖
cargo add dioxus dioxus-desktop
cargo add dioxus-blocks-components --path ../dioxus-blocks-components

# 3. 创建目录结构
mkdir -p src/components src/layouts src/views src/hooks src/utils src/types src/services
mkdir -p assets/css assets/img

# 4. 配置 Dioxus
cat > Dioxus.toml << EOF
[application]
name = "My Dioxus App"
default_platform = "web"
EOF
```

### 依赖管理

```toml
# Cargo.toml
[package]
name = "my-dioxus-app"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = "0.7"
dioxus-desktop = "0.7"
dioxus-router = "0.7"
dioxus-blocks-components = { path = "../dioxus-blocks-components" }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
tokio-test = "0.4"
```

### 开发命令

```bash
# 启动开发服务器（Web）
dx serve

# 启动开发服务器（Desktop）
cargo run

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 构建生产版本
cargo build --release
```

## 构建与部署

### Web 平台

```bash
# 构建
dx build --release --platform web

# 输出位置
dist/
```

**部署到 Netlify：**

```bash
npm install -g netlify-cli
netlify deploy --prod --dir=dist
```

**部署到 Vercel：**

```bash
npm install -g vercel
vercel --prod dist
```

### Desktop 平台

```bash
# 构建
cargo build --release

# 输出位置
target/release/my-dioxus-app

# 打包为安装包
cargo install cargo-bundle
cargo bundle --release
```

**Windows:**

```bash
# 生成 MSI 安装包
cargo bundle --release --format msi
```

**macOS:**

```bash
# 生成 .app 包
cargo bundle --release --format osx
```

**Linux:**

```bash
# 生成 .deb 包
cargo bundle --release --format deb
```

### 静态资源处理

```rust
// src/main.rs
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // 引入 CSS
        document::Stylesheet {
            href: asset!("/assets/css/index.css"),
        }
        document::Stylesheet {
            href: asset!("/assets/css/tailwind.css"),
        }
        
        Router::<Route> {}
    }
}
```

## Git 工作流

### 分支策略

```
main           # 生产分支，稳定代码
  └─ develop    # 开发分支，集成最新功能
       ├─ feature/user-auth     # 功能分支
       ├─ feature/product-list  # 功能分支
       └─ bugfix/login-error    # 修复分支
```

### Commit 规范

使用 Conventional Commits 格式：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**示例：**

```bash
# 功能
git commit -m "feat(components): add user card component"

# 修复
git commit -m "fix(navbar): resolve layout overflow issue"

# 重构
git commit -m "refactor(styles): extract common styles to constants"

# 文档
git commit -m "docs(readme): add setup instructions"
```

### Pull Request 流程

1. 从 `develop` 创建功能分支
2. 完成功能开发和测试
3. 提交代码并推送到远程
4. 创建 Pull Request 到 `develop`
5. Code Review
6. 合并到 `develop`
7. 定期合并 `develop` 到 `main`

## 性能优化

### 构建优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 代码分割

```rust
// 使用条件导入
#[cfg(feature = "web")]
use dioxus::web::document;

#[cfg(feature = "desktop")]
use dioxus::desktop::document;
```

### 懒加载

```rust
// 异步加载组件
use dioxus_lazy::lazy;

const LazyComponent = lazy(|| {
    // 重型组件
});

// 按需加载
if should_load {
    LazyComponent {}.to_element()
}
```

### 图片优化

```rust
// 使用适当格式
let logo = asset!("/assets/img/logo.webp");  // WebP 比 PNG 小

// 响应式图片
<img {
    srcset: format!(
        "{} 1x, {} 2x",
        asset!("/assets/img/logo.png"),
        asset!("/assets/img/logo@2x.png"),
    )
}
```

## 监控与日志

### 错误监控

```rust
// 集成 Sentry
use sentry::integrations::panic::register_panic_handler;

fn main() {
    let _guard = sentry::init(("https://example@sentry.io/123", sentry::ClientOptions {
        release: sentry::release_name!(env!("CARGO_PKG_VERSION")),
        ..Default::default()
    }));
    
    dioxus::launch(App);
}
```

### 性能监控

```rust
// Web Performance API
use web_sys::window;

#[component]
fn PerformanceMonitor() -> Element {
    use_effect(move || {
        if let Some(window) = window() {
            let performance = window.performance().unwrap();
            
            // 记录页面加载时间
            let load_time = performance.now();
            web_sys::console::log_1(&format!("Page load time: {}ms", load_time).into());
        }
    });
    
    View::new().to_element()
}
```

### 用户行为追踪

```rust
// Google Analytics
use web_sys::window;

fn track_event(category: &str, action: &str, label: &str) {
    if let Some(window) = window() {
        let _ = window.eval(&format!(
            "gtag('event', '{{'action: {action}, category: {category}, label: {label}}}');"
        ));
    }
}

// 使用
Button::new()
    .onclick(|_| {
        track_event("button", "click", "submit");
    })
    .to_element()
```

## 配置管理

### 环境变量

```rust
// .env
DATABASE_URL=postgres://localhost/myapp
API_BASE_URL=https://api.example.com
```

```rust
// 读取环境变量
use std::env;

let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let api_base = env::var("API_BASE_URL").unwrap_or_else(|_| "https://api.example.com".to_string());
```

### 配置文件

```toml
# config.toml
[app]
name = "My App"
version = "1.0.0"

[api]
base_url = "https://api.example.com"
timeout = 30

[features]
enable_analytics = true
enable_debug = false
```

```rust
// 加载配置
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    app: AppConfig,
    api: ApiConfig,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    version: String,
}

fn load_config() -> AppConfig {
    let config_str = std::fs::read_to_string("config.toml").unwrap();
    toml::from_str(&config_str).unwrap()
}
```

## 国际化 (i18n)

```rust
// 使用 i18n 库
use i18n_embed::{fluent, FluentLanguageLoader};

#[component]
fn I18nComponent() -> Element {
    let loader = FluentLanguageLoader::new("my_app", "en");
    
    Text::new(loader.get("welcome_message")).to_element()
}
```

## 安全最佳实践

### XSS 防护

```rust
// Dioxus 默认转义输出
// 无需额外处理

// 用户输入
Text::new(user_input).to_element()  // 自动转义

// HTML 输出（谨慎使用）
dioxus::core::VNode::VText(user_input)  // 转义
// dioxus::core::VNode::Element(user_html)  // 不转义，危险！
```

### CSRF 保护

```rust
// 添加 CSRF Token
use csrf::{AesGcmCsrfProtection, CsrfProtection};

let csrf = AesGcmProtection::new_from_key(b"secret-key-32-bytes-long");
let token = csrf.generate_token(&[], 3600);

// 在表单中添加
Input::new()
    .name("csrf_token")
    .value(token.clone())
    .to_element()
```

### Content Security Policy

```rust
// 设置 CSP
use web_sys::window;

#[component]
fn App() -> Element {
    use_effect(move || {
        if let Some(window) = window() {
            let document = window.document().unwrap();
            let meta = document.create_element("meta").unwrap();
            meta.set_attribute("http-equiv", "Content-Security-Policy").unwrap();
            meta.set_attribute(
                "content",
                "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';"
            ).unwrap();
            document.head().unwrap().append_child(&meta).unwrap();
        }
    });
    
    rsx! { Router::<Route> {} }
}
```

## 文档生成

### README.md

```markdown
# My Dioxus App

使用 Dioxus Blocks Components 构建的现代化 Web 应用。

## 功能特性

- 🚀 基于 Dioxus 0.7
- 🎨 15+ 丰富 UI 组件
- 📱 响应式设计
- 🌙 支持深色模式
- 🔐 用户认证

## 快速开始

\`\`\`bash
# 克隆项目
git clone https://github.com/username/my-dioxus-app.git

# 安装依赖
cargo install

# 启动开发服务器
dx serve
\`\`\`

## 项目结构

\`\`\`
src/
├── components/    # 可复用组件
├── layouts/       # 布局组件
├── views/         # 页面视图
├── hooks/         # 自定义 Hooks
└── services/      # API 服务
\`\`\`

## 部署

\`\`\`bash
# Web 平台
dx build --release --platform web

# Desktop 平台
cargo build --release
\`\`\`

## 许可证

MIT
```

### API 文档

```bash
# 生成文档
cargo doc --open
```

## 下一章

项目开发指南完成！你现在掌握了：

✅ 项目结构规划
✅ 开发流程管理
✅ 构建与部署
✅ Git 工作流
✅ 性能优化
✅ 监控与日志

**下一步：**

- 查看完整示例：[完整示例代码](../assets/examples.rs)
- 查看代码片段：[代码片段集合](../assets/snippets.md)
- 返回主入口：[返回](../SKILL.md)
