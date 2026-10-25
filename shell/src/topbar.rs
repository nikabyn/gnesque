use std::{iter, time::Duration};

use chrono::{DateTime, Local};
use color::{AlphaColor, Srgb};
use gnudi::{
    Align::Center, Animation, Battery, Blend, Full, Image, Justify::SpaceBetween, Layer,
    LayerWindow, Monitor, Padding, Parent, Radius, Rectangle, Row, Service, Text, Vertical, Weight,
    Widget, Workspaces, Zone, children, easing_linear,
};

use crate::services::{ColorScheme, Niri, NiriWindowExt};

pub struct Topbar {
    floating: bool,
    transition: Animation,
    border_width: f32,
    color_theme: ColorTheme,
}

impl Service for Topbar {
    fn new() -> Self {
        Self {
            floating: true,
            transition: Animation::new(0.0)
                .duration(Duration::from_millis(150))
                .easing(easing_linear),
            border_width: 2.0,
            color_theme: COLOR_THEME_DARK,
        }
    }
}

impl Topbar {
    fn set_floating(&mut self, value: bool) {
        self.floating = value;
        let goal = if self.floating { 0.0 } else { 1.0 };
        self.transition.to(goal);
    }

    pub fn margin(&self) -> Padding {
        let margin_horizontal = Blend::blend(10.0, -2.0, self.transition.value());
        let margin_top = Blend::blend(6.0, -2.0, self.transition.value());
        let margin_bottom = Blend::blend(10.0, 0.0, self.transition.value());

        Padding {
            left: margin_horizontal,
            right: margin_horizontal,
            top: margin_top,
            bottom: margin_bottom,
        }
    }

    pub fn radius(&self) -> Radius {
        Blend::blend(12.0, 0.0, self.transition.value()).into()
    }
}

#[derive(Clone)]
struct ColorTheme {
    foreground: AlphaColor<Srgb>,
    background: AlphaColor<Srgb>,
    border: AlphaColor<Srgb>,
}

const COLOR_THEME_LIGHT: ColorTheme = ColorTheme {
    foreground: AlphaColor::from_rgba8(10, 10, 10, 255),
    background: AlphaColor::from_rgba8(250, 250, 250, 220),
    border: AlphaColor::from_rgba8(0, 0, 0, 15),
};
const COLOR_THEME_DARK: ColorTheme = ColorTheme {
    foreground: AlphaColor::from_rgba8(250, 250, 250, 255),
    background: AlphaColor::from_rgba8(30, 30, 30, 220),
    border: AlphaColor::from_rgba8(0, 0, 0, 50),
};

pub fn topbar(monitor: &Monitor) -> LayerWindow {
    let mut bar = Topbar::write();

    let floating = !Niri::read()
        .focused_window()
        .is_some_and(|window| window.is_maximised(monitor));
    bar.set_floating(floating);

    bar.color_theme = if *ColorScheme::read() == ColorScheme::Light {
        COLOR_THEME_LIGHT
    } else {
        COLOR_THEME_DARK
    };

    let margin = topbar_margin(bar.margin());
    let content = topbar_bar(bar.radius(), bar.border_width, &bar.color_theme);

    LayerWindow::new()
        .width(Full)
        .height(48.0)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .space(Zone::Reserve)
        .child(margin.child(content))
        .namespace("gnesque-topbar")
}

fn topbar_margin(margin: Padding) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .padding(margin)
}

fn topbar_bar(radius: Radius, border_width: f32, color_theme: &ColorTheme) -> impl Widget + use<> {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .radius(radius)
        .border(border_width, color_theme.border)
        .fill(color_theme.background)
        // TODO Do not render shadow behind transparent surfaces
        // .shadow(Shadow::drop("#0000003c").blur(12.0).offset(0.0, 2.0))
        .child(
            Row::new(children![
                widget_start(color_theme),
                widget_center(color_theme),
                widget_end(color_theme),
            ])
            .width(Parent)
            .height(Parent)
            .justify(SpaceBetween)
            .align(Center),
        )
}

fn widget_start(color_theme: &ColorTheme) -> impl Widget + use<> {
    let workspaces = Workspaces::read();
    let niri_charm = Image::contain("/home/nika/dev/gnesque/shell/resources/niri.svg");

    let shapes = workspaces
        .list()
        .iter()
        .map(|workspace| {
            let diameter = 8.0;
            if workspace.active() {
                let circle = Rectangle::new()
                    .width(diameter * 2.0)
                    .height(diameter)
                    .radius(Full);
                circle.fill(color_theme.foreground)
            } else {
                let circle = Rectangle::new()
                    .width(diameter)
                    .height(diameter)
                    .radius(Full);
                circle.fill(color_theme.foreground.with_alpha(0.2))
            }
        })
        .map(|widget| Box::new(widget) as Box<dyn Widget>);

    let children = iter::chain(
        iter::once(Rectangle::new().width(20.0).height(20.0).fill(niri_charm))
            .map(|widget| Box::new(widget) as Box<dyn Widget>),
        shapes,
    );

    button().child(Row::new(children.collect()).align(Center).gap(4.0))
}

#[derive(PartialEq, Eq)]
pub struct Clock {
    pub date: String,
    pub time: String,
}

impl Clock {
    fn from_datetime(datetime: DateTime<Local>) -> Self {
        Self {
            date: format!("{}", datetime.format("%d.%m.%Y")),
            time: format!("{}", datetime.format("%H:%M")),
        }
    }
}

impl Service for Clock {
    fn new() -> Self {
        Self::from_datetime(Local::now())
    }

    fn interval() -> Duration {
        Duration::from_secs(1)
    }

    fn update(&mut self) -> bool {
        let next = Self::from_datetime(Local::now());
        let changed = &next != self;
        *self = next;

        changed
    }
}

fn widget_center(color_theme: &ColorTheme) -> impl Widget + use<> {
    let formatted_time = &Clock::read();
    button().child(
        Row::new(children![
            Text::new(&formatted_time.date)
                .weight(Weight::Medium)
                .color(color_theme.foreground),
            Text::new(&formatted_time.time)
                .weight(Weight::Medium)
                .tabular_numerics(true)
                .color(color_theme.foreground),
        ])
        .gap(10.0),
    )
}

fn widget_end(color_theme: &ColorTheme) -> impl Widget + use<> {
    let battery = Battery::read();
    let color = match battery.percent() {
        0..=5 => color::palette::css::RED,
        6..=15 => AlphaColor::from_rgb8(255, 255, 0),
        _ => color_theme.foreground,
    };
    let status = format!(
        "{}{}%",
        if battery.charging() { "+" } else { "" },
        battery.percent()
    );
    button().child(Text::new(status).weight(Weight::Medium).color(color))
}

fn button() -> Rectangle {
    Rectangle::new()
        .width(100.0)
        .height(Parent)
        .padding(12.0)
        .align_child(Center, Center)
}
