#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use seq::seq;
fn f1() -> u64 {
    1 * 2
}
fn f2() -> u64 {
    2 * 2
}
fn f3() -> u64 {
    3 * 2
}
fn f0() -> u64 {
    100
}
fn main() {
    let sum = f0() + f1() + f2() + f3();
    match (&sum, &(100 + 2 + 4 + 6)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
