//! Grid 组件
//!
//! 提供一个可自定义的网格布局组件，支持类似 Tailwind CSS 的 grid 功能。
//! 支持自定义列数、行数、间距等属性。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Grid, GridItem, GridRows, ToElement, Text};
//!
//! #[component]
//! fn App() -> Element {
//!     Grid::new(vec![
//!            GridItem::new(Text::new("1")),
//!            GridItem::new(Text::new("2")),
//!            GridItem::new(Text::new("3")),
//!        ])
//!        .rows(GridRows::Row4)
//!     .to_element()
//! }
//! ```
use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_blocks_macro::ComponentBase;

use crate::{Style, ToElement};

/// 网格列数枚举
///
/// 定义网格的列数, 1-12列
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GridCols {
    /// 1列
    Col1,
    /// 2列
    Col2,
    /// 3列
    Col3,
    /// 4列
    #[default]
    Col4,
    /// 5列
    Col5,
    /// 6列
    Col6,
    /// 7列
    Col7,
    /// 8列
    Col8,
    /// 9列
    Col9,
    /// 10列
    Col10,
    /// 11列
    Col11,
    /// 12列
    Col12,
}

impl From<GridCols> for u16 {
    fn from(cols: GridCols) -> Self {
        match cols {
            GridCols::Col1 => 1,
            GridCols::Col2 => 2,
            GridCols::Col3 => 3,
            GridCols::Col4 => 4,
            GridCols::Col5 => 5,
            GridCols::Col6 => 6,
            GridCols::Col7 => 7,
            GridCols::Col8 => 8,
            GridCols::Col9 => 9,
            GridCols::Col10 => 10,
            GridCols::Col11 => 11,
            GridCols::Col12 => 12,
        }
    }
}

/// 网格行数枚举
///
/// 定义网格的行数, 1-12行
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GridRows {
    /// 1行
    #[default]
    Row1,
    /// 2行
    Row2,
    /// 3行
    Row3,
    /// 4行
    Row4,
    /// 5行
    Row5,
    /// 6行
    Row6,
    /// 7行
    Row7,
    /// 8行
    Row8,
    /// 9行
    Row9,
    /// 10行
    Row10,
    /// 11行
    Row11,
    /// 12行
    Row12,
}

impl From<GridRows> for u16 {
    fn from(cols: GridRows) -> Self {
        match cols {
            GridRows::Row1 => 1,
            GridRows::Row2 => 2,
            GridRows::Row3 => 3,
            GridRows::Row4 => 4,
            GridRows::Row5 => 5,
            GridRows::Row6 => 6,
            GridRows::Row7 => 7,
            GridRows::Row8 => 8,
            GridRows::Row9 => 9,
            GridRows::Row10 => 10,
            GridRows::Row11 => 11,
            GridRows::Row12 => 12,
        }
    }
}

/// 网格项组件结构体
///
/// 提供一个可自定义的网格项，支持控制其在网格中的位置和跨度。
#[derive(Debug, Clone, ComponentBase)]
pub struct GridItem {
    /// 网格项的唯一标识符
    id: Option<String>,
    /// 网格项的CSS类名
    class: String,
    /// 网格项的内联样式
    style: Option<Style>,
    /// 网格项的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 网格项的子元素列表
    onclick: Option<EventHandler<MouseEvent>>,

    /// 网格项在列方向上的跨度，默认为 1
    col_span: usize,
    /// 网格项在行方向上的跨度，默认为 1
    row_span: usize,
    /// 网格项在列方向上的起始位置，默认为 0（自动）
    col_start: usize,
    /// 网格项在列方向上的结束位置，默认为 0（自动）
    col_end: usize,
    /// 网格项在行方向上的起始位置，默认为 0（自动）
    row_start: usize,
    /// 网格项在行方向上的结束位置，默认为 0（自动）
    row_end: usize,
}

impl Default for GridItem {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_grid-item".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,

            col_span: 1,
            row_span: 1,
            col_start: 0,
            col_end: 0,
            row_start: 0,
            row_end: 0,
        }
    }
}

unsafe impl Send for GridItem {}
unsafe impl Sync for GridItem {}

impl GridItem {
    /// 创建一个新的网格项实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的网格项实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{GridItem, Text};
    /// let grid_item = GridItem::new(Text::new("Hello"));
    /// ```
    pub fn new<T>(component: T) -> Self
    where
        T: ToElement + Clone + 'static,
    {
        Self {
            childrens: vec![Rc::new(component)],
            ..Default::default()
        }
    }

