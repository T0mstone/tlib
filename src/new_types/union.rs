use std::convert::identity;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Union<A, B> {
    A(A),
    B(B)
}

impl<A, B> Union<A, B> {
    /// Moves into the [`A`](#variant.A) value, panicking if called on a [`B`](#variant.B)
    pub fn unwrap_a(self) -> A {
        match self {
            Union::A(x) => x,
            _ => panic!("Called Union::unwrap_a() on a Union::B value")
        }
    }

    /// Moves into the [`B`](#variant.B) value, panicking if called on an [`A`](#variant.A)
    pub fn unwrap_b(self) -> B {
        match self {
            Union::B(x) => x,
            _ => panic!("Called Union::unwrap_b() on a Union::A value")
        }
    }

    /// Converts from `Union<A, B>` to `Union<&A, &B>`
    pub fn as_ref(&self) -> Union<&A, &B> {
        match self {
            Union::A(a) => Union::A(a),
            Union::B(b) => Union::B(b),
        }
    }

    /// Converts from `Union<A, B>` to `Union<&mut A, &mut B>`
    pub fn as_mut(&mut self) -> Union<&mut A, &mut B> {
        match self {
            Union::A(a) => Union::A(a),
            Union::B(b) => Union::B(b),
        }
    }

    /// Returns true if called on [`A`](#variant.A) and false on [`B`](#variant.B)
    pub fn is_a(&self) -> bool {
        match self {
            Union::A(_) => true,
            _ => false
        }
    }

    /// Returns true if called on [`B`](#variant.B) and false on [`A`](#variant.A)
    pub fn is_b(&self) -> bool {
        !self.is_a()
    }

    /// Returns an `Optiom<A>`, mapping [`A(x)`](#variant.A) to `Some(x)` and [`B(_)`](#variant.B) to `None`
    pub fn some_a(self) -> Option<A> {
        match self {
            Union::A(x) => Some(x),
            _ => None
        }
    }

    /// Returns an `Optiom<B>`, mapping [`B(x)`](#variant.B) to `Some(x)` and [`A(_)`](#variant.A) to `None`
    pub fn some_b(self) -> Option<B> {
        match self {
            Union::B(x) => Some(x),
            _ => None
        }
    }

    /// Applies `f_a` to the inside value if called on [`A`](#variant.A), Applies `f_b` to the inside value if called on [`B`](#variant.B)
    pub fn map<T, U, F1: FnMut(A) -> T, F2: FnMut(B) -> U>(self, mut f_a: F1, mut f_b: F2) -> Union<T, U> {
        match self {
            Union::A(x) => Union::A(f_a(x)),
            Union::B(x) => Union::B(f_b(x))
        }
    }

    /// Applies `f` to the inside value if called on [`A`](#variant.A), does nothing if called on a [`B`](#variant.B)
    pub fn map_a<T, F: FnMut(A) -> T>(self, f: F) -> Union<T, B> {
        self.map(f, identity)
    }

    /// Applies `f` to the inside value if called on [`B`](#variant.B), does nothing if called on a [`A`](#variant.A)
    pub fn map_b<T, F: FnMut(B) -> T>(self, f: F) -> Union<A, T> {
        self.map(identity, f)
    }

    /// works like [`map`](#method.map) but does not consume `self`
    pub fn map_ref<T, U, F1: FnMut(&A) -> T, F2: FnMut(&B) -> U>(&self, mut f_a: F1, mut f_b: F2) -> Union<T, U> {
        match self {
            Union::A(x) => Union::A(f_a(x)),
            Union::B(x) => Union::B(f_b(x))
        }
    }

    /// works like [`map_a`](#method.map_a) but does not consume `self` and clones any [`B`](#variant.B) value
    pub fn map_ref_a<T, F: FnMut(&A) -> T>(&self, f: F) -> Union<T, B>
        where B: Clone
    {
        self.map_ref(f, |x| x.clone())
    }

    /// works like [`map_b`](#method.map_b) but does not consume `self` and clones any [`A`](#variant.A) value
    pub fn map_ref_b<T, F: FnMut(&B) -> T>(&self, f: F) -> Union<A, T>
        where A: Clone
    {
        self.map_ref(|x| x.clone(), f)
    }
}

impl<A> Union<A, A> {
    /// Collapses `Union<A, A>` into `A`
    pub fn collapse(self) -> A {
        match self {
            Union::A(x) => x,
            Union::B(x) => x
        }
    }
}