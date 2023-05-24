#[macro_export]
macro_rules! export {
    (
        $(
          $name: ident(
              $($arg: ident: $atype: ty),*
          )
        ),*,
    ) => {
        pub fn dynamic_call(name_: &str, mut params_: Value, tokio_thread: &once_cell::sync::OnceCell<TokioThread>) -> Result<(),()> {
            match name_ {
                $(
                    stringify!($name) => {
                        $(let $arg: $atype = serde_json::from_value(params_[stringify!($arg)].take()).unwrap();)*
                        tokio_thread.get().unwrap().handle.spawn(async move {
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