    /// 设置网格项的列跨度
    ///
    /// # 参数
    ///
    /// * `col_span` - 数字值，定义网格项在列方向上的跨度
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格项实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::GridItem;
    /// GridItem::default().col_span(2);
    /// ```
    pub fn col_span(mut self, col_span: usize) -> Self {
        self.col_span = col_span;
        self
    }

    /// 设置网格项的行跨度
    ///
    /// # 参数
    ///
    /// * `row_span` - 数字值，定义网格项在行方向上的跨度
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格项实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::GridItem;
    /// GridItem::default().row_span(2);
    /// ```
    pub fn row_span(mut self, row_span: usize) -> Self {
        self.row_span = row_span;
        self
    }

    /// 设置网格项的列起始位置
    ///
    /// # 参数
    ///
    /// * `col_start` - 数字值，定义网格项在列方向上的起始位置，0表示自动
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格项实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::GridItem;
    /// GridItem::default().col_start(3);
    /// ```
    pub fn col_start(mut self, col_start: usize) -> Self {
        self.col_start = col_start;
        self
    }

    /// 设置网格项的列结束位置
    ///
    /// # 参数
    ///
    /// * `col_end` - 数字值，定义网格项在列方向上的结束位置，0表示自动
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格项实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::GridItem;
    /// GridItem::default().col_end(4);
    /// ```
    pub fn col_end(mut self, col_end: usize) -> Self {
        self.col_end = col_end;
        self
    }

    /// 设置网格项的行起始位置
    ///
    /// # 参数
    ///
    /// * `row_start` - 数字值，定义网格项在行方向上的起始位置，0表示自动
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格项实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::GridItem;
    /// GridItem::default().row_start(2);
    /// ```
    pub fn row_start(mut self, row_start: usize) -> Self {
        self.row_start = row_start;
        self
    }

    /// 设置网格项的行结束位置
    ///
    /// # 参数
    ///
    /// * `row_end` - 数字值，定义网格项在行方向上的结束位置，0表示自动
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格项实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::GridItem;
    /// GridItem::default().row_end(3);
    /// ```
    pub fn row_end(mut self, row_end: usize) -> Self {
        self.row_end = row_end;
        self
    }
}

impl ToElement for GridItem {
    /// 渲染网格项元素
    ///
    /// 将网格项组件渲染为 Dioxus 元素，可以在 UI 中显示。
    ///
    /// # 返回值
    ///
    /// 返回一个 Dioxus Element，表示渲染后的网格项
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus_blocks_components::{GridItem, ToElement};
    ///
    /// GridItem::default().col_span(2).to_element();
    /// ```
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let mut class = self.class.clone();
        let style = self
            .style
            .clone()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let childrens = self.childrens_to_element();

        // 添加自定义样式
        if self.col_span > 0 {
            class.push_str(&format!(" t_col-span-{}", self.col_span));
        }
        if self.row_span > 0 {
            class.push_str(&format!(" t_row-span-{}", self.row_span));
        }

        if self.col_start > 0 {
            class.push_str(&format!(" t_col-start-{}", self.col_start));
        }
        if self.col_end > 0 {
            class.push_str(&format!(" t_col-end-{}", self.col_end));
        }

        if self.row_start > 0 {
            class.push_str(&format!(" t_row-start-{}", self.row_start));
        }
        if self.row_end > 0 {
            class.push_str(&format!(" t_row-end-{}", self.row_end));
        }

        rsx! {
            div { id, class, style, {childrens} }
        }
    }
}

/// 网格组件结构体
///
/// 提供一个可自定义的网格布局容器，支持列数、行数、间距等配置。
#[derive(Debug, Clone, ComponentBase)]
pub struct Grid {
    /// 网格的唯一标识符
    id: Option<String>,
    /// 网格的CSS类名
    class: String,
    /// 网格的内联样式
    style: Option<Style>,
    /// 网格的子元素列表
    childrens: Vec<Rc<dyn ToElement>>,
    /// 网格的点击事件
    onclick: Option<EventHandler<MouseEvent>>,

