use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(target_os="ios")] {
        pub mod ios;

        pub use ios::export;
        pub use ios::DevicePromise;
        pub use ios::DeviceApi;
    } else if #[cfg(target_os="android")] {
        pub mod android;

        pub use android::export;
        pub use android::DevicePromise;
        pub use android::DeviceApi;
    } else {
        pub mod default;

        pub use default::export;
        pub use default::DeviceApi;
        pub use default::DevicePromise;
    }
);

pub fn get_default_domain() -> &'static str {
    #[cfg(all(feature = "development", feature = "staging"))]
    compile_error!("development and staging features cannot both be set");
    cfg_if!(
        if #[cfg(feature = "development")]{
            cfg_if!(
                if #[cfg(target_os="android")]{
                    return "http://10.0.2.2:3000";
                } else {
                    return "http://localhost:3000";
                }
            );
        } else if #[cfg(feature = "staging")]{
            return "https://api.staging.bubble.place";
        } else { // release
            return "https://api.bubble.place";
        }
    );
}
