//! Custom radio communication protocol for the Phoenix open-source quadcopter.

#![no_std]

use bitvec::BitVec;


/// Creates a new bitvector with Hamming error correction codes integrated.
#[allow(unused_variables)]
#[allow(dead_code)]
fn error_correct<'a>(bits: &BitVec<'a>) -> BitVec<'a> {
    todo!();
}