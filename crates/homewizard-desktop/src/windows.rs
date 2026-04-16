//! Window creation, positioning, and lifecycle management.
//!
//! The main popover window is positioned under the tray icon and
//! supports multi-monitor setups. Secondary windows (about, customize)
//! are centered and always-on-top.

use std::sync::Arc;

use tauri::{Emitter, Manager, Runtime};

use crate::events::warn_on_err;
use crate::state::AppState;

const MAIN_WINDOW: &str = "main";
const MAIN_SIZE: (f64, f64) = (400.0, 612.0);
const SCREEN_MARGIN: f64 = 8.0;

/// Configuration for a secondary (non-main) window.
struct WindowConfig {
    label: &'static str,
    url: &'static str,
    title: &'static str,
    size: (f64, f64),
}

const TRAY_CUSTOMIZE: WindowConfig = WindowConfig {
    label: "tray-customize",
    url: "/tray-customize",
    title: "Customize Menu Bar",
    size: (440.0, 500.0),
};

const ABOUT: WindowConfig = WindowConfig {
    label: "about",
    url: "/about",
    title: "About",
    size: (420.0, 700.0),
};

/// Show the main window and navigate to the settings view.
pub fn show_settings<R: Runtime>(app: &impl Manager<R>) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        warn_on_err("emit navigate", window.emit("navigate", "/settings"));
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// Toggle the main popover: hide if visible, show + position if hidden,
/// create if it doesn't exist yet.
pub fn toggle_main<R: Runtime>(app: &impl Manager<R>, tray_center_x: f64) {
    match app.get_webview_window(MAIN_WINDOW) {
        Some(window) if window.is_visible().unwrap_or(false) => {
            let _ = window.hide();
        }
        Some(window) => {
            let _ = window.show();
            let _ = window.set_focus();
            position_under_tray(&window, tray_center_x);
        }
        None => create_main(app, tray_center_x),
    }
}

/// Show or create the "Customize Menu Bar" window.
pub fn show_tray_customize<R: Runtime>(app: &impl Manager<R>) {
    show_or_create(app, &TRAY_CUSTOMIZE);
}

/// Show or create the About window.
pub fn show_about<R: Runtime>(app: &impl Manager<R>) {
    show_or_create(app, &ABOUT);
}

/// Show an existing secondary window or create a new one (centered, always-on-top).
fn show_or_create<R: Runtime>(app: &impl Manager<R>, config: &WindowConfig) {
    if let Some(window) = app.get_webview_window(config.label) {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let _ = tauri::WebviewWindowBuilder::new(
        app,
        config.label,
        tauri::WebviewUrl::App(config.url.into()),
    )
    .title(config.title)
    .inner_size(config.size.0, config.size.1)
    .min_inner_size(320.0, 200.0)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .visible(true)
    .center()
    .build();
}

/// Create the main popover window. When `always_on_top` is false,
/// the window auto-hides on focus loss.
fn create_main<R: Runtime>(app: &impl Manager<R>, tray_center_x: f64) {
    let on_top = get_always_on_top(app);

    let window = match tauri::WebviewWindowBuilder::new(
        app,
        MAIN_WINDOW,
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("HWTray")
    .inner_size(MAIN_SIZE.0, MAIN_SIZE.1)
    .resizable(false)
    .decorations(false)
    .transparent(true)
    .visible(true)
    .skip_taskbar(true)
    .always_on_top(on_top)
    .build()
    {
        Ok(w) => w,
        Err(e) => {
            tracing::error!("Failed to create main window: {e}");
            return;
        }
    };

    position_under_tray(&window, tray_center_x);

    if !on_top {
        let handle = window.app_handle().clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                if let Some(w) = handle.get_webview_window(MAIN_WINDOW) {
                    let _ = w.hide();
                }
            }
        });
    }
}

/// Position the window centered under the tray icon, clamped to screen bounds.
/// Emits `arrow_offset` so the frontend can align the popover arrow.
fn position_under_tray<R: Runtime>(window: &tauri::WebviewWindow<R>, tray_center_x: f64) {
    let Ok(size) = window.outer_size() else {
        return;
    };
    let Some(monitor) = find_monitor_at(window, tray_center_x) else {
        return;
    };

    let scale = monitor.scale_factor();
    let screen_x = monitor.position().x as f64;
    let screen_right = screen_x + monitor.size().width as f64 / scale;
    let win_w = size.width as f64 / scale;

    let x = (tray_center_x - win_w / 2.0).clamp(
        screen_x + SCREEN_MARGIN,
        screen_right - win_w - SCREEN_MARGIN,
    );

    // Tell the frontend where to draw the popover arrow
    let arrow_x = tray_center_x - x;
    let state: tauri::State<Arc<AppState>> = window.state();
    state.set_arrow_offset(arrow_x);
    warn_on_err("emit arrow_offset", window.emit("arrow_offset", arrow_x));

    let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
        x,
        y: 0.0,
    }));
}

/// Find the monitor that contains the given x coordinate.
/// Fast path: check current monitor first, fallback to iterating all.
fn find_monitor_at<R: Runtime>(window: &tauri::WebviewWindow<R>, x: f64) -> Option<tauri::Monitor> {
    let contains = |m: &tauri::Monitor| {
        let left = m.position().x as f64;
        let right = left + m.size().width as f64 / m.scale_factor();
        x >= left && x <= right
    };

    window
        .current_monitor()
        .ok()
        .flatten()
        .filter(contains)
        .or_else(|| window.available_monitors().ok()?.into_iter().find(contains))
}

/// Read `always_on_top` from the lock-free atomic mirror so window creation
/// never stalls on the config mutex.
fn get_always_on_top<R: Runtime>(app: &impl Manager<R>) -> bool {
    let state: tauri::State<Arc<AppState>> = app.state();
    state.get_always_on_top()
}
