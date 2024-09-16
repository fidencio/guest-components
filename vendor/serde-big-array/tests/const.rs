#![no_std]

use serde_big_array::BigArray;
use serde_derive::{Deserialize, Serialize};

const NUMBER: usize = 127;

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr: [u8; NUMBER],
}

#[test]
fn test() {
    let s = S { arr: [1; NUMBER] };
    let j = serde_json::to_string(&s).unwrap();
    let s_back = serde_json::from_str::<S>(&j).unwrap();
    assert!(&s.arr[..] == &s_back.arr[..]);
}
