//! This main is just for testing purposes (e.g. for testing macros)

use tlib3::hashmap;

fn main() {
    dbg!(hashmap!(
        3 => 5,
        4 => 6,
        7 => 1
    ));
}