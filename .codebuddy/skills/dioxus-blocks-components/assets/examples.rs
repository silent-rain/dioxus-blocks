//! Dioxus Blocks Components 完整示例
//!
//! 本文件包含多个完整可运行的示例，展示了 `dioxus-blocks-components` 的各种用法。

use dioxus::prelude::*;
use dioxus_blocks_components::*;

// ============================================================================
// 示例1：基础布局
// ============================================================================

/// 基础布局示例
/// 
/// 展示如何使用 View、Card、Grid 构建基础布局
#[component]
fn Example1_BasicLayout() -> Element {
    rsx! {
        Header {}
        MainContent {}
        Footer {}
    }
}

#[component]
fn Header() -> Element {
    View::new()
        .style(|s| s
            .height("64px")
            .background_color("#1976d2")
            .color("white")
            .display("flex")
            .justify_content("center")
            .align_items("center")
        )
        .children(Text::h1("Dioxus Blocks").style(|s| s.color("white")))
        .to_element()
}

#[component]
fn MainContent() -> Element {
    View::new()
        .style(|s| s
            .padding("32px")
            .max_width("1200px")
            .margin("0 auto")
        )
        .children(
            Grid::new(vec![
                GridItem::new(
                    Card::new()
                        .header(Text::h3("卡片1"))
                        .body(Text::p("这是第一个卡片的内容"))
                ),
                GridItem::new(
                    Card::new()
                        .header(Text::h3("卡片2"))
                        .body(Text::p("这是第二个卡片的内容"))
                ),
                GridItem::new(
                    Card::new()
                        .header(Text::h3("卡片3"))
                        .body(Text::p("这是第三个卡片的内容"))
                ),
            ])
            .cols(GridCols::Col3)
            .gap(24)
        )
        .to_element()
}

#[component]
fn Footer() -> Element {
    View::new()
        .style(|s| s
            .padding("24px")
            .background_color("#f5f5f5")
            .text_align("center")
            .color("#666")
        )
        .children(Text::p("© 2024 Dioxus Blocks. All rights reserved."))
        .to_element()
}

// ============================================================================
// 示例2：表单组件
// ============================================================================

/// 表单组件示例
///
/// 展示如何使用 Input、Select、Radio、Checkbox、Button 构建表单
#[component]
fn Example2_Form() -> Element {
    rsx! {
        FormContainer {}
    }
}

#[component]
fn FormContainer() -> Element {
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut role = use_signal(String::new);
    
    let mut interests = use_signal(Vec::<String>::new);
    let mut agree = use_signal(false);
    
    Card::new()
        .header(Text::h3("用户注册"))
        .body(
            View::new()
                .style(|s| s.display("flex").flex_direction("column").gap("16px"))
                .childrens2(vec![
                    Text::new("用户名").style(|s| s.font_weight("bold")).to_element(),
                    Input::new()
                        .value(username)
                        .placeholder("请输入用户名")
                        .to_element(),
                    
                    Text::new("邮箱").style(|s| s.font_weight("bold")).to_element(),
                    Input::new()
                        .input_type(InputType::Email)
                        .value(email)
                        .placeholder("请输入邮箱")
                        .to_element(),
                    
                    Text::new("密码").style(|s| s.font_weight("bold")).to_element(),
                    Input::new()
                        .input_type(InputType::Password)
                        .value(password)
                        .placeholder("请输入密码")
                        .to_element(),
                    
                    Text::new("角色").style(|s| s.font_weight("bold")).to_element(),
                    Select::new(vec![
                        SelectOption::new("user", "普通用户"),
                        SelectOption::new("admin", "管理员"),
                        SelectOption::new("guest", "访客"),
                    ])
                    .placeholder("请选择角色")
                    .to_element(),
                    
                    Text::new("兴趣").style(|s| s.font_weight("bold")).to_element(),
                    CheckboxGroup::new(vec![
                        Checkbox::new("tech", "科技"),
                        Checkbox::new("art", "艺术"),
                        Checkbox::new("sports", "运动"),
                    ])
                    .to_element(),
                    
                    Text::new("同意条款").style(|s| s.font_weight("bold")).to_element(),
                    Checkbox::new("agree", "我同意服务条款")
                        .to_element(),
                ])
                .to_element()
        )
        .footer(
            View::new()
                .style(|s| s.display("flex").justify_content("flex-end").gap("12px"))
                .childrens2(vec![
                    Button::new()
                        .text("取消")
                        .as_text()
                        .to_element(),
                    Button::new()
                        .text("注册")
                        .as_primary()
                        .onclick(move |_| {
                            println!("注册信息:");
                            println!("用户名: {}", username());
                            println!("邮箱: {}", email());
                            println!("角色: {}", role());
                            println!("兴趣: {:?}", interests());
                        })
                        .to_element(),
                ])
                .to_element()
        )
        .to_element()
}

