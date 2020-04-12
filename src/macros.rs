/// Counts how many arguments it receives.
///
/// Example:
/// ```
/// # use tlib::count_args;
/// let n = count_args!(
///     (2), (3), (5)
/// );
/// assert_eq!(n, 3);
/// ```
#[macro_export]
macro_rules! count_args {
    (@single $($t:tt)*) => { () };
    ($(($($x:tt)*)),*) => {
        [$($crate::count_args!(@single $($x)*)),*].len()
    };
}

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
#[cfg(feature = "use_std")]
#[macro_export]
macro_rules! hashmap(
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut m = std::collections::HashMap::with_capacity($crate::count_args!($(($key)),*));
            $(
                m.insert($key, $value);
            )*
            m
        }
     };
);

/// Constructs a HashSet.
///
/// Example:
/// ```
///
/// # use tlib::hashset;
/// # use std::collections::HashSet;
/// let hm = hashset!(
///     0,
///     1,
///     2
/// );
/// let mut control = HashSet::new();
/// control.insert(0);
/// control.insert(1);
/// control.insert(2);
/// assert_eq!(hm, control);
/// ```
#[cfg(feature = "use_std")]
#[macro_export]
macro_rules! hashset(
    ( $($e:expr),* ) => {
        {
            let mut m = std::collections::HashSet::with_capacity($crate::count_args!($(($e)),*));
            $(
                m.insert($e);
            )*
            m
        }
     };
);
