/// A type that can only be initialized once
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Singleton<T> {
    inner: Option<T>
}

impl<T> Singleton<T> {
    pub const fn new() -> Self {
        Self {
            inner: None
        }
    }

    fn assert_init(&self, funcname: &str) {
        if !self.is_initialized() {
            panic!("called `Singleton::{}()` but Singleton is not initialized", funcname);
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
            _ => false
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.inner.is_some()
    }

    /// Converts `Singleton<T>` to `Option<T>`, using `Clone`
    pub fn to_option(&self) -> Option<T>
        where T: Clone
    {
        self.inner.clone()
    }

    /// Converts the `Singleton<T>` to an `Option<T>`
    pub fn into_option(self) -> Option<T> {
        self.inner
    }

    /// Moves the value v out of the `Singleton<T>` if it is initialized.
    /// # Panics
    ///
    /// Panics if the value is not initialized.
    pub fn unwrap(self) -> T {
        self.assert_init("unwrap");
        //          ↓↓↓ this cannot fail
        self.inner.unwrap()
    }

    /// Returns a reference to the value v of the `Singleton<T>` if it is initialized.
    /// # Panics
    ///
    /// Panics if the value is not initialized.
    pub fn inner(&self) -> &T {
        self.assert_init("inner");
        match self.inner {
            Some(ref v) => v,
            _ => unreachable!()
        }
    }

    /// Returns a mutable reference to the value v of the `Singleton<T>` if it is initialized.
    /// # Panics
    ///
    /// Panics if the value is not initialized.
    pub fn inner_mut(&mut self) -> &mut T {
        self.assert_init("inner_mut");
        match self.inner {
            Some(ref mut v) => v,
            _ => unreachable!()
        }
    }

    /// works analogous to [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
    pub fn map<F: FnMut(T) -> U, U>(self, f: F) -> Singleton<U> {
        Singleton { inner: self.inner.map(f) }
    }
}