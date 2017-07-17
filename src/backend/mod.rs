
#[cfg(target_os = "android")]
use android_support;

// extern {
//     fn beets_init();
//     fn beets_init_gtk();
// }

pub fn init() {
    init_platform();
}

#[cfg(target_os = "android")]
fn init_platform() {
    android_support::write_log("beetflo::init_platform()");
}

#[cfg(target_os = "ios")]
fn init_platform() {
}

#[cfg(target_os = "windows")]
fn init_platform() {
}

#[cfg(target_os = "macos")]
fn init_platform() {
}

#[cfg(target_os = "linux")]
fn init_platform() {
    // unsafe {
        // beets_init();
        // beets_init_gtk();
    // }
}

