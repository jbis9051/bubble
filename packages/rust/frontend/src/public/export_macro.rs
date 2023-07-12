#[macro_export]
macro_rules! export {
    (
        $class: ty,
        $(
          $name: ident(
              $($arg: ident: $atype: ty),*
          ) -> Result<$rtype: ty, $err: ty>
        );*;
    ) =>
    {
        pub fn dynamic_call(instance: std::sync::Arc<FrontendInstance>, name_: &str, mut args_: serde_json::Value, promise: $crate::platform::DevicePromise) -> Result<(),()> {
            match name_ {
                $(
                    stringify!($name) => $crate::convert_func!($class, instance, args_, promise,
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
    ($class: ty, $instance: ident, $args_:ident,$promise: ident, $name: ident($($arg: ident: $atype: ty),*) -> Result<$rtype: ty, $err: ty>) => {
        {
            $(let $arg: $atype = serde_json::from_value($args_[stringify!($arg)].take()).unwrap();)*
            let handle = $instance.static_data.tokio.handle.clone();
            handle.spawn(
                $crate::public::promise::promisify::<$rtype, $err>(
                    $promise, async move {
                        <$class>::$name(&$instance, $($arg),*).await
                    }
                )
            );
            Ok(())
        }
    };
}
