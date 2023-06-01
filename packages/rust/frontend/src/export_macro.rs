#[macro_export]
macro_rules! export {
    (
        $(
          $name: ident(
              $($arg: ident: $atype: ty),*
          )
        ),*,
    ) => {
        pub fn dynamic_call(name_: &str, mut params_: Value) -> Result<(),()> {
            match name_ {
                $(
                    stringify!($name) => {
                        $(let $arg: $atype = serde_json::from_value(params_[stringify!($arg)].take()).unwrap();)*
                        $crate::GLOBAL_STATIC_DATA.get().unwrap().tokio.handle.spawn(async move {
                            $name($($arg),*).await;
                        });
                        Ok(())
                    }
                ),*
                _ => Err(())
            }
        }
    }
}
