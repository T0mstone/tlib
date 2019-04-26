use std::convert::identity;

/// A type that can be one of two types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Either<A, B> {
    /// Moves into the [`A`](#variant.A) value, panicking if called on a [`B`](#variant.B)
    pub fn unwrap_a(self) -> A {
        match self {
            Either::A(x) => x,
            _ => panic!("Called Union::unwrap_a() on a Union::B value"),
        }
    }

    /// Moves into the [`B`](#variant.B) value, panicking if called on an [`A`](#variant.A)
    pub fn unwrap_b(self) -> B {
        match self {
            Either::B(x) => x,
            _ => panic!("Called Union::unwrap_b() on a Union::A value"),
        }
    }

    /// Converts from `Either<A, B>` to `Either<&A, &B>`
    pub fn as_ref(&self) -> Either<&A, &B> {
        match self {
            Either::A(a) => Either::A(a),
            Either::B(b) => Either::B(b),
        }
    }

    /// Converts from `Either<A, B>` to `Either<&mut A, &mut B>`
    pub fn as_mut(&mut self) -> Either<&mut A, &mut B> {
        match self {
            Either::A(a) => Either::A(a),
            Either::B(b) => Either::B(b),
        }
    }

    /// Returns true if called on [`A`](#variant.A) and false on [`B`](#variant.B)
    pub fn is_a(&self) -> bool {
        match self {
            Either::A(_) => true,
            _ => false,
        }
    }

    /// Returns true if called on [`B`](#variant.B) and false on [`A`](#variant.A)
    pub fn is_b(&self) -> bool {
        !self.is_a()
    }

    /// Returns an `Option<A>`, mapping [`A(x)`](#variant.A) to `Some(x)` and [`B(_)`](#variant.B) to `None`
    pub fn some_a(self) -> Option<A> {
        match self {
            Either::A(x) => Some(x),
            _ => None,
        }
    }

    /// Returns an `Option<B>`, mapping [`B(x)`](#variant.B) to `Some(x)` and [`A(_)`](#variant.A) to `None`
    pub fn some_b(self) -> Option<B> {
        match self {
            Either::B(x) => Some(x),
            _ => None,
        }
    }

    /// Applies `f_a` to the inside value if called on [`A`](#variant.A), Applies `f_b` to the inside value if called on [`B`](#variant.B)
    pub fn map<T, U, F1: FnMut(A) -> T, F2: FnMut(B) -> U>(
        self,
        mut f_a: F1,
        mut f_b: F2,
    ) -> Either<T, U> {
        match self {
            Either::A(x) => Either::A(f_a(x)),
            Either::B(x) => Either::B(f_b(x)),
        }
    }

    /// Applies `f` to the inside value if called on [`A`](#variant.A), does nothing if called on a [`B`](#variant.B)
    pub fn map_a<T, F: FnMut(A) -> T>(self, f: F) -> Either<T, B> {
        self.map(f, identity)
    }

    /// Applies `f` to the inside value if called on [`B`](#variant.B), does nothing if called on a [`A`](#variant.A)
    pub fn map_b<T, F: FnMut(B) -> T>(self, f: F) -> Either<A, T> {
        self.map(identity, f)
    }

    /// works like [`map`](#method.map) but does not consume `self`
    pub fn map_ref<T, U, F1: FnMut(&A) -> T, F2: FnMut(&B) -> U>(
        &self,
        mut f_a: F1,
        mut f_b: F2,
    ) -> Either<T, U> {
        match self {
            Either::A(x) => Either::A(f_a(x)),
            Either::B(x) => Either::B(f_b(x)),
        }
    }

    /// works like [`map_a`](#method.map_a) but does not consume `self` and clones any [`B`](#variant.B) value
    pub fn map_ref_a<T, F: FnMut(&A) -> T>(&self, f: F) -> Either<T, B>
    where
        B: Clone,
    {
        self.map_ref(f, |x| x.clone())
    }

    /// works like [`map_b`](#method.map_b) but does not consume `self` and clones any [`A`](#variant.A) value
    pub fn map_ref_b<T, F: FnMut(&B) -> T>(&self, f: F) -> Either<A, T>
    where
        A: Clone,
    {
        self.map_ref(|x| x.clone(), f)
    }
}

impl<A> Either<A, A> {
    /// Collapses `Either<A, A>` into `A`
    pub fn collapse(self) -> A {
        match self {
            Either::A(x) => x,
            Either::B(x) => x,
        }
    }
}
