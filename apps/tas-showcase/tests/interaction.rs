use gpui_kit::test::TestWindowExt;
use gpui_kit::{AppContext, Bounds, Point, TestAppContext, WindowBounds, WindowOptions, px, size};
use tas_showcase::view::ShowcaseView;

#[gpui_kit::test]
fn button_and_shortcut_update_the_same_counter(cx: &mut TestAppContext) {
    let (window, view) = cx.update(|cx| {
        gpui_kit::init(cx);
        tas_showcase::bind_keys(cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point::default(),
                size: size(px(640.), px(480.)),
            })),
            ..Default::default()
        };
        gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| ShowcaseView::new(window, cx))
        })
        .expect("open test window")
    });

    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(view.read(cx).value(), 0);
        assert_eq!(window.find("counter-status").label(), Some("Count: 0"));
        assert_eq!(window.find("increment").label(), Some("Increment"));

        window.click("increment", cx);
        assert_eq!(view.read(cx).value(), 1);
        assert_eq!(window.find("counter-status").label(), Some("Count: 1"));

        window.press("ctrl-i", cx);
        assert_eq!(view.read(cx).value(), 2);
        assert_eq!(window.find("counter-status").label(), Some("Count: 2"));
    })
    .expect("update test window");
}
