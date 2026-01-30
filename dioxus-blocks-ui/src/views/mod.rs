//! Views module for Dioxus Blocks UI
//!
//! This module contains various view components used in the application.

mod text;
pub use text::TextViewRoute;

mod link;
pub use link::LinkViewRoute;

mod button;
pub use button::ButtonViewRoute;

mod image;
pub use image::ImageViewRoute;

mod view;
pub use view::ViewExampleRoute;

mod grid;
pub use grid::GridViewRoute;

mod card;
pub use card::CardViewRoute;

mod layout;
pub use layout::LayoutViewRoute;

mod home;
pub use home::HomeViewRoute;

mod blog;
pub use blog::BlogRoute;

mod input_number;
pub use input_number::InputNumberViewRoute;

mod input;
pub use input::InputViewRoute;

mod textarea;
pub use textarea::TextareaViewRoute;

mod radio;
pub use radio::RadioViewRoute;

mod checkbox;
pub use checkbox::CheckboxViewRoute;

// mod select;
// pub use select::SelectViewRoute;
