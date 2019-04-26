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
#[macro_export]
macro_rules! hashmap(
    { $($key:expr => $value:expr),* } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
     };
);

/// Allows you pull the version for an from your Cargo.toml as MAJOR.MINOR.PATCH_PKGVERSION_PRE
#[macro_export]
macro_rules! crate_version {
    () => {
        format!(
            "{}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
        )
    };
}

/// Simplifies conditional compilation
/// # Example
/// ```no_run
/// # use tlib::cfg_switch;
/// cfg_switch! {
///     case unix: {
///         // a
///     }
///     case target_os = "windows": {
///         // b
///     }
///     case feature = "foo": {
///         // c
///     }
///     default: {
///         // d
///     }
/// }
/// ```
///
/// expands to
///
/// ```no_run
/// #[cfg(unix)]
/// {
///     // a
/// }
/// #[cfg(target_os = "windows")]
/// {
///     // b
/// }
/// #[cfg(feature = "foo")]
/// {
///     // c
/// }
/// #[cfg(not(any(
///     unix,
///     target_os = "windows",
///     feature  = "foo"
/// )))]
/// {
///     // d
/// }
/// ```
///
#[macro_export]
macro_rules! cfg_switch {
    (default: $def:block) => { $def };
    ($(case $a:meta: $b:block)+ default: $def:block) => {
        $(
            #[cfg($a)]
            $b
        )+
        #[cfg(not(any($($a),+)))]
        $def
    };
}
