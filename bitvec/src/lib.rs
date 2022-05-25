//! A simple library for handling bitvectors.

#![no_std]

/// Holds a bitvector of a specified length.
pub struct BitVec<'a> {
    pub bits: &'a [bool],
}

impl BitVec<'_> {
    /// Constructs a new instance of `BitVec` from a list of bytes.
    #[allow(unused_variables)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        todo!();
    }
}