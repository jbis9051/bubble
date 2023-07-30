pub trait NativeApi {
    type Error;

    fn init() -> Self;
    fn request_location_permissions(&self) -> Result<bool, Self::Error>;
    fn has_location_permissions(&self) -> Result<bool, Self::Error>;
    fn subscribe_to_location_updates(&self) -> Result<(), Self::Error>;
    fn unsubscribe_from_location_updates(&self) -> Result<(), Self::Error>;
}
