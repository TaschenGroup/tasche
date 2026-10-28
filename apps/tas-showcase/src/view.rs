use tas::kit as gpui_kit;
use tas::kit::{
    Context, FocusHandle, IntoElement, ParentElement, Render, Role, Styled, TestSupportExt, Window,
    actions, component::button::Button, div, prelude::*,
};

use crate::counter::Counter;

actions!(tas_showcase, [Increment]);

/// Retained state and view for the showcase window.
pub struct ShowcaseView {
    counter: Counter,
    focus: FocusHandle,
}

impl ShowcaseView {
    /// Creates and focuses the showcase view.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        Self {
            counter: Counter::default(),
            focus,
        }
    }

    /// Returns the value shown in the window.
    pub fn value(&self) -> u64 {
        self.counter.value()
    }

    fn increment(&mut self, cx: &mut Context<Self>) {
        self.counter.increment();
        cx.notify();
    }

    fn on_increment_action(&mut self, _: &Increment, _: &mut Window, cx: &mut Context<Self>) {
        self.increment(cx);
    }
}

impl Render for ShowcaseView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = format!("Count: {}", self.counter.value());
        div()
            .key_context("TasShowcase")
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::on_increment_action))
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .id("counter-status")
                    .role(Role::Status)
                    .test_support()
                    .aria_label(status.clone())
                    .child(status),
            )
            .child(
                Button::new("increment")
                    .label("Increment")
                    .on_click(cx.listener(|this, _, _, cx| this.increment(cx))),
            )
    }
}
