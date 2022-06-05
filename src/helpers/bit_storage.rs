use log::{error, warn};
use rust_dataconverter_engine::{ListType, MapType, ObjectRef, ObjectRefMut, ObjectType, Types};
use crate::helpers::block_state::{BlockState, BlockStateOwned};

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

pub(crate) trait BitStorage {
    type Storage;

    fn try_wrap(bits: u8, size: usize, data: Self::Storage) -> Result<Self, String> where Self: Sized;
    fn get(&self, index: usize) -> u32;
    fn into_raw(self) -> Self::Storage;

    fn wrap(bits: u8, size: usize, data: Self::Storage) -> Self where Self: Sized {
        Self::try_wrap(bits, size, data).unwrap()
    }
}

pub(crate) trait BitStorageMut : BitStorage {
    fn set(&mut self, index: usize, value: u32);
    fn replace_storage<O>(&mut self, new_bits: u8, new_data: O) where Self::Storage: AsMut<O>, O: AsRef<[i64]>;
}

pub(crate) trait BitStorageOwned : BitStorageMut {
    fn new(bits: u8, size: usize) -> Self;

    fn resize(other: &impl BitStorage, new_bits: u8, size: usize) -> Self where Self: Sized {
        let mut result = Self::new(new_bits, size);
        for i in 0..size {
            result.set(i, other.get(i));
        }
        result
    }
}

const BIT_TO_LONG_SHIFT: u8 = 6; // log2(64)

pub(crate) struct PackedBitStorage<T> {
    data: T,
    bits: u8,
    mask: u32,
    size: usize,
}

impl<T> BitStorage for PackedBitStorage<T>
    where T: AsRef<[i64]>
{
    type Storage = T;

    fn try_wrap(bits: u8, size: usize, data: T) -> Result<Self, String> {
        if bits < 1 || bits > 32 {
            return Err(format!("Expected bits to be between 1 and 32, was: {}", bits));
        }
        let expected_len = (bits as usize * size + 63) / 64;
        if data.as_ref().len() != expected_len {
            return Err(format!("Expected data length for {} bits of size {} is {}, but was: {}", bits, size, expected_len, data.as_ref().len()));
        }
        Ok(Self {
            data,
            bits,
            mask: (1 << bits) - 1,
            size,
        })
    }

    fn get(&self, index: usize) -> u32 {
        debug_assert!(index < self.size);

        let bit_index = index * self.bits as usize;
        let word_index = bit_index >> BIT_TO_LONG_SHIFT;
        let end_word_index = ((index + 1) * self.bits as usize - 1) >> BIT_TO_LONG_SHIFT;
        let index_in_word = (bit_index ^ (word_index << BIT_TO_LONG_SHIFT)) as u8;
        if word_index == end_word_index {
            (self.data.as_ref()[word_index] >> index_in_word) as u32 & self.mask
        } else {
            let first_bits = 64 - index_in_word;
            ((self.data.as_ref()[word_index] >> index_in_word) as u32 & self.mask) | ((self.data.as_ref()[end_word_index] << first_bits) as u32 & self.mask)
        }
    }

    fn into_raw(self) -> T {
        self.data
    }
}

impl<T> BitStorageMut for PackedBitStorage<T>
    where T: AsMut<[i64]> + AsRef<[i64]>
{
    fn set(&mut self, index: usize, value: u32) {
        debug_assert!(index < self.size);
        debug_assert!(value <= self.mask);

        let bit_index = index * self.bits as usize;
        let word_index = bit_index >> BIT_TO_LONG_SHIFT;
        let end_word_index = ((index + 1) * self.bits as usize - 1) >> BIT_TO_LONG_SHIFT;
        let index_in_word = (bit_index ^ (word_index << BIT_TO_LONG_SHIFT)) as u8;
        self.data.as_mut()[word_index] = (self.data.as_ref()[word_index] & !((self.mask as i64) << index_in_word)) | (((value & self.mask) as i64) << index_in_word);
        if word_index != end_word_index {
            let bits_written = 64 - index_in_word;
            let bits_to_write = self.bits - bits_written;
            self.data.as_mut()[end_word_index] = (self.data.as_ref()[end_word_index] & !((1 << bits_to_write) - 1)) | ((value & self.mask) >> bits_written) as i64;
        }
    }

    fn replace_storage<O>(&mut self, new_bits: u8, new_data: O) where T: AsMut<O>, O: AsRef<[i64]> {
        assert_eq!((new_bits as usize * self.size + 63) / 64, new_data.as_ref().len());
        self.bits = new_bits;
        *self.data.as_mut() = new_data;
    }
}

