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

                len += 1;
            });

        BitArray {
            data,
            len
        }
    }


}

#[cfg(test)]
mod tests {
    use crate::BitArray;

    #[test]
    fn new_works() {
        let input = [true, false, false, false, false, true, true, true];

        let arr = BitArray::new(input);

        assert_eq!(format!("{:b}", arr.data), "10000111");
        assert_eq!(arr.len, 8);
    }
}