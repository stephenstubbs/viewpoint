#![allow(clippy::float_cmp, clippy::assertions_on_constants)]

use super::*;

#[test]
fn test_iphone_13_descriptor() {
    assert_eq!(IPHONE_13.name, "iPhone 13");
    assert_eq!(IPHONE_13.viewport.width, 390);
    assert_eq!(IPHONE_13.viewport.height, 844);
    assert_eq!(IPHONE_13.device_scale_factor, 3.0);
    assert!(IPHONE_13.is_mobile);
    assert!(IPHONE_13.has_touch);
}

#[test]
fn test_desktop_chrome_descriptor() {
    assert_eq!(DESKTOP_CHROME.name, "Desktop Chrome");
    assert_eq!(DESKTOP_CHROME.viewport.width, 1280);
    assert_eq!(DESKTOP_CHROME.viewport.height, 720);
    assert_eq!(DESKTOP_CHROME.device_scale_factor, 1.0);
    assert!(!DESKTOP_CHROME.is_mobile);
    assert!(!DESKTOP_CHROME.has_touch);
}

#[test]
fn test_all_devices() {
    let devices = all_devices();
    assert!(devices.len() > 30);
}

#[test]
fn test_find_device() {
    let device = find_device("iPhone 13");
    assert!(device.is_some());
    assert_eq!(device.unwrap().name, "iPhone 13");

    let device_case = find_device("iphone 13");
    assert!(device_case.is_some());

    let not_found = find_device("NonexistentDevice");
    assert!(not_found.is_none());
}
