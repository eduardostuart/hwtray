//! System tray icon with left-click popover and right-click context menu.

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Runtime,
};

use crate::windows;

const TRAY_ICON: &[u8] = include_bytes!("../icons/tray-icon.png");

/// Register the tray icon, context menu, and event handlers.
/// Also initializes the `tray_title` crate for menu bar metric display.
pub fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    TrayIconBuilder::new()
        .icon(tauri::image::Image::from_bytes(TRAY_ICON)?)
        .icon_as_template(true)
        .tooltip("HWTray")
        .menu(&build_menu(app)?)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_click)
        .build(app)?;

    tray_title::init();
    Ok(())
}

/// Route right-click menu selections to the appropriate window.
fn handle_menu_event<R: Runtime>(app: &tauri::AppHandle<R>, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "customize" => windows::show_tray_customize(app),
        "settings" => windows::show_settings(app),
        "about" => windows::show_about(app),
        "quit" => app.exit(0),
        _ => {}
    }
}

/// Toggle the main popover on left-click, centered under the tray icon.
fn handle_tray_click(tray_icon: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        rect,
        ..
    } = event
    {
        let pos = rect.position.to_logical::<f64>(1.0);
        let size = rect.size.to_logical::<f64>(1.0);
        windows::toggle_main(tray_icon.app_handle(), pos.x + size.width / 2.0);
    }
}

/// Build the right-click context menu.
fn build_menu<R: Runtime>(
    app: &impl tauri::Manager<R>,
) -> Result<tauri::menu::Menu<R>, Box<dyn std::error::Error>> {
    Ok(MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id("customize", "Customize Menu Bar…").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("settings", "Settings").build(app)?)
        .item(&MenuItemBuilder::with_id("about", "About").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("quit", "Quit").build(app)?)
        .build()?)
}
