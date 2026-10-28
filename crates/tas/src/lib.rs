//! Common application entry points for Tasche.
//!
//! This version supports the default GPUI Kit backend. Enabling GPUI Kit's
//! alternative `gpui-fast` backend is not supported by this entry point.

mod application;

pub use application::{RunError, RunErrorKind, run};
pub use gpui_kit as kit;
