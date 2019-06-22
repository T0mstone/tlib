use self::util::*;
use std::fs;
use std::path::PathBuf;

mod util {
    pub trait IterJoin<T>: Iterator<Item = T> + Sized
    where
        T: Clone,
    {
        fn concat(a: T, b: T) -> T;

        fn concat_3(a: T, b: T, c: T) -> T {
            let tmp = Self::concat(a, b);
            Self::concat(tmp, c)
        }

        fn try_join<U: Into<T>>(mut self, delim: U) -> Option<T> {
            let delim = delim.into();
            let first = self.next()?;
            let val = self.fold(first, |acc, x| Self::concat_3(acc, delim.clone(), x));
            Some(val)
        }

        fn join<U: Into<T>>(mut self, delim: U) -> T
        where
            T: Default,
        {
            self.try_join(delim).unwrap_or_default()
        }
    }

    impl<I: Iterator<Item = String>> IterJoin<String> for I {
        fn concat(_: String, _: String) -> String {
            unimplemented!()
        }

        #[inline]
        fn concat_3(a: String, b: String, c: String) -> String {
            a + &b[..] + &c[..]
        }
    }

    impl<T: Clone, I: Iterator<Item = Vec<T>>> IterJoin<Vec<T>> for I {
        fn concat(_: Vec<T>, _: Vec<T>) -> Vec<T> {
            unimplemented!()
        }

        #[inline]
        fn concat_3(mut a: Vec<T>, b: Vec<T>, c: Vec<T>) -> Vec<T> {
            a.extend(b);
            a.extend(c);
            a
        }
    }

    fn letter(i: usize) -> char {
        const ALPH: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        ALPH.chars().nth(i).unwrap()
    }

    pub fn encode_base26(i: usize) -> String {
        let low = i % 26;
        let high = i / 26;
        if high == 0 {
            letter(low).to_string()
        } else {
            let mut res = encode_base26(high - 1);
            res.push(letter(low));
            res
        }
    }

    #[derive(Clone)]
    pub struct IdentIter {
        i: usize,
    }

    impl IdentIter {
        pub const fn new() -> Self {
            Self { i: 0 }
        }
    }

    impl Iterator for IdentIter {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            self.i += 1;
            Some(encode_base26(self.i - 1))
        }
    }

    pub struct Accum<I: IntoIterator> {
        iter: I,
        len: usize,
    }

    impl<I: IntoIterator> Accum<I> {
        pub fn new(iter: I, len: usize) -> Self {
            Self { iter, len }
        }
    }

    impl<I: IntoIterator + Clone> Iterator for Accum<I> {
        type Item = Vec<I::Item>;

        fn next(&mut self) -> Option<Vec<I::Item>> {
            let iter = self.iter.clone().into_iter();
            let res: Vec<_> = iter.take(self.len).collect();

            self.len += 1;

            if res.len() + 1 != self.len {
                None
            } else {
                Some(res)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let i = &self.iter.clone().into_iter() as &dyn Iterator<Item = I::Item>;
            i.size_hint()
        }
    }
}

fn make_impl(types: &[String]) -> String {
    let head = types.get(0).expect("Vec too short");
    let tail = types.get(1..).expect("Vec too short");
    let tail_cons = (1..types.len()).map(|i| format!("self.{}", i)).join(",");
    let tail_cons_2 = (0..tail.len()).map(|i| format!("t.{}", i)).join(",");
    let len = types.len();

    let types_s = types.join(",");
    let tail_s = tail.join(",");

    format!(
        "impl<{}, Aug> Tuple<Aug> for ({0}) {{\
         type  Head = {};\
         type  Tail = ({});\
         type  Augmented = (Aug, {0});\
         fn  split_first(self) -> (Self::Head, Self::Tail) {{ (self.0, ({})) }}\
         fn  construct(h: Self::Head, t: Self::Tail) -> Self {{ (h, {}) }}\
         fn  len() -> usize {{ {} }}\
         }}",
        types_s, head, tail_s, tail_cons, tail_cons_2, len
    )
    // dummy character (keep double space)
    .replace("  ", "§")
    // remove spaces
    .replace(" ", "")
    // insert double spaces back (now as single spaces)
    .replace("§", " ")
}

fn make_filepath(from: usize, to: usize) -> PathBuf {
    PathBuf::from(format!("src/tuple_tools/impl{}to{}.rs", from, to))
}

fn make_range(from: usize, len: usize) -> String {
    let iter = Accum::new(IdentIter::new(), 0).skip(from).take(len);
    iter.map(|types| make_impl(&types)).join("\n")
}

fn write_file(content: String, path: PathBuf) {
    let content = format!("use crate::tuple_tools::Tuple;\n{}", content);
    fs::write(path, content).expect("Failed to create file");
}

fn make_file(start: usize, len: usize) {
    let content = make_range(start, len);
    let path = make_filepath(start, start + len - 1);
    write_file(content, path);
}

fn main() {
    make_file(3, 23);
    make_file(26, 25);
    make_file(51, 50);
    make_file(101, 50);
    make_file(151, 50);
}
