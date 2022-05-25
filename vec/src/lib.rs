//! A simple library for handling statically allocated vectors.

#![no_std]

#[derive(Copy, Clone)]
pub enum MaybeUninit<T> {
    Uninit,
    Value (T),
}


/// Holds a bit,
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Bit {
    Zero = 0,
    One = 255,
}


/// Holds a generic, statically allocated vector of a specified capacity.
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vec<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    len: usize,
    capacity: usize,
}

impl<T, const N: usize> Vec<T, N> {
    const ELEM: MaybeUninit<T> = MaybeUninit::Uninit;
    const ITER: [MaybeUninit<T>; N] = [Self::ELEM; N];

    /// Constructs a new `Vec` of a specified type and capacity.
    pub fn new() -> Self {
        Self {
            buffer: Self::ITER,
            len: 0,
            capacity: N,
        }
    }

    /// Returns the length of the vector.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the vector.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Appends a new value to a vector.
    ///
    /// # Return Type
    /// This returns an `Option<()>` as it first checks that the `Vec` is not at capacity.
    /// 
    /// # Safety
    /// This function requires `unsafe` code.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn push(&mut self, value: T) -> Option<()> {
        if self.len() < self.capacity() {
            // TODO: Implement pushing (unsafe)
            todo!();
        } else {
            None
        }
    }
}


/// Holds a block of 11 information bits and 5 Hamming error correction bits.
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct BitBlock {
    data: Vec<MaybeUninit<Bit>, 11>,
    error: Vec<MaybeUninit<Bit>, 5>,
}

impl BitBlock {
    /// Creates an empty `BitBlock` struct,  initializing all bits to `MaybeUninit::Uninit`.
    pub fn new() -> Self {
        Self {
            data: Vec::<MaybeUninit<Bit>, 11>::new(),
            error: Vec::<MaybeUninit<Bit>, 5>::new(),
        }
    }

    /// Returns a 16-bit `BitVector` containing the bits of this `BitBlock`.
    pub fn read(&self) -> Vec<MaybeUninit<Bit>, 16> {
        todo!();
    }
}


/// Holds a list of 16-bit `BitBlock` structs.
#[allow(dead_code)]
pub struct Message<const N: usize> {
    blocks: [BitBlock; N],
}

impl<const N: usize> Message<N> {
    /// Creates an empty `Message` struct, initializing all bits to `MaybeUninit::Uninit`.
    pub fn new() -> Self {
        Self {
            blocks: [BitBlock::new(); N],
        }
    }

    /// Creates a `Message` struct from an array of bits.
    #[allow(unused_variables)]
    pub fn from(buffer: &[Bit]) -> Self {
        // TODO: Create instances of `BitBlock`
        // TODO: Implement error correction
        todo!();
    }
}