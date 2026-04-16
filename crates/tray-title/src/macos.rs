//! macOS implementation using NSStatusItem and NSAttributedString.

use std::cell::RefCell;
use std::collections::HashMap;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{msg_send, MainThreadMarker};
use objc2_app_kit::{NSFont, NSTextAlignment, NSVariableStatusItemLength};
use objc2_foundation::{NSAttributedString, NSDictionary, NSMutableAttributedString, NSString};

use crate::{Color, Span, TrayItem};

thread_local! {
    static STATUS_ITEMS: RefCell<HashMap<String, Retained<AnyObject>>> =
        RefCell::new(HashMap::new());
}

/// No-op — items are created on demand in `update_items`.
pub fn init() {}

/// Sync the set of menu bar status items to match the provided list.
/// Creates new items, updates existing ones, and removes stale ones.
pub fn update_items(items: &[TrayItem<'_>]) {
    let Some(_mtm) = MainThreadMarker::new() else {
        return;
    };

    STATUS_ITEMS.with(|cell| {
        let mut map = cell.borrow_mut();

        let active_ids: Vec<String> = items.iter().map(|i| i.id.to_string()).collect();
        let stale: Vec<String> = map
            .keys()
            .filter(|k| !active_ids.contains(k))
            .cloned()
            .collect();
        for id in stale {
            if let Some(item) = map.remove(&id) {
                remove_status_item(&item);
            }
        }

        for tray_item in items {
            let ns_item = map
                .entry(tray_item.id.to_string())
                .or_insert_with(create_status_item);

            set_item_title(ns_item, tray_item.name, tray_item.value_spans);
        }
    });
}

fn create_status_item() -> Retained<AnyObject> {
    unsafe {
        let cls = objc2::runtime::AnyClass::get(c"NSStatusBar").unwrap();
        let bar: Retained<AnyObject> = msg_send![cls, systemStatusBar];
        msg_send![&*bar, statusItemWithLength: NSVariableStatusItemLength]
    }
}

fn remove_status_item(item: &AnyObject) {
    unsafe {
        let cls = objc2::runtime::AnyClass::get(c"NSStatusBar").unwrap();
        let bar: Retained<AnyObject> = msg_send![cls, systemStatusBar];
        let _: () = msg_send![&*bar, removeStatusItem: item];
    }
}

fn set_item_title(item: &AnyObject, name: &str, value_spans: &[Span<'_>]) {
    let button: *mut AnyObject = unsafe { msg_send![item, button] };
    if button.is_null() {
        return;
    }

    let result = NSMutableAttributedString::from_nsstring(&NSString::from_str(""));

    // Line 1: device name
    append_styled(&result, &format!("{name}\n"), 7.0, 0.0, None);

    // Line 2: value spans with optional colors
    for span in value_spans {
        append_styled(&result, span.text, 10.0, 0.5, span.color);
    }

    unsafe {
        let _: () = msg_send![button, setAttributedTitle: &*result];
    }
}

fn append_styled(
    result: &NSMutableAttributedString,
    text: &str,
    size: f64,
    weight: f64,
    color: Option<Color>,
) {
    let ns_text = NSString::from_str(text);
    let font = NSFont::monospacedDigitSystemFontOfSize_weight(size, weight);

    let style: Retained<AnyObject> = unsafe {
        let cls = objc2::runtime::AnyClass::get(c"NSMutableParagraphStyle").unwrap();
        let style: *mut AnyObject = msg_send![cls, new];
        let _: () = msg_send![style, setAlignment: NSTextAlignment::Center];
        let line_h = size + 1.0;
        let _: () = msg_send![style, setMinimumLineHeight: line_h];
        let _: () = msg_send![style, setMaximumLineHeight: line_h];
        Retained::from_raw(style).unwrap()
    };

    let baseline_num: Retained<AnyObject> = unsafe {
        let cls = objc2::runtime::AnyClass::get(c"NSNumber").unwrap();
        msg_send![cls, numberWithDouble: -2.0_f64]
    };

    let font_key = NSString::from_str("NSFont");
    let style_key = NSString::from_str("NSParagraphStyle");
    let baseline_key = NSString::from_str("NSBaselineOffset");

    if let Some(c) = color {
        let ns_color: Retained<AnyObject> = unsafe {
            let cls = objc2::runtime::AnyClass::get(c"NSColor").unwrap();
            msg_send![cls, colorWithRed: c.r, green: c.g, blue: c.b, alpha: 1.0_f64]
        };

        let color_key = NSString::from_str("NSColor");
        let keys: &[&NSString] = &[&font_key, &style_key, &baseline_key, &color_key];
        let values: &[&AnyObject] = unsafe {
            &[
                core::mem::transmute::<&NSFont, &AnyObject>(&font),
                &*style,
                &*baseline_num,
                &*ns_color,
            ]
        };
        let dict = NSDictionary::from_slices(keys, values);
        let attr_str = unsafe { NSAttributedString::new_with_attributes(&ns_text, &dict) };
        result.appendAttributedString(&attr_str);
    } else {
        let keys: &[&NSString] = &[&font_key, &style_key, &baseline_key];
        let values: &[&AnyObject] = unsafe {
            &[
                core::mem::transmute::<&NSFont, &AnyObject>(&font),
                &*style,
                &*baseline_num,
            ]
        };
        let dict = NSDictionary::from_slices(keys, values);
        let attr_str = unsafe { NSAttributedString::new_with_attributes(&ns_text, &dict) };
        result.appendAttributedString(&attr_str);
    }
}
