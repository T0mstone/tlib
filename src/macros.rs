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

/// Constructs a HashMap,
/// calling [`into`](https://doc.rust-lang.org/std/convert/trait.Into.html#tymethod.into)
/// on the keys and values
///
/// Example:
/// ```
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
///
/// Optionally, you can start with the wanted type parameters
/// ```
/// # use tlib::hashmap;
/// # use std::collections::HashMap;
/// let hm = hashmap!(use HashMap<i32, _>;
///     0u8 => 1,
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
macro_rules! hashmap {
    ( $(use HashMap<$tk:ty, $tv:ty>;)? $($key:expr => $value:expr),* ) => {
        {
            let mut m = std::collections::HashMap$(::<$tk, $tv>)?::with_capacity($crate::count_args!($(($key)),*));
            $(
                m.insert(std::convert::Into::into($key), std::convert::Into::into($value));
            )*
            m
        }
    };
}

/// Constructs a HashSet,
/// calling [`into`](https://doc.rust-lang.org/std/convert/trait.Into.html#tymethod.into)
/// on the values
///
/// Example:
/// ```
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
///
/// Optionally, you can start with the wanted type parameters
/// ```
/// # use tlib::hashset;
/// # use std::collections::HashSet;
/// let hm = hashset!(use HashSet<i32>;
///     0u8,
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
macro_rules! hashset {
    ( $(use HashSet<$t:ty>;)? $($e:expr),* ) => {
        {
            let mut m = std::collections::HashSet$(::<$t>)?::with_capacity($crate::count_args!($(($e)),*));
            $(
                m.insert(std::convert::Into::into($e));
            )*
            m
        }
     };
}

/// A macro to print pretty looking and informative debug logs.
///
/// ## Difference from `dbg!`
/// There are three differences:
/// 1. `dbgr!` takes its arguments by ref (like `println!`) so it can be called at any point without worrying about ownership
/// 2. `dbgr!` has more functionality
///
/// ## Motivation
/// The intended use case for `dbgr!` is a different one than that of `dbg!`.\
/// Where `dbg!` aims to let you put it in the middle of function calls (like `f(dbg!(a), b, c)`),
/// `dbgr!` aims to be more like `println!`.
///
/// ## Syntax
/// `dbgr!` takes zero or more expressions.
///
/// ```ignore
/// dbgr!();     // prints "[<file>] reached <line>:<column>."
/// dbgr!(a);    // prints "[<file>@<line>] a = <a, debug printed>"
/// dbgr!(a, b); // prints "[<file>@<line>] a = <a, debug printed>, b = <b, debug printed>"
/// dbgr!(#; a); // prints "[<file>@<line>] a = <a, debug pretty printed>"
/// dbgr!(<any literal>); // prints "[<file>@<line>] <the literal, debug printed>" (only works with a single literal)
/// dbgr!(# <any literal>); // prints "[<file>@<line>] <the literal, debug pretty printed>" (only works with a single literal)
/// dbgr!(: <any literal>); // prints "[<file>@<line>] <the literal, display printed>" (only works with a single literal)
/// ```
#[cfg(feature = "use_std")]
#[macro_export]
macro_rules! dbgr {
    () => {
        println!(
            concat!(
                "[",
                file!(),
                "] reached ",
                line!(),
                ":",
                column!(),
                "."
            ),
        )
    };
    ($l:literal) => {
        println!(
            concat!(
                "[",
                file!(),
                "@",
                line!(),
                "] {:?}",
            ),
            $l
        )
    };
    (# $l:literal) => {
        println!(
            concat!(
                "[",
                file!(),
                "@",
                line!(),
                "] {:#?}",
            ),
            $l
        )
    };
    (: $l:literal) => {
        println!(
            concat!(
                "[",
                file!(),
                "@",
                line!(),
                "] {}",
            ),
            $l
        )
    };
    ($($e:expr),+) => {
        println!(
            concat!(
                "[",
                file!(),
                "@",
                line!(),
                "] "
                $(
                , stringify!($e),
                " = {:?}",
                )", "+
            ),
            $($e),+
        )
    };
    (#; $($e:expr),+) => {
        println!(
            concat!(
                "[",
                file!(),
                "@",
                line!(),
                "] "
                $(
                , stringify!($e),
                " = {:#?}",
                )", "+
            ),
            $($e),+
        )
    };
}

/// Returns a boolean that tells you whether a specific environment variable had a certain value at compiletime
///
/// ## Motivation
/// This is useful for when you want to easily switch debug code on and off with wnvironment variables.
/// Instead of writing the following code snippet every time
/// ```ignore
/// match option_env!(LIT) {
///     Some(s) => {
///         s.parse::<i64>().map_or(false, |lvl| lvl >= MIN)
///     }
///     None => false,
/// }
/// ```
/// you can just write `debug_lvl!(LIT >= MIN)`
/// This works with all comparison operators (`==`, `!=`, `>`, `<`, `>=`, `<=`)
#[macro_export]
macro_rules! debug_lvl {
    (@tok >= $a:expr, $b:expr) => {
        $a >= $b
    };
    (@tok <= $a:expr, $b:expr) => {
        $a <= $b
    };
    (@tok == $a:expr, $b:expr) => {
        $a == $b
    };
    (@tok != $a:expr, $b:expr) => {
        $a != $b
    };
    (@tok > $a:expr, $b:expr) => {
        $a > $b
    };
    (@tok < $a:expr, $b:expr) => {
        $a < $b
    };
    (@tok $t:tt $a:expr, $b:expr) => {
        compile_error!(concat!("Invalid Token: ", stringify!($t), " (expected a comparison operator)"))
    };

    ($debug_env:literal $t:tt $min:literal) => {{
        const MIN: i64 = $min;
        match ::core::option_env!($debug_env).and_then(|s| s.parse::<i64>().ok()) {
            Some(lvl) => $crate::debug_lvl!(@tok $t lvl, MIN),
            None => false,
        }
    }};
}
