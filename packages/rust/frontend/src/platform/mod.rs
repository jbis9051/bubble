use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(target_os="ios")] {
        pub mod ios;

        pub use ios::export;
        pub use ios::DevicePromise;
    } else if #[cfg(target_os="android")] {
        pub mod android;

        pub use android::export;
        pub use android::DevicePromise;
    } else {
        pub mod default;

        pub use default::export;
        pub use default::DevicePromise;
    }
);