    /// 网格的列数，默认为 GridCols::Four
    cols: Option<GridCols>,
    /// 网格的行数，默认为 None
    rows: Option<GridRows>,
    /// 网格项之间的间距，默认为 None
    gap: String,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            id: None,
            class: "t_grid".to_string(),
            style: None,
            childrens: Vec::new(),
            onclick: None,

            cols: None,
            rows: None,
            gap: "4px".to_string(),
        }
    }
}

impl ToElement for Grid {
    /// 渲染网格元素
    ///
    /// 将网格组件渲染为 Dioxus 元素，可以在 UI 中显示。
    ///
    /// # 返回值
    ///
    /// 返回一个 Dioxus Element，表示渲染后的网格
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// # use dioxus_blocks_components::{Grid, GridCols, ToElement};
    ///
    /// let grid = Grid::default().cols(GridCols::Col3);
    /// ```
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let mut class = self.class.clone();
        let mut style = self
            .style
            .clone()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let onclick_handler = self.onclick;
        let childrens = self.childrens_to_element();

        // 列数
        if let Some(cols) = self.cols.clone() {
            class.push_str(" t_grid-cols");

            let cols_value: u16 = cols.into();
            style.push_str(&format!(
                "grid-template-columns: repeat({}, minmax(0, 1fr));",
                cols_value
            ));
        }

        // 行数
        if let Some(rows) = self.rows.clone() {
            class.push_str(" t_grid-rows");

            let rows_value: u16 = rows.into();
            style.push_str(&format!(
                "grid-template-rows: repeat({}, minmax(0, 1fr));",
                rows_value
            ));
        }

        // 间距
        style.push_str(&format!(" gap: {};", self.gap));

        rsx! {
            div {
                id,
                class,
                style,
                onclick: move |event: MouseEvent| {
                    if let Some(handler) = onclick_handler {
                        handler.call(event);
                    }
                },
                {childrens}
            }
        }
    }
}

impl Grid {
    /// 创建一个新的网格实例
    ///
    /// # 返回值
    ///
    /// 返回一个具有默认值的网格实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Grid, GridItem, Text};
    ///
    /// let grid = Grid::new(vec![
    ///        GridItem::new(Text::new("1")),
    ///        GridItem::new(Text::new("2")),
    ///        GridItem::new(Text::new("3")),
    ///    ]);
    /// ```
    pub fn new(components: Vec<GridItem>) -> Self {
        Self {
            childrens: components
                .into_iter()
                .map(|v| Rc::new(v) as Rc<dyn ToElement>)
                .collect(),
            ..Default::default()
        }
    }

    /// 设置网格的列数
    ///
    /// # 参数
    ///
    /// * `cols` - GridCols 枚举值，定义网格的列数
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Grid, GridCols};
    /// let grid = Grid::default().cols(GridCols::Col1);
    /// ```
    pub fn cols(mut self, cols: GridCols) -> Self {
        self.cols = Some(cols);
        self
    }

    /// 设置网格的行数
    ///
    /// # 参数
    ///
    /// * `rows` - GridRows 枚举值，定义网格的行数
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::{Grid, GridRows};
    /// let grid = Grid::default().rows(GridRows::Row1);
    /// ```
    pub fn rows(mut self, rows: GridRows) -> Self {
        self.rows = Some(rows);
        self
    }

    /// 设置网格项之间的间距
    ///
    /// # 参数
    ///
    /// * `gap` 定义网格项之间的间距
    ///
    /// # 返回值
    ///
    /// 返回修改后的网格实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Grid;
    /// let grid = Grid::default().gap(4);
    /// ```
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = format!("{gap}px");
        self
    }

    /// 设置网格项之间的间距
    ///
    /// # 参数
    ///
    /// * `gap_x` 定义网格项之间的水平间距
    /// * `gap_y` 定义网格项之间的垂直间距
    /// # 返回值
    ///
    /// 返回修改后的网格实例，支持链式调用
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use dioxus_blocks_components::Grid;
    ///let grid =  Grid::default().gap_xy(4, 8);
    /// ```
    pub fn gap_xy(mut self, gap_x: usize, gap_y: usize) -> Self {
        self.gap = format!("{gap_x}px {gap_y}px");
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Text;

    use super::*;

    #[test]
    fn test_grid() {
        Grid::new(vec![
            GridItem::new(Text::new("1")),
            GridItem::new(Text::new("2")),
            GridItem::new(Text::new("3")),
        ])
        .rows(GridRows::Row4);
    }
}
