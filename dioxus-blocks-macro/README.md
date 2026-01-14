# Dioxus Blocks Macro

这个包提供了一些 Dioxus Blocks 过程宏，用于简化一些常见的操作。

## `Route` 派生宏

- 在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
dioxus-blocks-macro = "0.1.0"
```

- 使用 `Route` 派生宏：

```rust
use dioxus::prelude::*;

use dioxus_blocks_components::Card;
use dioxus_blocks_macro::Route;

#[derive(Debug, Default, Clone, Route)]
struct CardView {}

impl CardView {
    pub fn element(&self) -> Element {
        rsx! {
            // ... 内容
        }
    }
}
```

- 宏展开

```rust
#[component]
pub fn CardViewRoute() -> Element {
    let ele = CardView::default();
    rsx! {
        {ele.element()}
    }
}
```
