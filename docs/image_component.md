# Image 组件使用指南

## 简介

Image 组件提供了一个可自定义的图片显示组件，支持加载状态、替代文本、尺寸等配置。

## 基本用法

```rust
use dioxus_blocks_components::{Image, ToElement};

let image = Image::new("https://example.com/image.jpg")
    .alt("示例图片")
    .width(300)
    .height(200)
    .to_element();
```

## 响应式图片

```rust
use dioxus_blocks_components::{Image, ObjectFit, ToElement};

let image = Image::new("https://example.com/image.jpg")
    .width_str("100%")
    .height_str("auto")
    .object_fit(ObjectFit::Cover)
    .rounded(true)
    .to_element();
```

## API 文档

### 构造方法

#### `new(src)`
创建一个新的图片实例。

**参数:**
- `src: String` - 图片的 URL 地址

**返回值:**
- `Image` - 新的图片实例

### 尺寸设置

#### `width(width: usize)`
设置图片的宽度（像素值）。

**参数:**
- `width: usize` - 宽度值（像素）

#### `width_str(width: String)`
设置图片的宽度（字符串值）。

**参数:**
- `width: String` - 宽度值，可以是 "100%", "auto", "50px" 等

#### `height(height: usize)`
设置图片的高度（像素值）。

**参数:**
- `height: usize` - 高度值（像素）

#### `height_str(height: String)`
设置图片的高度（字符串值）。

**参数:**
- `height: String` - 高度值，可以是 "100%", "auto", "50px" 等

### 样式设置

#### `alt(alt: String)`
设置图片的替代文本。

**参数:**
- `alt: String` - 替代文本，用于图片加载失败时显示

#### `object_fit(object_fit: ObjectFit)`
设置图片的对象适应方式。

**参数:**
- `object_fit: ObjectFit` - 对象适应方式枚举值

**ObjectFit 枚举:**
- `Cover` - 保持纵横比，裁剪以填充容器
- `Contain` - 保持纵横比，完整显示在容器内
- `Fill` - 拉伸以填满容器
- `None` - 保持原始尺寸（默认）
- `ScaleDown` - 缩放至最小尺寸

#### `rounded(rounded: bool)`
设置图片是否圆角。

**参数:**
- `rounded: bool` - 是否圆角，true 为圆角，false 为不圆角

#### `shadow(shadow: bool)`
设置图片是否阴影。

**参数:**
- `shadow: bool` - 是否阴影，true 为阴影，false 为无阴影

## 使用示例

### 示例 1: 基本图片

```rust
Image::new("https://via.placeholder.com/300")
    .alt("示例图片")
    .width(300)
    .height(200)
    .to_element()
```

### 示例 2: 响应式图片

```rust
Image::new("https://via.placeholder.com/600x400")
    .alt("响应式图片")
    .width_str("100%")
    .height_str("auto")
    .object_fit(ObjectFit::Cover)
    .rounded(true)
    .to_element()
```

### 示例 3: 圆角和阴影

```rust
Image::new("https://via.placeholder.com/200")
    .alt("圆角阴影图片")
    .width(200)
    .height(200)
    .rounded(true)
    .shadow(true)
    .object_fit(ObjectFit::Cover)
    .to_element()
```

### 示例 4: 不同 ObjectFit 模式

```rust
// Cover 模式
Image::new("https://via.placeholder.com/150")
    .width(150)
    .height(100)
    .object_fit(ObjectFit::Cover)

// Contain 模式
Image::new("https://via.placeholder.com/150")
    .width(150)
    .height(100)
    .object_fit(ObjectFit::Contain)

// Fill 模式
Image::new("https://via.placeholder.com/150")
    .width(150)
    .height(100)
    .object_fit(ObjectFit::Fill)
```

## 注意事项

1. `alt` 属性应该始终提供，以提高可访问性
2. 使用 `width_str` 和 `height_str` 可以设置 CSS 单位（如 "100%", "auto", "em" 等）
3. `object_fit` 属性对于响应式布局特别有用
4. 组件已实现 `Send` 和 `Sync` trait，可以安全地在线程间共享

## 完整示例

```rust
use dioxus::prelude::*;
use dioxus_blocks_components::{Image, ObjectFit, Text, ToElement, View};

#[component]
fn ImageGallery() -> Element {
    rsx! {
        View::new()
            .children(Text::h1("图片画廊"))
            .children(
                Image::new("https://example.com/photo1.jpg")
                    .alt("风景照片 1")
                    .width_str("100%")
                    .height(300)
                    .object_fit(ObjectFit::Cover)
                    .rounded(true)
            )
            .children(
                Image::new("https://example.com/photo2.jpg")
                    .alt("风景照片 2")
                    .width_str("100%")
                    .height(300)
                    .object_fit(ObjectFit::Cover)
                    .rounded(true)
            )
            .to_element()
    }
}
```
