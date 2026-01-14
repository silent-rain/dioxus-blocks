//! 路由包装器宏实现
//!
//! 提供用于自动生成路由组件的宏实现，将普通组件转换为可路由的组件。
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// 为给定的结构体实现 `Route` 派生宏
///
/// 该宏会生成一个与结构体同名的组件函数（后缀为"Route"），
/// 该组件会创建并渲染原始结构体的实例。这使得普通组件可以
/// 在 Dioxus 路由系统中使用。
///
/// # 功能
///
/// - 自动生成与原始组件同名的路由组件（添加"Route"后缀）
/// - 路由组件会创建原始组件的默认实例并渲染
/// - 生成的组件可以直接在 Dioxus 路由中使用
pub fn impl_derive_route(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let route_name = Ident::new(&format!("{}Route", struct_name), struct_name.span());

    let expanded: TokenStream2 = quote! {
        #[component]
        pub fn #route_name() -> Element {
            let ele = #struct_name::default();
            rsx! {
                {ele.to_element()}
            }
        }
    };

    TokenStream::from(expanded)
}