impl BitStorageOwned for PackedBitStorage<Vec<i64>> {
    fn new(bits: u8, size: usize) -> Self {
        assert!(bits >= 1 && bits <= 32);
        Self {
            data: vec![0i64; (bits as usize * size + 63) / 64],
            bits,
            mask: (1 << bits) - 1,
            size,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct LocalPos {
    pub(crate) index: u16,
}
impl LocalPos {
    pub(crate) fn new(x: u8, y: u8, z: u8) -> Self {
        Self { index: ((x & 15) as u16) | ((y as u16) << 8) | (((z & 15) as u16) << 4) }
    }
    pub(crate) fn x(self) -> u8 { (self.index & 15) as u8 }
    pub(crate) fn y(self) -> u8 { (self.index >> 8) as u8 }
    pub(crate) fn z(self) -> u8 { ((self.index >> 4) & 15) as u8 }

    pub(crate) fn down(self) -> Self {
        debug_assert!(self.y() > 0);
        Self { index: self.index - 256 }
    }
    pub(crate) fn up(self) -> Self {
        debug_assert!(self.y() < 255);
        Self { index: self.index + 256 }
    }
    pub(crate) fn north(self) -> Self {
        debug_assert!(self.z() > 0);
        Self { index: self.index - 16 }
    }
    pub(crate) fn south(self) -> Self {
        debug_assert!(self.z() < 15);
        Self { index: self.index + 16 }
    }
    pub(crate) fn west(self) -> Self {
        debug_assert!(self.x() > 0);
        Self { index: self.index - 1 }
    }
    pub(crate) fn east(self) -> Self {
        debug_assert!(self.x() < 15);
        Self { index: self.index + 1 }
    }

    pub(crate) fn try_down(self) -> Option<Self> {
        if self.y() > 0 { Some(self.down()) } else { None }
    }
    pub(crate) fn try_up(self) -> Option<Self> {
        if self.y() < 255 { Some(self.up()) } else { None }
    }
    pub(crate) fn try_north(self) -> Option<Self> {
        if self.z() > 0 { Some(self.north()) } else { None }
    }
    pub(crate) fn try_south(self) -> Option<Self> {
        if self.z() < 15 { Some(self.south()) } else { None }
    }
    pub(crate) fn try_west(self) -> Option<Self> {
        if self.x() > 0 { Some(self.west()) } else { None }
    }
    pub(crate) fn try_east(self) -> Option<Self> {
        if self.x() < 15 { Some(self.east()) } else { None }
    }

    pub(crate) fn offset(self, dir: Direction) -> Self {
        match dir {
            Direction::Down => self.down(),
            Direction::Up => self.up(),
            Direction::North => self.north(),
            Direction::South => self.south(),
            Direction::West => self.west(),
            Direction::East => self.east(),
        }
    }

    pub(crate) fn try_offset(self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Down => self.try_down(),
            Direction::Up => self.try_up(),
            Direction::North => self.try_north(),
            Direction::South => self.try_south(),
            Direction::West => self.try_west(),
            Direction::East => self.try_east(),
        }
    }

    pub(crate) fn with_section_y(self, section_y: u8) -> Self {
        debug_assert!(self.y() < 16);
        Self { index: self.index + ((section_y as u16) << 12) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub(crate) const VALUES: [Direction; 6] = [
        Direction::Down,
        Direction::Up,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
}


pub(crate) struct Section<S> {
    pub(crate) palette: Vec<BlockStateOwned>,
    pub(crate) section_y: i32,
    storage: S,
}

impl<'a, S> Section<S>
    where S: BitStorage<Storage = &'a mut Vec<i64>>
{
    pub(crate) fn new<T: Types + ?Sized>(chunk_x: i32, chunk_z: i32, section: &'a mut T::Map, initializer: &mut impl SectionInitializer<T>) -> Option<Self> {
        let [palette, section_y, block_states] = section.get_mut_multi(["Palette", "Y", "BlockStates"]);
        let palette = palette?.as_list()?;
        let section_y = section_y.and_then(|o| o.as_i64()).unwrap_or(0) as i32;
        if palette.is_empty() {
            warn!("Chunk {}x{} section {} has empty palette", chunk_x, chunk_z, section_y);
            return None;
        }
        let block_states = match block_states?.as_ref_mut() {
            ObjectRefMut::LongArray(arr) => arr,
            _ => return None
        };

        let mut palette: Vec<_> = palette.iter().flat_map(|state| state.as_map()).flat_map(|state| BlockState::from_nbt::<T>(state)).collect();

        if initializer.init_skippable(&mut palette, section_y) {
            return None;
        }

        let palette: Vec<_> = <&Vec<_>>::into_iter(&palette).map(|state| state.to_owned()).collect();

        let bits = ceil_log2(palette.len() as u32).max(4);
        let storage = match S::try_wrap(bits, 4096, block_states) {
            Ok(storage) => storage,
            Err(err) => {
                warn!("Chunk {}x{} section {} has invalid block data: {}", chunk_x, chunk_z, section_y, err);
                return None;
            }
        };

        Some(Self {
            palette,
            section_y,
            storage,
        })
    }
}

impl<'a, S, V> Section<S>
    where S: BitStorage<Storage = V>, V: AsRef<[i64]>
{
    pub(crate) fn get_block(&self, pos: LocalPos) -> Option<&BlockStateOwned> {
        let index = pos.index & 4095;
        let palette_index = self.storage.get(index as usize);
        if palette_index as usize >= self.palette.len() {
            return None;
        }
        Some(&self.palette[palette_index as usize])
    }
}

impl<'a, S> Section<S>
    where S: BitStorageMut<Storage = &'a mut Vec<i64>>
{
    pub(crate) fn set_block<O: BitStorageOwned<Storage = Vec<i64>>>(&mut self, pos: LocalPos, block: BlockStateOwned) {
        let next_palette_index = self.palette.len() as u16;
        let palette_index = <&Vec<BlockStateOwned>>::into_iter(&self.palette).position(|it| it == &block).map(|index| index as u16).unwrap_or_else(|| {
            self.palette.push(block);
            next_palette_index
        });
        if palette_index >= 16 && palette_index.is_power_of_two() {
            let new_bits = (palette_index.log2() + 1) as u8;
            let new_storage = O::resize(&self.storage, new_bits, 4096);
            self.storage.replace_storage(new_bits, new_storage.into_raw());
        }

        let index = pos.index & 4095;
        self.storage.set(index as usize, palette_index as u32);
    }
}

pub(crate) trait SectionInitializer<T: Types + ?Sized> {
    fn init_skippable(&mut self, palette: &mut [BlockState], section_y: i32) -> bool;
}
