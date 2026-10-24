use amane::{Fill, Full, Layer, LayerWindow, Monitor, Rectangle, Service, Size::Parent};

use crate::services::ColorScheme;

pub fn wallpaper(monitor: &Monitor) -> LayerWindow {
    let color_scheme = ColorScheme::read();
    let path = if *color_scheme == ColorScheme::Light {
        "/home/nika/Pictures/F37-day.png"
    } else {
        "/home/nika/Pictures/F37-night.png"
    };
    let image = amane::Image::cover(path).thumbnail(monitor.width, monitor.height);

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .layer(Layer::Background)
        .space(amane::Zone::Ignore)
        .namespace("gnesque-wallpaper")
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(Fill::Image(image)),
        )
}

pub fn backdrop(monitor: &Monitor) -> LayerWindow {
    let color_scheme = ColorScheme::read();
    let path = if *color_scheme == ColorScheme::Light {
        "/home/nika/Pictures/F37-day.png"
    } else {
        "/home/nika/Pictures/F37-night.png"
    };
    let image = amane::Image::cover(path)
        .thumbnail(monitor.width / 16, monitor.height / 16)
        .blurred(2);

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .layer(Layer::Background)
        .space(amane::Zone::Ignore)
        .namespace("gnesque-backdrop")
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(Fill::Image(image)),
        )
}
