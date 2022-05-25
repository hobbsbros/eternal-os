//! Custom radio communication protocol and radio communication drivers for the Phoenix open-source quadcopter.

use vec::{
    Message,
};


const MESSAGE_LEN: usize = 32;


/// Transmits bitvector over radio protocol.
#[allow(unused_variables)]
#[allow(dead_code)]
pub fn transmit(bits: &Message<MESSAGE_LEN>) {
    todo!();
}