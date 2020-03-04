/// Constructs a HashMap.
///
/// Example:
/// ```
///
/// # use tlib::hashmap;
/// # use std::collections::HashMap;
/// let hm = hashmap!(
///     0 => 1,
///     1 => 2,
///     2 => 3
/// );
/// let mut control = HashMap::new();
/// control.insert(0, 1);
/// control.insert(1, 2);
/// control.insert(2, 3);
/// assert_eq!(hm, control);
/// ```
#[cfg(not(feature = "no_std"))]
#[macro_export]
macro_rules! hashmap(
    ( $($key:expr => $value:expr),* ) => {
        {
            let n = [$($key),*].len();
            let mut m = std::collections::HashMap::with_capacity(n);
            $(
                m.insert($key, $value);
            )*
            m
        }
     };
);
//
// /// Allows you pull the version for an from your Cargo.toml as MAJOR.MINOR.PATCH_PKGVERSION_PRE
// #[macro_export]
// macro_rules! crate_version {
//     () => {
//         #[cfg(feature = "no_std")]
//         use core as std;
//         std::format!(
//             "{}.{}.{}{}",
//             std::env!("CARGO_PKG_VERSION_MAJOR"),
//             std::env!("CARGO_PKG_VERSION_MINOR"),
//             std::env!("CARGO_PKG_VERSION_PATCH"),
//             std::option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
//         )
//     };
// }