// ============================================================================
// 示例3：状态管理
// ============================================================================

/// 状态管理示例
///
/// 展示如何使用 Signal、use_resource、use_context 管理状态
#[component]
fn Example3_StateManagement() -> Element {
    rsx! {
        ThemeProvider {
            children: rsx! {
                CounterExample {}
                AsyncDataExample {}
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    background_color: String,
    text_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
        }
    }
}

#[component]
fn ThemeProvider(children: Element) -> Element {
    let mut theme = use_signal(Theme::default);
    
    use_context_provider(|| theme.clone());
    
    rsx! {
        View::new()
            .style(move |s| {
                s.background_color(theme().background_color.clone())
                    .color(theme().text_color.clone())
                    .min_height("100vh")
            })
            .children([
                ThemeSwitcher {},
                children,
            ])
            .to_element()
    }
}

#[component]
fn ThemeSwitcher() -> Element {
    let theme: Signal<Theme> = use_context();
    
    Button::new()
        .text("切换主题")
        .onclick(move |_| {
            let current = theme();
            let new_theme = if current.background_color == "#ffffff" {
                Theme {
                    background_color: "#121212".to_string(),
                    text_color: "#ffffff".to_string(),
                }
            } else {
                Theme {
                    background_color: "#ffffff".to_string(),
                    text_color: "#333333".to_string(),
                }
            };
            theme.set(new_theme);
        })
        .to_element()
}

#[component]
fn CounterExample() -> Element {
    let mut count = use_signal(|| 0);
    let doubled = use_memo(move || count() * 2);
    
    Card::new()
        .header(Text::h3("计数器"))
        .body(
            View::new()
                .style(|s| s.display("flex").flex_direction("column").gap("16px").align_items("center"))
                .children([
                    Text::new(format!("当前值: {}", count())).to_element(),
                    Text::new(format!("双倍值: {}", doubled())).to_element(),
                    View::new()
                        .style(|s| s.display("flex").gap("12px"))
                        .children([
                            Button::new()
                                .text("-1")
                                .onclick(move |_| *count.write() -= 1)
                                .to_element(),
                            Button::new()
                                .text("+1")
                                .onclick(move |_| *count.write() += 1)
                                .to_element(),
                        ])
                        .to_element(),
                ])
                .to_element()
        )
        .to_element()
}

#[component]
fn AsyncDataExample() -> Element {
    let data = use_resource(move || async move {
        // 模拟异步数据获取
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        Ok::<String, String>("异步数据加载成功".to_string())
    });
    
    Card::new()
        .header(Text::h3("异步数据"))
        .body(
            match data() {
                Some(Ok(msg)) => Text::new(msg).style(|s| s.color("green")).to_element(),
                Some(Err(e)) => Text::new(format!("错误: {}", e)).style(|s| s.color("red")).to_element(),
                None => Text::new("加载中...").style(|s| s.color("#666")).to_element(),
            }
        )
        .to_element()
}

// ============================================================================
// 示例4：路由导航
// ============================================================================

/// 路由导航示例
///
/// 展示如何使用 Link 和 NavigationTarget 进行路由导航
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/products")]
    Products {},
    #[route("/products/:id")]
    ProductDetail { id: String },
}

