use self::util::*;
use std::fs;
use std::path::PathBuf;

mod util {
    use once_cell::sync::Lazy;
    use std::borrow::Cow;
    use std::ops::{Bound, RangeBounds};

    pub trait IterJoin<T>: Iterator<Item = T> + Sized
    where
        T: Clone,
    {
        fn concat(a: T, b: T) -> T;

        fn concat_3(a: T, b: T, c: T) -> T {
            let tmp = Self::concat(a, b);
            Self::concat(tmp, c)
        }

        fn join<U: Into<T>>(mut self, delim: U) -> Option<T> {
            let delim = delim.into();
            let first = self.next()?;
            let val = self.fold(first, |acc, x| Self::concat_3(acc, delim.clone(), x));
            Some(val)
        }
    }

    impl<I: Iterator<Item = String>> IterJoin<String> for I {
        fn concat(a: String, b: String) -> String {
            a + &b[..]
        }
    }

    impl<T: Clone, I: Iterator<Item = Vec<T>>> IterJoin<Vec<T>> for I {
        fn concat(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
            a.extend(b);
            a
        }
    }

    pub trait MapBound: Sized {
        type FInp;
        fn map<U>(self, f: impl FnOnce(Self::FInp) -> U) -> Bound<U>;
    }

    impl<T> MapBound for Bound<T> {
        type FInp = T;

        fn map<U>(self, f: impl FnOnce(T) -> U) -> Bound<U> {
            match self {
                Bound::Included(t) => Bound::Included(f(t)),
                Bound::Excluded(t) => Bound::Excluded(f(t)),
                Bound::Unbounded => Bound::Unbounded,
            }
        }
    }

    pub fn char_range<R: RangeBounds<char>>(cr: R) -> Vec<char> {
        let b1 = cr.start_bound().map(|&c| c as u8);
        let b2 = cr.end_bound().map(|&c| c as u8);
        let u1 = match b1 {
            Bound::Unbounded => 0,
            Bound::Included(u) => u,
            Bound::Excluded(u) => u + 1,
        };
        let u2 = match b2 {
            Bound::Unbounded => u8::max_value(),
            Bound::Included(u) => u,
            Bound::Excluded(u) if u != 0 => u - 1,
            Bound::Excluded(0) => panic!("impossible bound"),
            _ => unreachable!(),
        };
        (u1..=u2).map(|u| u as char).collect()
    }

    pub static ALPH: Lazy<Cow<[char]>> = Lazy::new(|| Cow::from(char_range('A'..='Z')));

    pub fn char_concat(c1: char, c2: char) -> String {
        String::from_utf8(vec![c1 as u8, c2 as u8]).unwrap()
    }
}

fn make_impl(types: &[String]) -> String {
    let head = types.get(0).expect("Vec too short");
    let tail = types.get(1..).expect("Vec too short");
    let tail_cons = (1..types.len())
        .map(|i| format!("self.{}", i))
        .join(",")
        .unwrap_or(String::new());
    let tail_cons_2 = (0..tail.len())
        .map(|i| format!("t.{}", i))
        .join(",")
        .unwrap_or(String::new());

    let types_s = types.join(",");
    let tail_s = tail.join(",");

    format!(
        "impl<{0}> Tuple for ({0}) {{\
         type Head = {1};\
         type Tail = ({2});\
         fn split_first(self) -> (Self::Head, Self::Tail) {{ (self.0, ({3})) }}\
         fn construct(h: Self::Head, t: Self::Tail) -> Self {{ (h, {4}) }}\
         }}",
        types_s, head, tail_s, tail_cons, tail_cons_2
    )
}

fn make_1char_all() -> String {
    (3..=ALPH.len())
        .map(|i| {
            let slice: Vec<_> = ALPH[..i].iter().map(|c| c.to_string()).collect();
            make_impl(&slice)
        })
        .join("\n")
        .unwrap_or(String::new())
}

fn make_2char_types(c1: char) -> (Vec<String>, usize) {
    let mut res: Vec<String> = ALPH.iter().map(|c| c.to_string()).collect();
    // c1 is excluded
    for c1t in char_range('A'..c1) {
        let aug_alph = ALPH.iter().map(|c| char_concat(c1t, *c));
        res.extend(aug_alph);
    }
    let first_len = res.len() + 1;
    let aug_alph = ALPH.iter().map(|c| char_concat(c1, *c));
    res.extend(aug_alph);
    (res, first_len)
}

fn make_2char_all(letter: char) -> String {
    let (types, first_len) = make_2char_types(letter);
    (first_len..=types.len())
        .map(|len| {
            let slice = &types[..len];
            make_impl(slice)
        })
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s
            } else {
                format!("{}\n{}", acc, s)
            }
        })
}

#[inline]
fn make_filepath(letter: char) -> PathBuf {
    PathBuf::from(format!("src/tuple_tools/{}2.rs", letter))
}

fn make_file(content: String, path: PathBuf) -> std::io::Result<()> {
    let content = format!("use crate::tuple_tools::Tuple;\n{}", content);
    fs::write(path, content)?;
    Ok(())
}

fn main() {
    {
        let content = make_1char_all();
        let path = PathBuf::from("src/tuple_tools/base1.rs");
        make_file(content, path).expect("failed to create file");
    }
    for c in char_range('A'..='D') {
        let content = make_2char_all(c);
        let path = make_filepath(c);
        make_file(content, path).expect("failed to create file");
    }
}
