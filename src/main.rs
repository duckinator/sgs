#![cfg(not(target_os = "android"))] // Don't compile this file for Android.

use sgs::native_main;

fn main() {
    native_main();
}