#[component]
fn Example4_Navigation() -> Element {
    rsx! {
        Navbar {}
        Outlet::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    View::new()
        .style(|s| s
            .height("64px")
            .background_color("#ffffff")
            .border_bottom("1px solid #e0e0e0")
            .display("flex")
            .justify_content("center")
            .align_items("center")
            .gap("24px")
        )
        .childrens2(vec![
            Link::new(NavigationTarget::from(Route::Home {}))
                .text("首页")
                .to_element(),
            Link::new(NavigationTarget::from(Route::About {}))
                .text("关于")
                .to_element(),
            Link::new(NavigationTarget::from(Route::Products {}))
                .text("产品")
                .to_element(),
        ])
        .to_element()
}

#[component]
fn Home() -> Element {
    Card::new()
        .header(Text::h3("首页"))
        .body(Text::p("欢迎使用 Dioxus Blocks！"))
        .to_element()
}

#[component]
fn About() -> Element {
    Card::new()
        .header(Text::h3("关于"))
        .body(Text::p("Dioxus Blocks 是一个基于 Dioxus 0.7 的组件库。"))
        .to_element()
}

#[component]
fn Products() -> Element {
    Card::new()
        .header(Text::h3("产品列表"))
        .body(
            Grid::new(vec![
                GridItem::new(
                    Card::new()
                        .body(Text::p("产品1"))
                ),
                GridItem::new(
                    Card::new()
                        .body(Text::p("产品2"))
                ),
                GridItem::new(
                    Card::new()
                        .body(Text::p("产品3"))
                ),
            ])
            .cols(GridCols::Col3)
            .gap(16)
        )
        .to_element()
}

#[component]
fn ProductDetail(id: String) -> Element {
    Card::new()
        .header(Text::h3(&format!("产品详情 - {}", id)))
        .body(Text::p(&format!("这是产品 {} 的详细信息", id)))
        .to_element()
}

// ============================================================================
// 示例5：列表渲染
// ============================================================================

/// 列表渲染示例
///
/// 展示如何渲染列表，支持条件渲染和分页
#[component]
fn Example5_ListRendering() -> Element {
    rsx! {
        ProductList {}
    }
}

#[derive(Clone, Debug)]
struct Product {
    id: i32,
    name: String,
    price: f64,
}

impl Product {
    fn demo_products() -> Vec<Product> {
        vec![
            Product { id: 1, name: "产品A".to_string(), price: 99.0 },
            Product { id: 2, name: "产品B".to_string(), price: 199.0 },
            Product { id: 3, name: "产品C".to_string(), price: 299.0 },
            Product { id: 4, name: "产品D".to_string(), price: 399.0 },
            Product { id: 5, name: "产品E".to_string(), price: 499.0 },
            Product { id: 6, name: "产品F".to_string(), price: 599.0 },
        ]
    }
}

#[component]
fn ProductList() -> Element {
    let products = use_signal(Product::demo_products);
    let mut page = use_signal(|| 1);
    let page_size = 3;
    
    let start = (page() - 1) * page_size;
    let end = start + page_size;
    let current_products = products()[start..end.min(products().len())].to_vec();
    
    View::new()
        .style(|s| s.padding("32px"))
        .children([
            Grid::new(
                current_products.into_iter()
                    .map(|product| 
                        GridItem::new(
                            Card::new()
                                .header(Text::h3(&product.name))
                                .body(Text::p(format!("￥{:.2}", product.price)))
                        )
                    )
                    .collect()
            )
            .cols(GridCols::Col3)
            .gap(24)
            .to_element(),
            Pagination {
                page: page(),
                total_pages: (products().len() + page_size - 1) / page_size,
                on_prev: move |_| {
                    if page() > 1 {
                        *page.write() -= 1;
                    }
                },
                on_next: move |_| {
                    let total = (products().len() + page_size - 1) / page_size;
                    if page() < total {
                        *page.write() += 1;
                    }
                },
            },
        ])
        .to_element()
}

#[component]
fn Pagination(
    page: usize,
    total_pages: usize,
    on_prev: EventHandler<MouseEvent>,
    on_next: EventHandler<MouseEvent>,
) -> Element {
    View::new()
        .style(|s| s
            .display("flex")
            .justify_content("center")
            .align_items("center")
            .gap("16px")
            .margin_top("24px")
        )
        .children([
            Button::new()
                .text("上一页")
                .disabled(page == 1)
                .as_text()
                .onclick(on_prev)
                .to_element(),
            Text::new(format!("{} / {}", page, total_pages)).to_element(),
            Button::new()
                .text("下一页")
                .disabled(page == total_pages)
                .as_text()
                .onclick(on_next)
                .to_element(),
        ])
        .to_element()
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        View::new()
            .style(|s| s.min_height("100vh").background_color("#f5f5f5"))
            .childrens2(vec![
                Link::new(NavigationTarget::from("/examples/1"))
                    .text("示例1：基础布局")
                    .to_element(),
                Link::new(NavigationTarget::from("/examples/2"))
                    .text("示例2：表单组件")
                    .to_element(),
                Link::new(NavigationTarget::from("/examples/3"))
                    .text("示例3：状态管理")
                    .to_element(),
                Link::new(NavigationTarget::from("/examples/4"))
                    .text("示例4：路由导航")
                    .to_element(),
                Link::new(NavigationTarget::from("/examples/5"))
                    .text("示例5：列表渲染")
                    .to_element(),
            ])
            .to_element()
    }
}
