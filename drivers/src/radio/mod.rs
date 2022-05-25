//! Custom radio communication protocol and radio communication drivers for the Phoenix open-source quadcopter.

use bitvec::BitVec;


/// Creates a new bitvector with Hamming error correction codes integrated.
#[allow(unused_variables)]
fn error_correct<'a>(bits: &BitVec<'a>) -> BitVec<'a> {
    todo!();
}

/// Transmits bitvector over radio protocol.
#[allow(unused_variables)]
#[allow(dead_code)]
pub fn transmit<'a>(bits: &BitVec<'a>) {
    let bits = error_correct(bits);

    todo!();
}