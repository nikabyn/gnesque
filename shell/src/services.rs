use gnudi::{Bus, Monitor, Service, Value};
use niri_ipc::Window;
use wgpu::naga::FastHashMap;

#[derive(PartialEq, Eq)]
pub enum ColorScheme {
    NoPreference = 0,
    Dark = 1,
    Light = 2,
}

impl Service for ColorScheme {
    fn new() -> Self {
        ColorScheme::NoPreference
    }

    fn listen() {
        let bus = Bus::session();

        for signal in bus.signals("org.freedesktop.portal.Settings", "SettingChanged") {
            if !matches!(
                signal.arguments().get(0),
                Some(Value::Text(name)) if name == "org.freedesktop.appearance"
            ) {
                continue;
            }

            *ColorScheme::write() = match signal.arguments().get(2) {
                Some(Value::Number(1.0)) => ColorScheme::Dark,
                Some(Value::Number(2.0)) => ColorScheme::Light,
                _ => ColorScheme::NoPreference,
            };
        }
    }
}

pub struct Niri {
    pub windows: FastHashMap<u64, niri_ipc::Window>,
    pub overview_opened: bool,
}

impl Service for Niri {
    fn new() -> Self {
        Self {
            windows: FastHashMap::default(),
            overview_opened: false,
        }
    }

    fn listen() {
        let mut socket = niri_ipc::socket::Socket::connect().unwrap();

        let reply = socket.send(niri_ipc::Request::EventStream).unwrap();
        if !matches!(reply.unwrap(), niri_ipc::Response::Handled) {
            todo!()
        };

        let mut read_event = socket.read_events();
        while let Ok(event) = read_event() {
            let mut state = Self::write();

            match event {
                niri_ipc::Event::WindowsChanged { windows } => {
                    state.windows.clear();
                    for window in windows {
                        state.windows.insert(window.id, window);
                    }
                }
                niri_ipc::Event::WindowOpenedOrChanged { window } => {
                    state
                        .windows
                        .entry(window.id)
                        .and_modify(|old_window| *old_window = window.clone())
                        .or_insert(window);
                }
                niri_ipc::Event::WindowClosed { id } => {
                    state.windows.retain(|key, _| *key != id);
                }
                niri_ipc::Event::WindowLayoutsChanged { changes } => {
                    for (id, layout) in changes {
                        state
                            .windows
                            .entry(id)
                            .and_modify(|window| window.layout = layout);
                    }
                }
                niri_ipc::Event::OverviewOpenedOrClosed { is_open } => {
                    state.overview_opened = is_open
                }
                niri_ipc::Event::WindowFocusChanged { id } => {
                    for (_, window) in &mut state.windows {
                        window.is_focused = Some(window.id) == id;
                    }
                }
                _ => {}
            }
        }
    }
}

impl Niri {
    pub fn focused_window(&self) -> Option<Window> {
        self.windows
            .iter()
            .find_map(|(_, window)| window.is_focused.then_some(window.clone()))
    }
}

pub trait NiriWindowExt {
    fn is_fullscreen(&self, monitor: &Monitor) -> bool;
    fn is_maximised(&self, monitor: &Monitor) -> bool;
}

impl NiriWindowExt for niri_ipc::Window {
    fn is_fullscreen(&self, monitor: &Monitor) -> bool {
        self.layout.window_size.1 as i64 == monitor.height as i64
            && self.layout.window_size.0 as i64 == monitor.width as i64
    }

    fn is_maximised(&self, monitor: &Monitor) -> bool {
        self.layout.window_size.1 as i64 != monitor.height as i64
            && self.layout.window_size.0 as i64 == monitor.width as i64
    }
}
