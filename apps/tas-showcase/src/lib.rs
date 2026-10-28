//! A small development-only consumer of the Tasche application entry point.

pub mod counter;
pub mod view;

use tas::kit::{App, AppContext, KeyBinding, WindowOptions};

/// Registers the shortcuts used by the showcase.
pub fn bind_keys(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "ctrl-i",
        view::Increment,
        Some("TasShowcase"),
    )]);
}

/// Opens the showcase window and returns any startup failure.
pub fn run() -> Result<(), tas::RunError> {
    tas::run(WindowOptions::default(), |window, cx| {
        bind_keys(cx);
        window.set_window_title("Tasche Showcase");
        cx.new(|cx| view::ShowcaseView::new(window, cx))
    })
}
