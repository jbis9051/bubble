#[macro_export]
macro_rules! export {
    (
        $(
          $name: ident(
              $($arg: ident: $atype: ty),*
          ) -> Result<$rtype: ty, $err: ty>
        );*;
    ) => {
        pub fn dynamic_call(name_: &str, mut args_: Value, promise: $crate::platform::DevicePromise) -> Result<(),()> {
            match name_ {
                $(
                    stringify!($name) => convert_func!(args_, promise,
                        $name($($arg: $atype),*) -> Result<$rtype, $err>
                    ),
                )*
                _ => Err(())
            }
        }
    }
}

#[macro_export]
macro_rules! convert_func {
    ($args_:ident,$promise: ident, $name: ident($($arg: ident: $atype: ty),*) -> Result<$rtype: ty, $err: ty>) => {
        {
            $(let $arg: $atype = serde_json::from_value($args_[stringify!($arg)].take()).unwrap();)*
            $crate::GLOBAL_STATIC_DATA.get().unwrap().tokio.handle.spawn(
                $crate::promise::promisify::<$rtype, $err>(
                    $promise,
                    $crate::$name($($arg),*)
                )
            );
            Ok(())
        }
    };
}
