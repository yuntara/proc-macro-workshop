#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use seq::seq;
enum Interrupt {
    Irq0,
    Irq1,
    Irq2,
    Irq3,
    Irq4,
    Irq5,
    Irq6,
    Irq7,
    Irq8,
    Irq9,
    Irq10,
    Irq11,
    Irq12,
    Irq13,
    Irq14,
    Irq15,
}
#[automatically_derived]
impl ::core::marker::Copy for Interrupt {}
#[automatically_derived]
impl ::core::clone::Clone for Interrupt {
    #[inline]
    fn clone(&self) -> Interrupt {
        *self
    }
}
impl ::core::marker::StructuralPartialEq for Interrupt {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Interrupt {
    #[inline]
    fn eq(&self, other: &Interrupt) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Interrupt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Interrupt::Irq0 => ::core::fmt::Formatter::write_str(f, "Irq0"),
            Interrupt::Irq1 => ::core::fmt::Formatter::write_str(f, "Irq1"),
            Interrupt::Irq2 => ::core::fmt::Formatter::write_str(f, "Irq2"),
            Interrupt::Irq3 => ::core::fmt::Formatter::write_str(f, "Irq3"),
            Interrupt::Irq4 => ::core::fmt::Formatter::write_str(f, "Irq4"),
            Interrupt::Irq5 => ::core::fmt::Formatter::write_str(f, "Irq5"),
            Interrupt::Irq6 => ::core::fmt::Formatter::write_str(f, "Irq6"),
            Interrupt::Irq7 => ::core::fmt::Formatter::write_str(f, "Irq7"),
            Interrupt::Irq8 => ::core::fmt::Formatter::write_str(f, "Irq8"),
            Interrupt::Irq9 => ::core::fmt::Formatter::write_str(f, "Irq9"),
            Interrupt::Irq10 => ::core::fmt::Formatter::write_str(f, "Irq10"),
            Interrupt::Irq11 => ::core::fmt::Formatter::write_str(f, "Irq11"),
            Interrupt::Irq12 => ::core::fmt::Formatter::write_str(f, "Irq12"),
            Interrupt::Irq13 => ::core::fmt::Formatter::write_str(f, "Irq13"),
            Interrupt::Irq14 => ::core::fmt::Formatter::write_str(f, "Irq14"),
            Interrupt::Irq15 => ::core::fmt::Formatter::write_str(f, "Irq15"),
        }
    }
}
fn main() {
    let interrupt = Interrupt::Irq8;
    match (&(interrupt as u8), &8) {
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
    match (&interrupt, &Interrupt::Irq8) {
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
