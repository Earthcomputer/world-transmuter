use log::error;
use rust_dataconverter_engine::{MapType, ObjectRef, ObjectType, Types};

pub(crate) const fn bitset_size(size: usize) -> usize {
    const USIZE_BITS: usize = std::mem::size_of::<usize>() * 8;
    (size + USIZE_BITS - 1) / USIZE_BITS
}

#[macro_export]
macro_rules! make_bit_arr {
    ($size:literal; $($values:literal),* $(,)?) => {
        {
            use bitvec::prelude::*;
            const USIZE_BITS: usize = std::mem::size_of::<usize>() * 8;
            let mut arr = [0usize; $crate::helpers::bit_storage::bitset_size(256)];
            $(
                arr[$values as usize / USIZE_BITS] |= 1usize << ($values as usize % USIZE_BITS);
            )*
            BitArray { data: arr, ..BitArray::ZERO }
        }
    }
}

pub(crate) struct ChunkNibbleArray<'a> {
    data: &'a [i8],
}

impl<'a> ChunkNibbleArray<'a> {
    pub(crate) fn wrap<T: Types + ?Sized>(nbt: &'a T::Map, key: &str) -> Option<Self> {
        nbt.get(key).and_then(|o| match o.as_ref() {
            ObjectRef::ByteArray(arr) => Some(arr),
            _ => None
        }).and_then(|arr| {
            if arr.len() != 2048 {
                error!("ChunkNibbleArrays should be 2048 bytes not: {}", arr.len());
                return None;
            }
            Some(Self { data: arr })
        })
    }

    pub(crate) fn get(&self, index: u16) -> u8 {
        let value = self.data[(index >> 1) as usize] as u8;
        // if we are an even index, we want lower 4 bits
        // if we are an odd index, we want upper 4 bits
        (value >> ((index & 1) << 2)) & 0xF
    }
}

pub(crate) const fn ceil_log2(n: u32) -> u8 {
    if n == 1 {
        return 0
    }
    (n - 1).log2() as u8 + 1
}

const BIT_TO_LONG_SHIFT: u8 = 6; // log2(64)

pub(crate) struct PackedBitStorage {
    data: Vec<i64>,
    bits: u8,
    mask: u32,
    size: usize,
}

impl PackedBitStorage {
    pub(crate) fn new(bits: u8, size: usize) -> Self {
        assert!(bits >= 1 && bits <= 32);
        Self {
            data: vec![0i64; (bits as usize * size + 63) / 64],
            bits,
            mask: (1 << bits) - 1,
            size,
        }
    }

    pub(crate) fn wrap(bits: u8, size: usize, data: Vec<i64>) -> Self {
        assert!(bits >= 1 && bits <= 32);
        assert_eq!(data.len(), (bits as usize * size + 63) / 64);
        Self {
            data,
            bits,
            mask: (1 << bits) - 1,
            size,
        }
    }

    pub(crate) fn into_raw(self) -> Vec<i64> {
        self.data
    }

    pub(crate) fn set(&mut self, index: usize, value: u32) {
        debug_assert!(index < self.size);
        debug_assert!(value <= self.mask);

        let bit_index = index * self.bits as usize;
        let word_index = bit_index >> BIT_TO_LONG_SHIFT;
        let end_word_index = ((index + 1) * self.bits as usize - 1) >> BIT_TO_LONG_SHIFT;
        let index_in_word = (bit_index ^ (word_index << BIT_TO_LONG_SHIFT)) as u8;
        self.data[word_index] = (self.data[word_index] & !((self.mask as i64) << index_in_word)) | (((value & self.mask) as i64) << index_in_word);
        if word_index != end_word_index {
            let bits_written = 64 - index_in_word;
            let bits_to_write = self.bits - bits_written;
            self.data[end_word_index] = (self.data[end_word_index] & !((1 << bits_to_write) - 1)) | ((value & self.mask) >> bits_written) as i64;
        }
    }

    pub(crate) fn get(&self, index: usize) -> u32 {
        debug_assert!(index < self.size);

        let bit_index = index * self.bits as usize;
        let word_index = bit_index >> BIT_TO_LONG_SHIFT;
        let end_word_index = ((index + 1) * self.bits as usize - 1) >> BIT_TO_LONG_SHIFT;
        let index_in_word = (bit_index ^ (word_index << BIT_TO_LONG_SHIFT)) as u8;
        if word_index == end_word_index {
            (self.data[word_index] >> index_in_word) as u32 & self.mask
        } else {
            let first_bits = 64 - index_in_word;
            ((self.data[word_index] >> index_in_word) as u32 & self.mask) | ((self.data[end_word_index] << first_bits) as u32 & self.mask)
        }
    }
}
