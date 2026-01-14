# API 改进说明

## 问题背景

原来的API设计在使用时存在以下问题：

1. **繁琐的元素包装**：`header()` 方法只能接受单个元素，多个元素需要用 `Wrap` 手动包装
2. **不直观的样式设置**：样式需要手动拼接字符串
3. **缺乏灵活性**：无法在构建时访问组件当前状态

## 改进方案

### 1. 闭包构建器方法

为组件添加了新的闭包构建器方法：

#### Card 组件
- `header_with(|card| rsx! { ... })` - 使用闭包设置头部
- `body_with(|card| rsx! { ... })` - 使用闭包设置主体
- `children_with(|card| vec![...])` - 使用闭包设置多个子元素
- `footer_with(|card| rsx! { ... })` - 使用闭包设置底部

#### Text 组件
- `style(|s| s.width("100px").height("100px"))` - 使用闭包设置样式

### 2. 样式构建器改进

`Style` 组件现在支持链式调用和闭包构建：

```rust
// 原来的方式
Card::new().style("width: 100%; padding: 20px; margin: 10px;")

// 改进后的方式
Card::new().style(|s| s
    .width("100%")
    .padding("20px")
    .margin("10px")
)
```

## 使用示例对比

### 原来的API

```rust
Card::new()
    .header(Wrap::new().childrens(vec![
        Text::h2("标签用法"),
        Text::p("由tag属性来选择文本标签类型，支持H1-H6, P, Span等标签。"),
    ]))
    .childrens(vec![
        Text::h1("这是 H1 标题"),
        Text::h2("这是 H2 标题"),
        Text::h3("这是 H3 标题"),
        // ...
    ])
    .style("width: 100%; padding: 20px;")
    .into()
```

### 改进后的API

```rust
Card::new()
    .header_with(|_| rsx! {
        Text::h2("标签用法"),
        Text::p("由tag属性来选择文本标签类型，支持H1-H6, P, Span等标签。"),
    })
    .children_with(|_| vec![
        Text::h1("这是 H1 标题").into(),
        Text::h2("这是 H2 标题").into(),
        Text::h3("这是 H3 标题").into(),
        // ...
    ])
    .style(|s| s
        .width("100%")
        .padding("20px")
    )
    .into()
```

## 改进优势

1. **更简洁**：不需要手动包装多个元素
2. **更直观**：闭包参数更符合Rust习惯
3. **类型安全**：样式方法提供类型检查
4. **可扩展性**：闭包可以访问组件当前状态
5. **链式调用**：支持流畅的API调用

## 向后兼容性

所有原来的API方法都保持不变，新添加的方法是可选的，确保现有代码不会破坏。

## 未来计划

1. 为更多组件添加类似的闭包构建器方法
2. 扩展样式构建器的功能
3. 添加更多便捷的构建器方法