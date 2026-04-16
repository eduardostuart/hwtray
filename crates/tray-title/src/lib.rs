//! Set styled (attributed) tray icon titles on macOS.
//!
//! Creates independent `NSStatusItem`s in the menu bar, each with
//! a styled two-line label (name + value).

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::{init, update_items};

#[cfg(not(target_os = "macos"))]
pub fn init() {}

#[cfg(not(target_os = "macos"))]
pub fn update_items(_items: &[TrayItem<'_>]) {}

/// RGB color (0.0–1.0 per channel).
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// A segment of text with optional color.
pub struct Span<'a> {
    pub text: &'a str,
    pub color: Option<Color>,
}

/// A single metric to display as an independent menu bar item.
pub struct TrayItem<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub value_spans: &'a [Span<'a>],
}
