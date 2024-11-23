#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// todo maybe store the len in the u128 too, so it can be stored more efficiently

/// Array for storing boolean values.
///
/// Has a max capacity of 128.
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitArray {
    /// Stores the bits of the bit array. Caps the max capacity to 128 bits
    data: u128,
    /// The current len of the bit array
    len: u8
}

impl BitArray {
    /// The maximum amount of bits in an u128.
    const MAX_LEN: u8 = 128;

    pub fn new(iter: impl IntoIterator<Item=bool>) -> Self {
        let mut data = 0;
        let mut len = 0;

        iter
            .into_iter()
            .for_each(|b| {
                // shift the content of data one to the left, leaving a zero as latest element
                data <<= 1;

                if b {
                    // if the current bool is true, the rightmost bit (the latest element)
                    // is set to 1 using bitwise OR with 1
                    data |= 1;
                }

                if len < Self::MAX_LEN {
                    len += 1;
                } else {
                    panic!("Attempted to create a BitArray with an iterator yielding more than {} elements", Self::MAX_LEN)
                }
            });

        BitArray {
            data,
            len
        }
    }

    /// Get the bit value of the array at the given index.
    /// Returns None if the index is out of bounds
    pub fn get(&self, index: u8) -> Option<bool> {
        if !(0..self.len).contains(&index) {
            None
        } else {
            // As the bits are stored in a way where the latest
            // element is on the leas significant bit, the true index of the
            // wished bit must be calculated with this formula
            let bit_index = (self.len - 1) - index;
            // Bitwise AND with data and the desired index, which
            // will leave a number with a single bit set or zero if the bit was not set.
            // If anything greater than zero remained, the bit was set (true), otherwise not (false)
            Some(self.data & (1 << bit_index) > 0)
        }
    }

    /// Create an iterator over the bits of this array
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

        assert_eq!(format!("{:b}", arr.data), "10000111");
        assert_eq!(arr.len, 8);
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
        assert_eq!(arr.get(9), None)
    }

    #[test]
    fn iter_works() {
        let arr = BitArray::new([true, false, false, false, false, true, true, true]);

        let collect = arr.iter().collect::<Vec<_>>();

        assert_eq!(collect, vec![true, false, false, false, false, true, true, true])
    }
}