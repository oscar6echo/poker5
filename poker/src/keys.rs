use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{self, Serialize};
#[cfg(feature = "serde")]
use serde_big_array::BigArray;

use crate::util::*;

pub const NB_FACE: usize = 13;
pub const NB_SUIT: usize = 4;
pub const DECK_SIZE: usize = NB_SUIT * NB_FACE;
pub const SUIT_MASK: u32 = 511;
pub const SUIT_BIT_SHIFT: u32 = 9;

pub const SUIT: [char; 4] = ['C', 'D', 'H', 'S'];
pub const SUIT_KEY: [u32; 4] = [0, 1, 29, 37];

pub const FACE: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

pub const FLUSH_FIVE_KEY: [u32; NB_FACE] = [0, 1, 2, 4, 8, 16, 32, 56, 104, 192, 352, 672, 1288];
pub const FLUSH_SEVEN_KEY: [u32; NB_FACE] = [1, 2, 4, 8, 16, 32, 64, 128, 240, 464, 896, 1728, 3328];

pub const FACE_FIVE_KEY: [u32; NB_FACE] = [0, 1, 5, 22, 94, 312, 992, 2422, 5624, 12522, 19998, 43258, 79415];
pub const FACE_SEVEN_KEY: [u32; NB_FACE] = [0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181];

pub const MAX_SUIT_KEY: u32 = SUIT_KEY[3] * 7;

pub const MAX_FLUSH_FIVE_KEY: u32 = sum_last(FLUSH_FIVE_KEY, 5);
pub const MAX_FLUSH_SEVEN_KEY: u32 = sum_last(FLUSH_SEVEN_KEY, 7);

pub const MAX_FACE_FIVE_KEY: u32 = FACE_FIVE_KEY[NB_FACE - 1] * 4 + FACE_FIVE_KEY[NB_FACE - 2] * 1;
pub const MAX_FACE_SEVEN_KEY: u32 = FACE_SEVEN_KEY[NB_FACE - 1] * 4 + FACE_SEVEN_KEY[NB_FACE - 2] * 3;

pub const NO_VALUE: u32 = 999_999_999;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Keys {
    pub nb_face: usize,
    pub nb_suit: usize,
    pub deck_size: usize,
    pub suit_mask: u32,
    pub suit_bit_shift: u32,
    pub suit: [char; 4],
    pub suit_key: [u32; 4],
    pub face: [char; 13],

    pub flush_five_key: [u32; NB_FACE],
    pub flush_seven_key: [u32; NB_FACE],
    pub face_five_key: [u32; NB_FACE],
    pub face_seven_key: [u32; NB_FACE],

    pub max_suit_key: u32,
    pub max_flush_five_key: u32,
    pub max_flush_seven_key: u32,
    pub max_face_five_key: u32,
    pub max_face_seven_key: u32,

    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub card_face: [usize; DECK_SIZE],
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub card_suit: [usize; DECK_SIZE],
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub card_flush_key: [u32; DECK_SIZE],
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub card_face_key: [u32; DECK_SIZE],

    pub card_sy: HashMap<usize, String>,
    pub card_no: HashMap<String, usize>,
}

