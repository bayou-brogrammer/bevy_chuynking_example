#[macro_export]
macro_rules! switch_in_game_state {
    ($e:expr) => {
        |mut commands: Commands| {
            commands.insert_resource(NextState($e));
        }
    };
}

#[macro_export]
macro_rules! rm_resource {
    ($r:ident) => {
        |mut commands: Commands| {
            commands.remove_resource::<$r>();
        }
    };
}

#[macro_export]
macro_rules! impl_new{
  ($to:ty,$($v:ident: $t:ty),*)  => {
      impl $to {
          pub fn new($($v: $t),*) -> $to
          {
              Self {
                  $($v),*
              }
          }
      }
  };
}

#[macro_export]
macro_rules! impl_default {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self { Self::new() }
        }
    };
}
