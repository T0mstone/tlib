pub trait Tuple {
    type Head;
    type Tail: Tuple;

    fn split_first(self) -> (Self::Head, Self::Tail);

    fn construct(head: Self::Head, tail: Self::Tail) -> Self;
}

impl Tuple for () {
    type Head = ();
    type Tail = ();

    fn split_first(self) -> (Self::Head, Self::Tail) {
        ((), ())
    }

    fn construct(_: Self::Head, _: Self::Tail) -> Self {
        ()
    }
}

impl<T> Tuple for (T,) {
    type Head = T;
    type Tail = ();

    fn split_first(self) -> (Self::Head, Self::Tail) {
        (self.0, ())
    }

    fn construct(h: Self::Head, _: Self::Tail) -> Self {
        (h,)
    }
}

impl<T, U> Tuple for (T, U) {
    type Head = T;
    type Tail = (U,);

    fn split_first(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1,))
    }

    fn construct(h: Self::Head, t: Self::Tail) -> Self {
        (h, t.0)
    }
}

mod base1;

#[cfg(feature = "tuple_tools_a2")]
#[allow(non_snake_case)]
mod A2;

#[cfg(feature = "tuple_tools_b2")]
#[allow(non_snake_case)]
mod B2;

#[cfg(feature = "tuple_tools_c2")]
#[allow(non_snake_case)]
mod C2;

#[cfg(feature = "tuple_tools_d2")]
#[allow(non_snake_case)]
mod D2;
