//! Binary entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    homewizard_desktop_lib::run()
}
