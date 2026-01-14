//! Grid 组件
//!
//! 提供一个可自定义的网格布局组件，支持类似 Tailwind CSS 的 grid 功能。
//! 支持自定义列数、行数、间距等属性。
//!
//! # 示例
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_blocks_components::{Grid, GridCols, ToElement, Text};
//!
//! #[component]
//! fn App() -> Element {
//!     let components = vec![
//!         Text::new("Batch 1"),
//!         Text::new("Batch 2"),
//!         Text::new("Batch 3"),
//!     ];
//!     Grid::new()
//!         .cols(GridCols::Col4)
//!         .gap(4)
//!         .childrens(components)
//!         .to_element()
//! }
//! ```
use std::sync::Arc;

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

impl GridCols {
    /// 获取列数对应的数值
    pub fn value(&self) -> u16 {
        match self {
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

impl GridRows {
    /// 获取行数对应的数值
    pub fn value(&self) -> u16 {
        match self {
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
    childrens: Vec<Arc<dyn ToElement>>,
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
    /// # use dioxus_blocks_components::GridItem;
    /// let grid_item = GridItem::new();
    /// ```
    pub fn new() -> Self {
        Self {
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
    /// GridItem::new().col_span(2);
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
    /// GridItem::new().row_span(2);
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
    /// GridItem::new().col_start(3);
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
    /// GridItem::new().col_end(4);
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
    /// GridItem::new().row_start(2);
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
    /// GridItem::new().row_end(3);
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
    /// GridItem::new().col_span(2).to_element();
    /// ```
    fn to_element(&self) -> Element {
        let id = self.id.clone();
        let mut class = self.class.clone();
        let mut style = self
            .style
            .clone()
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let childrens = self.childrens_to_element();

        // 添加自定义样式
        if self.col_span > 12 {
            style.push_str(&format!("grid-column: span {};", self.col_span));
        }

        if self.row_span > 6 {
            style.push_str(&format!("grid-row: span {};", self.row_span));
        }

        if self.col_start > 13 {
            style.push_str(&format!("grid-column-start: {};", self.col_start));
        }

        if self.col_end > 13 {
            style.push_str(&format!("grid-column-end: {};", self.col_end));
        }

        if self.row_start > 7 {
            style.push_str(&format!("grid-row-start: {};", self.row_start));
        }

        if self.row_end > 7 {
            style.push_str(&format!("grid-row-end: {};", self.row_end));
        }

        // 添加列跨度类名
        if self.col_span <= 12 {
            class.push_str(&format!(" t_col-span-{}", self.col_span));
        }

        // 添加行跨度类名
        if self.row_span <= 6 {
            class.push_str(&format!(" t_row-span-{}", self.row_span));
        }

        // 添加列起始位置类名
        if self.col_start == 0 {
            class.push_str(" t_col-start-auto");
        } else if self.col_start <= 13 {
            class.push_str(&format!(" t_col-start-{}", self.col_start));
        }

        // 添加列结束位置类名
        if self.col_end == 0 {
            class.push_str(" t_col-end-auto");
        } else if self.col_end <= 13 {
            class.push_str(&format!(" t_col-end-{}", self.col_end));
        }

        // 添加行起始位置类名
        if self.row_start == 0 {
            class.push_str(" t_row-start-auto");
        } else if self.row_start <= 7 {
            class.push_str(&format!(" t_row-start-{}", self.row_start));
        }

        // 添加行结束位置类名
        if self.row_end == 0 {
            class.push_str(" t_row-end-auto");
        } else if self.row_end <= 7 {
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
    childrens: Vec<Arc<dyn ToElement>>,
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
    /// let grid = Grid::new().cols(GridCols::Col3);
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

        // 对于列数
        if let Some(cols) = &self.cols {
            class.push_str(" t_grid-cols");

            let cols_value = cols.value();
            style.push_str(&format!(
                "grid-template-columns: repeat({}, minmax(0, 1fr));",
                cols_value
            ));
        }

        // 对于行数
        if let Some(rows) = &self.rows {
            class.push_str(" t_grid-rows");

            let rows_value = rows.value();
            style.push_str(&format!(
                "grid-template-rows: repeat({}, minmax(0, 1fr));",
                rows_value
            ));
        }

        // 对于间距
        style.push_str(&format!(" gap: {}", self.gap));

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
    /// # use dioxus_blocks_components::Grid;
    /// let grid = Grid::new();
    /// ```
    pub fn new() -> Self {
        Self {
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
    /// let grid = Grid::new().cols(GridCols::Col1);
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
    /// let grid = Grid::new().rows(GridRows::Row1);
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
    /// let grid = Grid::new().gap(4);
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
    ///let grid =  Grid::new().gap_xy(4, 8);
    /// ```
    pub fn gap_xy(mut self, gap_x: usize, gap_y: usize) -> Self {
        self.gap = format!("{gap_x}px {gap_y}px");
        self
    }
}
