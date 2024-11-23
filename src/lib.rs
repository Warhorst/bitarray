#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Array for storing boolean values.
///
/// Has a max capacity of 128.
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitArray {
    /// Stores the bits of the bit array. Caps the max capacity to 128 bits
    data: u128,
}

impl BitArray {
    /// The maximum amount of bits in an u128.
    const MAX_LEN: u8 = 128;

    /// Create an empty array and set the first (or all) elements to the values provided by the given iterator.
    pub fn new(iter: impl IntoIterator<Item=bool>) -> Self {
        let mut arr = BitArray::default();

        iter
            .into_iter()
            .enumerate()
            .for_each(|(i, b)| {
                let res = arr.set(i as u8, b);

                if res.is_err() {
                    panic!("Attempted to create a BitArray with an iterator yielding more than {} elements", Self::MAX_LEN)
                }
            });

        arr
    }

    /// Get the bit value of the array at the given index.
    /// Returns None if the index is out of bounds
    pub fn get(&self, index: u8) -> Option<bool> {
        if !(0..Self::MAX_LEN).contains(&index) {
            None
        } else {
            // Bitwise AND with data and the desired index, which
            // will leave a number with a single bit set or zero if the bit was not set.
            // If anything greater than zero remained, the bit was set (true), otherwise not (false)
            Some(self.data & (1 << index) > 0)
        }
    }

    /// Set the bit at the given index to the given bit. Returns Err if the index
    /// is out of bounds.
    pub fn set(&mut self, index: u8, bit: bool) -> Result<(), ()> {
        if !(0..Self::MAX_LEN).contains(&index) {
            Err(())
        } else if bit {
            // perform bitwise OR with a 1 shifted to the desired index, which will switch it to 1
            self.data |= 1 << index;
            Ok(())
        } else {
            // perform bitwise AND with a number where every bit is switched to 1 except the desired
            // one, which will switch it to 0
            self.data &= !(1 << index);
            Ok(())
        }
    }

    /// Create an iterator over all the bits of this array
    pub fn iter(&self) -> BitArrayIter {
        // Just copy the array, as it is not that big
        BitArrayIter::new(*self)
    }
}

pub struct BitArrayIter {
    array: BitArray,
    counter: u8
}

impl BitArrayIter {
    fn new(array: BitArray) -> Self {
        BitArrayIter {
            array,
            counter: 0
        }
    }
}

impl Iterator for BitArrayIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.array.get(self.counter);
        self.counter += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(BitArray::MAX_LEN as usize))
    }
}

#[cfg(test)]
mod tests {
    use crate::BitArray;

    #[test]
    fn new_works() {
        let arr = BitArray::new([true, false, false, false, false, true, true, true]);
        assert_eq!(format!("{:b}", arr.data), "11100001");
    }

    #[test]
    #[should_panic]
    fn new_with_too_large_iter_fails() {
        let _ = BitArray::new([true; 129]);
    }

    #[test]
    fn get_works() {
        let arr = BitArray::new([true, false, false, false, false, true, true, true]);

        assert_eq!(arr.get(0), Some(true));
        assert_eq!(arr.get(2), Some(false));
        assert_eq!(arr.get(129), None)
    }

    #[test]
    fn iter_works() {
        let iter = [true, false, false, false, false, true, true, true];
        let arr = BitArray::new(iter);

        arr.iter()
            .enumerate()
            .for_each(|(i, b)| if i < iter.len() {
                assert_eq!(iter[i], b, "the first elements must be the same as in the given iter");
            } else {
                assert_eq!(false, b, "the remaining elements must be false, as they were not set")
            });
    }
}