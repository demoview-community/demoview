use gpui::{
    black, blue, colors, div, green, prelude::*, px, rgb, size, white, App, Application, Bounds, Context, ContextEntry, Entity, SharedString, Timer, Window, WindowBounds, WindowKind, WindowOptions
};
use raylib::color::Color;

trait Factory {
    fn new() -> Self
    where Self: Sized;
}
#[derive(Copy, Clone)]
struct Packets;
#[derive(Copy, Clone)]
struct Viewer;
#[derive(Copy, Clone)]
struct Info;
#[derive(Copy, Clone)]
struct Hex;
impl Factory for Packets {
    fn new() -> Packets {
        Packets {}
    }
}
impl Factory for Viewer {
    fn new() -> Viewer {
        Viewer {}
    }
}
impl Factory for Info {
    fn new() -> Info {
        Info {}
    }
}
impl Factory for Hex {
    fn new() -> Hex {
        Hex {}
    }
}
impl Render for Packets {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .child("packets")
    }
}
impl Render for Viewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(green())
            .child("viewer")
    }
}
impl Render for Info {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child("info")
    }
}
impl Render for Hex {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child("hex")
    }
}
struct MainPage {
    packets: Packets,
    viewer: Viewer,
    info: Info,
    hex: Hex
}
impl Factory for MainPage {
    fn new() -> MainPage {
        MainPage {
            packets: Packets::new(),
            viewer: Viewer::new(),
            info: Info::new(),
            hex: Hex::new()
        }
    }
}
impl Render for MainPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(div()
            .flex_grow()
            .flex()
            .flex_col()
            .size_full()
            .text_xl()
            .text_color(white())
            .bg(black())
            .items_center()
            .justify_center()
            .m_1()
            .gap_1()
            .child(
                div()
                .size_full()
                .flex()
                .flex_row()
                .gap_1()
                .child(
                    cx.new(|_| self.packets)
                )
                .child(
                    cx.new(|_| self.viewer)
                )
            )
            .child(
                div()
                .size_full()
                .flex()
                .flex_row()
                .gap_1()
                .child(
                    cx.new(|_| self.info)
                )
                .child(
                    cx.new(|_| self.hex)
                )
            )
        )
    }
}
fn main() {
    println!("Hello, world!");
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        let window_result = cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        }, |_, cx| {
            cx.new(|_| MainPage::new())
        });
        match window_result {
            Ok(_) => println!("yay"),
            Err(e) => println!("{}", e)
        }
    })
}
