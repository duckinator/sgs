//#![cfg(not(target_os = "android"))] // Don't compile this file for Android.

use sgs::native_main;

#[cfg(not(target_os = "android"))]
fn main() {
    native_main();
}


#[cfg(target_os = "android")]
fn main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let native_options = NativeOptions {
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    common_main(native_options);
}
