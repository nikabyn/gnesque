use amane::App;

mod ipc;
mod services;
mod topbar;
mod wallpaper;

fn main() {
    App::new()
        .ipc("topbar", ipc::topbar)
        .window_per_monitor(topbar::topbar)
        .window_per_monitor(wallpaper::wallpaper)
        .window_per_monitor(wallpaper::backdrop)
        .font("Adwaita Sans")
        .run();
}
