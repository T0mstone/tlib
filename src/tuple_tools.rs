pub trait Tuple<Aug> {
    type Head;
    type Tail: Tuple<Self::Head>;
    type Augmented;

    fn split_first(self) -> (Self::Head, Self::Tail);

    fn construct(head: Self::Head, tail: Self::Tail) -> Self;

    fn push_head(self, new_head: Aug) -> Self::Augmented
    where
        Self::Augmented: Tuple<Self::Head, Head = Aug, Tail = Self>,
        Self: Sized,
    {
        Self::Augmented::construct(new_head, self)
    }

    fn len() -> usize;
}

pub trait TupleTypeUnion {
    type Union;
}

impl TupleTypeUnion for ((), ()) {
    type Union = ();
}

impl<A1> TupleTypeUnion for ((A1,), ()) {
    type Union = (A1,);
}

impl<A1, A2> TupleTypeUnion for ((A1,), (A2,)) {
    type Union = (A1, A2);
}

impl<A1, B1, A2> TupleTypeUnion for ((A1, B1), (A2,)) {
    type Union = (A1, B1, A2);
}

impl<Aug> Tuple<Aug> for () {
    type Head = ();
    type Tail = ();
    type Augmented = (Aug,);

    fn split_first(self) -> (Self::Head, Self::Tail) {
        ((), ())
    }

    fn construct(_: Self::Head, _: Self::Tail) -> Self {
        ()
    }

    fn len() -> usize {
        0
    }
}

impl<A, Aug> Tuple<Aug> for (A,) {
    type Head = A;
    type Tail = ();
    type Augmented = (Aug, A);

    fn split_first(self) -> (Self::Head, Self::Tail) {
        (self.0, ())
    }

    fn construct(h: Self::Head, _: Self::Tail) -> Self {
        (h,)
    }

    fn len() -> usize {
        1
    }
}

impl<A, B, Aug> Tuple<Aug> for (A, B) {
    type Head = A;
    type Tail = (B,);
    type Augmented = (Aug, A, B);

    fn split_first(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1,))
    }

    fn construct(h: Self::Head, t: Self::Tail) -> Self {
        (h, t.0)
    }

    fn len() -> usize {
        2
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
