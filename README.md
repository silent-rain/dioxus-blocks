# Dioxus Blocks

一个基于 Dioxus 的UI组件库。

## 项目结构

本项目是一个 Dioxus 组件库工作空间，包含以下成员：

```text
dioxus-blocks/
├─ README.md
├─ Cargo.toml
├─ dioxus-blocks-components/ # 组件库核心包
│  ├─ src/
│  │  ├─ lib.rs
│  │  ├─ components/
│  │  │  ├─ button.rs
│  │  │  ├─ card.rs
│  │  │  └─ mod.rs
│  │  ├─ style.rs
│  │  └─ constant.rs
│  └─ assets/
│     └─ css/
├─ dioxus-blocks-macro/ # 过程宏包
│  └─ src/
│     ├─ lib.rs
│     └─ component.rs
└─ dioxus-blocks-ui/ # 示例应用
   ├─ src/
   │  ├─ main.rs
   │  ├─ route.rs
   │  └─ views/
   └─ assets/
```

## 运行示例

进入示例应用目录并启动服务：

```bash
cd dioxus-blocks-ui
dx serve
```