pub fn build() -> Keys {
    assert!(
        MAX_SUIT_KEY < (1 << SUIT_BIT_SHIFT),
        "suit keys are too large to be stored in SUIT_BIT_SHIFT={} bits",
        SUIT_BIT_SHIFT
    );

    assert!(
        MAX_FACE_SEVEN_KEY < (1 << (32 - SUIT_BIT_SHIFT)),
        "face keys are too large to be stored in 32-SUIT_BIT_SHIFT={} bits",
        32 - SUIT_BIT_SHIFT
    );

    let mut card_face: [usize; DECK_SIZE] = [0; DECK_SIZE];
    let mut card_suit: [usize; DECK_SIZE] = [0; DECK_SIZE];
    let mut card_flush_key: [u32; DECK_SIZE] = [0; DECK_SIZE];
    let mut card_face_key: [u32; DECK_SIZE] = [0; DECK_SIZE];
    let mut card_sy = HashMap::new();
    let mut card_no = HashMap::new();

    for f in 0..NB_FACE {
        for s in 0..NB_SUIT {
            let n = NB_SUIT * f + s;
            card_face[n] = f;
            card_suit[n] = s;

            card_flush_key[n] = FLUSH_SEVEN_KEY[f];
            card_face_key[n] = (FACE_SEVEN_KEY[f] << SUIT_BIT_SHIFT) + SUIT_KEY[s];

            card_sy.insert(n, format!("{}{}", FACE[f], SUIT[s]));
            card_no.insert(format!("{}{}", FACE[f], SUIT[s]), n);
        }
    }

    Keys {
        // constants
        nb_face: NB_FACE,
        nb_suit: NB_SUIT,
        deck_size: DECK_SIZE,
        suit_mask: SUIT_MASK,
        suit_bit_shift: SUIT_BIT_SHIFT,
        suit: SUIT,
        suit_key: SUIT_KEY,
        face: FACE,
        flush_five_key: FLUSH_FIVE_KEY,
        flush_seven_key: FLUSH_SEVEN_KEY,
        face_five_key: FACE_FIVE_KEY,
        face_seven_key: FACE_SEVEN_KEY,
        max_suit_key: MAX_SUIT_KEY,
        max_flush_five_key: MAX_FLUSH_FIVE_KEY,
        max_flush_seven_key: MAX_FLUSH_SEVEN_KEY,
        max_face_five_key: MAX_FACE_FIVE_KEY,
        max_face_seven_key: MAX_FACE_SEVEN_KEY,

        // constructed
        card_face: card_face,
        card_suit: card_suit,
        card_flush_key: card_flush_key,
        card_face_key: card_face_key,
        card_sy,
        card_no,
    }
}

impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n--- checks")?;
        writeln!(
            f,
            "max_suit_key={} < 2^suit_bit_shift=2^{}={} ? {}",
            self.max_suit_key,
            self.suit_bit_shift,
            1 << self.suit_bit_shift,
            self.max_suit_key < (1 << self.suit_bit_shift)
        )?;
        writeln!(
            f,
            "max_face_seven_key={} < 2^(32-suit_bit_shift)=2^{}={} ? {}",
            fmt_nb(self.max_face_seven_key),
            32 - self.suit_bit_shift,
            fmt_nb(1 << (32 - self.suit_bit_shift)),
            self.max_face_seven_key < (1 << (32 - self.suit_bit_shift))
        )?;
        writeln!(f, "\n--- cards")?;
        writeln!(f, "face = {:?}", self.face)?;
        writeln!(f, "suit = {:?}", self.suit)?;
        writeln!(f, "card_no = {:?}", self.card_no)?;
        writeln!(f, "card_sy = {:?}", self.card_sy)?;

        writeln!(f, "\n--- eval keys")?;
        writeln!(f, "nb_face = {:?}", self.nb_face)?;
        writeln!(f, "nb_suit = {:?}", self.nb_suit)?;
        writeln!(f, "deck_size = {:?}", self.deck_size)?;
        writeln!(f, "suit_mask = {:?}", self.suit_mask)?;
        writeln!(f, "suit_bit_shift = {:?}", self.suit_bit_shift)?;

        writeln!(f, "suit_key = {:?}", self.suit_key)?;
        writeln!(f, "flush_five_key = {:?}", self.flush_five_key)?;
        writeln!(f, "flush_seven_key = {:?}", self.flush_seven_key)?;
        writeln!(f, "face_five_key = {:?}", self.face_five_key)?;
        writeln!(f, "face_seven_key = {:?}", self.face_seven_key)?;

        writeln!(f, "max_suit_key = {:?}", self.max_suit_key)?;
        writeln!(f, "max_flush_five_key = {:?}", self.max_flush_five_key)?;
        writeln!(f, "max_flush_seven_key = {:?}", self.max_flush_seven_key)?;
        writeln!(f, "max_face_five_key = {:?}", self.max_face_five_key)?;
        writeln!(f, "max_face_seven_key = {:?}", self.max_face_seven_key)?;

        writeln!(f, "card_face = {:?}", self.card_face)?;
        writeln!(f, "card_suit = {:?}", self.card_suit)?;

        writeln!(f, "card_flush_key = {:?}", self.card_flush_key)?;
        writeln!(f, "card_face_key = {:?}", self.card_face_key)?;

        writeln!(f, "\n------")
    }
}

const fn sum_last(arr: [u32; NB_FACE], n: usize) -> u32 {
    let mut sum = 0;
    let mut i = arr.len() - n;
    while i < arr.len() {
        sum += arr[i];
        i += 1;
    }
    return sum;
}

#[cfg(test)]
mod tests {

    use super::Keys;
    use crate::util::is_normal;

    #[test]
    fn check_keys_normal() {
        is_normal::<Keys>();
    }
}
