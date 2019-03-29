#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Union<A, B> {
    A(A),
    B(B)
}

// Unwrapping
impl<A, B> Union<A, B> {
    /// Moves into the [`A`](#variant.A) value, panicking if called on a [`B`](#variant.B)
    pub fn unwrap_a(self) -> A {
        match self {
            Union::A(x) => x,
            _ => panic!("Called Union::unwrap_a() on a B value")
        }
    }

    /// Moves into the [`B`](#variant.B) value, panicking if called on an [`A`](#variant.A)
    pub fn unwrap_b(self) -> B {
        match self {
            Union::B(x) => x,
            _ => panic!("Called Union::unwrap_b() on an A value")
        }
    }

    /// Returns a mutable reference to the [`A`](#variant.A) value, panicking if called on a [`B`](#variant.B)
    pub fn a_mut(&mut self) -> &mut A {
        match self {
            Union::A(x) => x,
            _ => panic!("Called Union::a_mut() on a B value")
        }
    }

    /// Returns a mutable reference to the [`B`](#variant.B) value, panicking if called on an [`A`](#variant.A)
    pub fn b_mut(&mut self) -> &mut B {
        match self {
            Union::B(x) => x,
            _ => panic!("Called Union::b_mut() on an A value")
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

    /// Returns an `Optiom<&A>`, mapping [`A(x)`](#variant.A) to `Some(&x)` and [`B(_)`](#variant.B) to `None`
    pub fn to_option_a(&self) -> Option<&A> {
        match self {
            Union::A(x) => Some(x),
            _ => None
        }
    }

    /// Returns an `Optiom<&B>`, mapping [`B(x)`](#variant.B) to `Some(&x)` and [`A(_)`](#variant.A) to `None`
    pub fn to_option_b(&self) -> Option<&B> {
        match self {
            Union::B(x) => Some(x),
            _ => None
        }
    }

    /// Returns an `Optiom<&A>`, mapping [`A(x)`](#variant.A) to `Some(x)` and [`B(_)`](#variant.B) to `None`
    pub fn into_option_a(self) -> Option<A> {
        match self {
            Union::A(x) => Some(x),
            _ => None
        }
    }

    /// Returns an `Optiom<&B>`, mapping [`B(x)`](#variant.B) to `Some(x)` and [`A(_)`](#variant.A) to `None`
    pub fn into_option_b(self) -> Option<B> {
        match self {
            Union::B(x) => Some(x),
            _ => None
        }
    }
}

// Other useful functions
impl<A, B> Union<A, B> {
    /// Applies `f` to the inside value if called on [`A`](#variant.A), does nothing if called on a [`B`](#variant.B)
    pub fn map_a<T, F: FnMut(A) -> T>(self, mut f: F) -> Union<T, B> {
        match self {
            Union::A(x) => Union::A(f(x)),
            Union::B(x) => Union::B(x)
        }
    }

    /// Applies `f` to the inside value if called on [`B`](#variant.B), does nothing if called on a [`A`](#variant.A)
    pub fn map_b<T, F: FnMut(B) -> T>(self, mut f: F) -> Union<A, T> {
        match self {
            Union::A(x) => Union::A(x),
            Union::B(x) => Union::B(f(x))
        }
    }

    /// Applies `f_a` to the inside value if called on [`A`](#variant.A), Applies `f_B` to the inside value if called on [`B`](#variant.B)
    pub fn map<T, U, F1: FnMut(A) -> T, F2: FnMut(B) -> U>(self, mut f_a: F1, mut f_b: F2) -> Union<T, U> {
        match self {
            Union::A(x) => Union::A(f_a(x)),
            Union::B(x) => Union::B(f_b(x))
        }
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

impl<A: Clone, B: Clone> Union<A, B> {
    /// Changes `Union<A, B>` to `Union<A, B>` by cloning the inner value
    /// Also works with `Union<&A, B>`, `Union<A, &B>` and `Union<&A, &B>`
    pub fn cloned(&self) -> Union<A, B> {
        match self {
            Union::A(x) => Union::A((*x).clone()),
            Union::B(x) => Union::B((*x).clone()),
        }
    }
}