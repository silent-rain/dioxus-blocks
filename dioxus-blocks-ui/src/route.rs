// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use crate::{
    LayoutRoute,
    views::{BlogRoute, GridViewRoute, HomeViewRoute, ImageViewRoute, TextViewRoute},
};

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
///
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(LayoutRoute)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        HomeViewRoute {},
        #[route("/grid")]
        GridViewRoute {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the BlogRoute component must accept
        // an `id` prop of type `i32`.
        BlogRoute { id: i32 },
        // #[route("/card")]
        // CardViewRoute {},
        // #[route("/grid")]
        // GridViewRoute {},
        #[route("/text")]
        TextViewRoute {},
        #[route("/image")]
        ImageViewRoute {},
}
