//! 布局相关样式
//!
//! 提供 display、flex、position 等布局相关的样式方法。

use super::Style;

impl Style {
    /// 宽度
    ///
    /// # 参数
    /// * `width` - 宽度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().width("100px");
    /// ```
    ///
    pub fn width<T: Into<String>>(self, width: T) -> Self {
        self.insert_style("width", width.into())
    }

    /// 高度
    ///
    /// # 参数
    /// * `height` - 高度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().height("100px");
    /// ```
    ///
    pub fn height<T: Into<String>>(self, height: T) -> Self {
        self.insert_style("height", height.into())
    }

    /// 显示方式
    ///
    /// # 参数
    /// * `display` - 显示方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().display("flex");
    /// ```
    ///
    pub fn display<T: Into<String>>(self, display: T) -> Self {
        self.insert_style("display", display.into())
    }

    /// flex 方向
    ///
    /// # 参数
    /// * `direction` - flex 方向值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().flex_direction("row");
    /// ```
    ///
    pub fn flex_direction<T: Into<String>>(self, direction: T) -> Self {
        self.insert_style("flex-direction", direction.into())
    }

    /// flex 属性
    ///
    /// # 参数
    /// * `value` - flex 值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().flex("1");
    /// Style::default().flex("0 1 auto");
    /// ```
    ///
    pub fn flex<T: Into<String>>(self, value: T) -> Self {
        self.insert_style("flex", value.into())
    }

    /// flex 换行
    ///
    /// # 参数
    /// * `wrap` - flex 换行值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().flex_wrap("wrap");
    /// Style::default().flex_wrap("nowrap");
    /// ```
    ///
    pub fn flex_wrap<T: Into<String>>(self, wrap: T) -> Self {
        self.insert_style("flex-wrap", wrap.into())
    }

    /// 对齐方式
    ///
    /// # 参数
    /// * `align` - 对齐方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().align_items("center");
    /// ```
    ///
    pub fn align_items<T: Into<String>>(self, align: T) -> Self {
        self.insert_style("align-items", align.into())
    }

    /// 内容对齐方式
    ///
    /// # 参数
    /// * `justify` - 对齐方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().justify_content("center");
    /// ```
    ///
    pub fn justify_content<T: Into<String>>(self, justify: T) -> Self {
        self.insert_style("justify-content", justify.into())
    }

    /// 间距
    ///
    /// # 参数
    /// * `gap` - 间距值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().gap("16px");
    /// ```
    ///
    pub fn gap<T: Into<String>>(self, gap: T) -> Self {
        self.insert_style("gap", gap.into())
    }

    /// 定位方式
    ///
    /// # 参数
    /// * `position` - 定位方式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().position("relative");
    /// ```
    ///
    pub fn position<T: Into<String>>(self, position: T) -> Self {
        self.insert_style("position", position.into())
    }

    /// Z轴层级
    ///
    /// # 参数
    /// * `index` - Z轴层级值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().z_index("10");
    /// ```
    ///
    pub fn z_index<T: Into<String>>(self, index: T) -> Self {
        self.insert_style("z-index", index.into())
    }

    /// 溢出处理
    ///
    /// # 参数
    /// * `overflow` - 溢出处理值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().overflow("hidden");
    /// ```
    ///
    pub fn overflow<T: Into<String>>(self, overflow: T) -> Self {
        self.insert_style("overflow", overflow.into())
    }

    /// 变换
    ///
    /// # 参数
    /// * `transform` - 变换值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().transform("scale(0.98)");
    /// ```
    ///
    pub fn transform<T: Into<String>>(self, transform: T) -> Self {
        self.insert_style("transform", transform.into())
    }
}
