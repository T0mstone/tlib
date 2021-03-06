#[cfg(not(feature = "use_std"))]
use core as std;

use std::ops::{Deref, DerefMut};

/// A type that can only be initialized once
///
/// Intended for use with `static mut` and with types you only ever need once (e.g. a Context or a Logger)
///
/// # Example
/// ```no_run
/// # use tlib::Singleton;
/// # struct Logger;
/// # impl Logger { pub fn new() -> Self { Logger } }
/// static mut LOGGER: Singleton<Logger> = Singleton::new();
///
/// pub fn init() {
///     unsafe {
///         LOGGER.init(Logger::new());
///     }
/// }
///
/// // ...
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Singleton<T> {
    inner: Option<T>,
}

impl<T> Deref for Singleton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        if !self.is_initialized() {
            panic!("tried to dereference an uninitialized Singleton");
        }
        self.as_ref().unwrap()
    }
}

impl<T> DerefMut for Singleton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.is_initialized() {
            panic!("tried to dereference an uninitialized Singleton");
        }
        self.as_mut().unwrap()
    }
}

impl<T> Singleton<T> {
    /// Creates a new, uninitialized singleton
    pub const fn new() -> Self {
        Self { inner: None }
    }

    fn assert_init(&self, funcname: &str) {
        if !self.is_initialized() {
            panic!(
                "called `Singleton::{}()` but Singleton is not initialized",
                funcname
            );
        }
    }

    /// Stores the value in the `Singleton<T>` if it is not already initialized,
    ///
    /// does nothing if it is.
    ///
    /// Returns whether the init was successful
    pub fn init(&mut self, t: T) -> bool {
        match self.inner {
            None => {
                self.inner = Some(t);
                true
            }
            _ => false,
        }
    }

    /// Returns whether the singleton is initialized
    pub fn is_initialized(&self) -> bool {
        self.inner.is_some()
    }

    /// Converts `Singleton<T>` to `Option<T>`, using `Clone`
    pub fn to_option(&self) -> Option<T>
    where
        T: Clone,
    {
        self.inner.clone()
    }

    /// Converts the `Singleton<T>` to an `Option<T>`
    pub fn into_option(self) -> Option<T> {
        self.inner
    }

    /// Moves the value v out of the `Singleton<T>` if it is initialized.
    ///
    /// # Panics
    ///
    /// Panics if the value is not initialized.
    pub fn unwrap(self) -> T {
        self.assert_init("unwrap");
        self.inner.unwrap()
    }

    /// Moves the value v out of the `Singleton<T>` if it is initialized.
    ///
    /// # Panics
    ///
    /// Panics if the value is not initialized with a custom panic message provided by `msg`.
    pub fn expect(self, msg: &str) -> T {
        if !self.is_initialized() {
            panic!("{}", msg);
        }
        self.inner.unwrap()
    }

    /// Returns a reference to the value v of the `Singleton<T>` if it is initialized.
    ///
    /// # Panics
    ///
    /// Panics if the value is not initialized.
    pub fn inner(&self) -> &T {
        self.assert_init("inner");
        self.inner.as_ref().unwrap()
    }

    /// Returns a mutable reference to the value v of the `Singleton<T>` if it is initialized.
    /// # Panics
    ///
    /// Panics if the value is not initialized.
    pub fn inner_mut(&mut self) -> &mut T {
        self.assert_init("inner_mut");
        self.inner.as_mut().unwrap()
    }

    /// Converts from Singleton<T> to Singleton<&T>
    pub fn as_ref(&self) -> Singleton<&T> {
        Singleton {
            inner: self.inner.as_ref(),
        }
    }

    /// Converts from Singleton<T> to Singleton<&mut T>
    pub fn as_mut(&mut self) -> Singleton<&mut T> {
        Singleton {
            inner: self.inner.as_mut(),
        }
    }

    /// works analogous to [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
    pub fn map<F: FnMut(T) -> U, U>(self, f: F) -> Singleton<U> {
        Singleton {
            inner: self.inner.map(f),
        }
    }

    /// works like [`map`](#method.map) but modifies `self` instead of returning a new `Singleton`.
    pub fn map_in_place<F: FnMut(T) -> T>(&mut self, mut f: F) {
        let inner = match self.inner.take() {
            Some(t) => Some(f(t)),
            None => None,
        };
        self.inner = inner;
    }
}

impl<T: Clone> Singleton<&T> {
    /// Maps a `Singleton<&T>` to a `Singleton<T>` by cloning the contents of the singleton.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn cloned(&self) -> Singleton<T> {
        Singleton {
            inner: self.inner.cloned(),
        }
    }
}

impl<T: Copy> Singleton<&T> {
    /// Maps a `Singleton<&T>` to a `Singleton<T>` by copying the contents of the singleton.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn copied(&self) -> Singleton<T> {
        Singleton {
            inner: self.inner.copied(),
        }
    }
}
