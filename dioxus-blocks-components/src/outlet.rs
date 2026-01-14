//! Outlet component for rendering routed content

use dioxus::prelude::*;
use dioxus::router::Routable;

use crate::ToElement;

#[derive(Debug, Clone)]
pub struct Outlet<R: Routable + Clone> {
    _phantom: std::marker::PhantomData<R>,
}

impl<R: Routable + Clone + std::fmt::Debug> ToElement for Outlet<R> {
    fn to_element(&self) -> Element {
        rsx! {
            dioxus::prelude::Outlet::<R> {}
        }
    }
}

impl<R: Routable + Clone + std::fmt::Debug> Default for Outlet<R> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
