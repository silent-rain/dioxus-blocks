//! 路由包装器宏实现
//!
//! 提供用于自动生成路由组件的宏实现，将普通组件转换为可路由的组件。
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

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
/// - 如果结构体带有字段，生成的组件会接收对应的参数
/// - 生成的组件可以直接在 Dioxus 路由中使用
///
/// # 示例
///
/// ## 无参数结构体
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_blocks_macro::Route;
/// use dioxus_blocks_components::{ToElement, View};
///
/// #[derive(Debug, Default, Clone, Route)]
/// struct HomeView {}
///
/// impl ToElement for HomeView {
///     fn to_element(&self) -> Element {
///         View::new().to_element()
///     }
/// }
/// ```
///
/// ## 带参数结构体
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_blocks_macro::Route;
/// use dioxus_blocks_components::{ToElement, View};
///
/// #[derive(Debug, Clone, Route)]
/// struct BlogView {
///     id: i32,
/// }
///
/// impl ToElement for BlogView {
///     fn to_element(&self) -> Element {
///         View::new().to_element()
///     }
/// }
/// ```
pub fn impl_derive_route(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let route_name = Ident::new(&format!("{}Route", struct_name), struct_name.span());

    // 判断结构体字段
    let component_params = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();

                let params = field_names
                    .iter()
                    .zip(field_types.iter())
                    .map(|(name, ty)| {
                        quote! { #name: #ty }
                    });

                let construct_fields = field_names.iter().map(|name| {
                    quote! { #name }
                });

                Some((
                    params.collect::<Vec<_>>(),
                    construct_fields.collect::<Vec<_>>(),
                ))
            }
            Fields::Unnamed(fields) => {
                let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
                let param_names: Vec<_> = field_types
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        Ident::new(&format!("field{}", i), proc_macro2::Span::call_site())
                    })
                    .collect();

                let params = param_names
                    .iter()
                    .zip(field_types.iter())
                    .map(|(name, ty)| {
                        quote! { #name: #ty }
                    });

                let construct_fields = param_names.iter().map(|name| {
                    quote! { #name }
                });

                Some((
                    params.collect::<Vec<_>>(),
                    construct_fields.collect::<Vec<_>>(),
                ))
            }
            Fields::Unit => None,
        },
        _ => None,
    };

    let expanded: TokenStream2 = if let Some((params, construct_fields)) = component_params {
        quote! {
            #[component]
            pub fn #route_name(#(#params),*) -> Element {
                let ele = #struct_name { #(#construct_fields),* };
                ele.to_element()
            }
        }
    } else {
        quote! {
            #[component]
            pub fn #route_name() -> Element {
                let ele = #struct_name::default();
                ele.to_element()
            }
        }
    };

    TokenStream::from(expanded)
}
