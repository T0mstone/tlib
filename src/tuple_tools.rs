/// A trait a lot of tuple types implement. Basically a substitution for variadic generics
pub trait Tuple<Aug> {
    /// The first type
    type Head;
    /// A tuple of the remaining types
    type Tail: Tuple<Self::Head>;
    /// The next higher tuple type
    type Augmented;

    /// The amount of items in the tuple
    const LEN: usize;

    /// split a tuple at the value level
    fn split_first(self) -> (Self::Head, Self::Tail);

    /// construct a tuple at the value level
    fn construct(head: Self::Head, tail: Self::Tail) -> Self;

    /// push a new head to the front of the tuple
    fn push_head(self, new_head: Aug) -> Self::Augmented
    where
        Self::Augmented: Tuple<Self::Head, Head = Aug, Tail = Self>,
        Self: Sized,
    {
        Self::Augmented::construct(new_head, self)
    }
}

impl<Aug> Tuple<Aug> for () {
    type Head = ();
    type Tail = ();
    type Augmented = (Aug,);

    const LEN: usize = 0;

    fn split_first(self) -> (Self::Head, Self::Tail) {
        ((), ())
    }

    fn construct(_: Self::Head, _: Self::Tail) -> Self {
        ()
    }
}

impl<A, Aug> Tuple<Aug> for (A,) {
    type Head = A;
    type Tail = ();
    type Augmented = (Aug, A);

    const LEN: usize = 1;

    fn split_first(self) -> (Self::Head, Self::Tail) {
        (self.0, ())
    }

    fn construct(h: Self::Head, _: Self::Tail) -> Self {
        (h,)
    }
}

impl<A, B, Aug> Tuple<Aug> for (A, B) {
    type Head = A;
    type Tail = (B,);
    type Augmented = (Aug, A, B);

    const LEN: usize = 2;

    fn split_first(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1,))
    }

    fn construct(h: Self::Head, t: Self::Tail) -> Self {
        (h, t.0)
    }
}

mod impl3to25;

#[cfg(feature = "tuple_tools_50")]
mod impl26to50;

#[cfg(feature = "tuple_tools_100")]
mod impl51to100;

#[cfg(feature = "tuple_tools_150")]
mod impl101to150;

#[cfg(feature = "tuple_tools_200")]
mod impl151to200;
