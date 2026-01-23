//! 视觉效果相关样式
//!
//! 提供透明度、阴影、过渡等视觉效果相关的样式方法。

use super::Style;

impl Style {
    /// 透明度
    ///
    /// # 参数
    /// * `opacity` - 透明度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().opacity("0.5");
    /// ```
    ///
    pub fn opacity<T: Into<String>>(self, opacity: T) -> Self {
        self.insert_style("opacity", opacity.into())
    }

    /// 阴影
    ///
    /// # 参数
    /// * `shadow` - 阴影值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().box_shadow("0 2px 4px rgba(0,0,0,0.1)");
    /// ```
    ///
    pub fn box_shadow<T: Into<String>>(self, shadow: T) -> Self {
        self.insert_style("box-shadow", shadow.into())
    }

    /// 过渡
    ///
    /// # 参数
    /// * `transition` - 过渡值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().transition("all 0.3s ease");
    /// ```
    ///
    pub fn transition<T: Into<String>>(self, transition: T) -> Self {
        self.insert_style("transition", transition.into())
    }

    /// 光标样式
    ///
    /// # 参数
    /// * `cursor` - 光标样式值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().cursor("pointer");
    /// ```
    ///
    pub fn cursor<T: Into<String>>(self, cursor: T) -> Self {
        self.insert_style("cursor", cursor.into())
    }

    /// 最大宽度
    ///
    /// # 参数
    /// * `max_width` - 最大宽度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().max_width("100%");
    /// ```
    ///
    pub fn max_width<T: Into<String>>(self, max_width: T) -> Self {
        self.insert_style("max-width", max_width.into())
    }

    /// 最小宽度
    ///
    /// # 参数
    /// * `min_width` - 最小宽度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().min_width("200px");
    /// ```
    ///
    pub fn min_width<T: Into<String>>(self, min_width: T) -> Self {
        self.insert_style("min-width", min_width.into())
    }

    /// 最大高度
    ///
    /// # 参数
    /// * `max_height` - 最大高度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().max_height("100vh");
    /// ```
    ///
    pub fn max_height<T: Into<String>>(self, max_height: T) -> Self {
        self.insert_style("max-height", max_height.into())
    }

    /// 最小高度
    ///
    /// # 参数
    /// * `min_height` - 最小高度值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().min_height("50px");
    /// ```
    ///
    pub fn min_height<T: Into<String>>(self, min_height: T) -> Self {
        self.insert_style("min-height", min_height.into())
    }

    /// 指针事件
    ///
    /// # 参数
    /// * `pointer_events` - 指针事件值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().pointer_events("none");
    /// ```
    ///
    pub fn pointer_events<T: Into<String>>(self, pointer_events: T) -> Self {
        self.insert_style("pointer-events", pointer_events.into())
    }

    /// 用户选择
    ///
    /// # 参数
    /// * `user_select` - 用户选择值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().user_select("none");
    /// ```
    ///
    pub fn user_select<T: Into<String>>(self, user_select: T) -> Self {
        self.insert_style("user-select", user_select.into())
    }

    /// 可见性
    ///
    /// # 参数
    /// * `visibility` - 可见性值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().visibility("hidden");
    /// ```
    ///
    pub fn visibility<T: Into<String>>(self, visibility: T) -> Self {
        self.insert_style("visibility", visibility.into())
    }

    /// 白空格
    ///
    /// # 参数
    /// * `white_space` - 白空格值，可以是任何实现了 ``Into<String>`` 的类型
    ///
    /// # 返回值
    /// * 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```
    /// # use dioxus_blocks_components::Style;
    /// Style::default().white_space("nowrap");
    /// ```
    ///
    pub fn white_space<T: Into<String>>(self, white_space: T) -> Self {
        self.insert_style("white-space", white_space.into())
    }

    /// 设置图片的对象适应方式
    ///
    /// # 参数
    ///
    /// * `object_fit` - 对象适应方式
    ///
    /// # 返回值
    ///
    /// 返回修改后的图片实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Style;
    /// Style::default().object_fit("cover");
    /// ```
    pub fn object_fit<T: Into<String>>(self, object_fit: T) -> Self {
        self.insert_style("object-fit", object_fit.into())
    }

    /// 设置图片的对象位置
    ///
    /// # 参数
    ///
    /// * `object_position` - 对象位置
    ///
    /// # 返回值
    ///
    /// 返回修改后的样式实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Style;
    /// Style::default().object_position("center");
    /// ```
    pub fn object_position<T: Into<String>>(self, object_position: T) -> Self {
        self.insert_style("object-position", object_position.into())
    }
}
