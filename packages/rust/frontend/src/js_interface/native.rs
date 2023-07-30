use crate::js_interface::FrontendInstance;
use crate::public::native_api::NativeApi;
use bridge_macro::bridge;

impl FrontendInstance {
    #[bridge]
    pub async fn request_location_permissions(&self) -> Result<bool, ()> {
        self.device_api
            .request_location_permissions()
            .map_err(|_| ())
    }

    #[bridge]
    pub async fn has_location_permissions(&self) -> Result<bool, ()> {
        self.device_api.has_location_permissions().map_err(|_| ())
    }

    #[bridge]
    pub async fn subscribe_to_location_updates(&self) -> Result<(), ()> {
        self.device_api
            .subscribe_to_location_updates()
            .map_err(|_| ())
    }

    #[bridge]
    pub async fn unsubscribe_from_location_updates(&self) -> Result<(), ()> {
        self.device_api
            .unsubscribe_from_location_updates()
            .map_err(|_| ())
    }
}
