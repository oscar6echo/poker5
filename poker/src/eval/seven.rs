use std::iter::zip;
use std::sync::Arc;
use std::time::Instant;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::eval::five;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TableSeven {
    pub face_rank: Vec<u32>,
    pub flush_rank: Vec<u32>,
    pub flush_suit: Vec<i32>,
    pub t5: five::TableFive,
}

pub fn build_tables(verbose: bool) -> Arc<TableSeven> {
    let start = Instant::now();

    let t5 = five::build_tables(false);

    let face_key = t5.pk.face_seven_key;
    let flush_key = t5.pk.flush_seven_key;
    let nb_face = t5.pk.nb_face;
    let nb_suit = t5.pk.nb_suit;

    let mut t7 = TableSeven {
        face_rank: vec![0; t5.pk.max_face_seven_key as usize + 1],
        flush_rank: vec![0; t5.pk.max_flush_seven_key as usize + 1],
        flush_suit: vec![0; t5.pk.max_suit_key as usize + 1],
        t5,
    };

    // face rank
    for f1 in 0..nb_face {
        for f2 in 0..(f1 + 1) {
            for f3 in 0..(f2 + 1) {
                for f4 in 0..(f3 + 1) {
                    for f5 in 0..(f4 + 1) {
                        for f6 in 0..(f5 + 1) {
                            for f7 in 0..(f6 + 1) {
                                // no 5 or more same faces
                                if (f1 - f5 > 0) && (f2 - f6 > 0) && (f3 - f7 > 0) {
                                    let hand_face_key = face_key[f1]
                                        + face_key[f2]
                                        + face_key[f3]
                                        + face_key[f4]
                                        + face_key[f5]
                                        + face_key[f6]
                                        + face_key[f7];
                                    // arbitrary valid suits (4*0, 3*1)
                                    let (c1, c2, c3, c4, c5, c6, c7) =
                                        (4 * f1, 4 * f2, 4 * f3, 4 * f4, 4 * f5 + 1, 4 * f6 + 1, 4 * f7 + 1);
                                    t7.face_rank[hand_face_key as usize] =
                                        get_rank_seven(&t7.t5, [c1, c2, c3, c4, c5, c6, c7]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // flush rank 7 cards
    for (f1, &k1) in zip(6.., flush_key[6..nb_face].iter()) {
        for (f2, &k2) in zip(0.., flush_key[0..f1].iter()) {
            for (f3, &k3) in zip(0.., flush_key[0..f2].iter()) {
                for (f4, &k4) in zip(0.., flush_key[0..f3].iter()) {
                    for (f5, &k5) in zip(0.., flush_key[0..f4].iter()) {
                        for (f6, &k6) in zip(0.., flush_key[0..f5].iter()) {
                            for (f7, &k7) in zip(0.., flush_key[0..f6].iter()) {
                                let hand_flush_key = k1 + k2 + k3 + k4 + k5 + k6 + k7;
                                // arbitrary suit (7*0)
                                let c1 = 4 * f1;
                                let c2 = 4 * f2;
                                let c3 = 4 * f3;
                                let c4 = 4 * f4;
                                let c5 = 4 * f5;
                                let c6 = 4 * f6;
                                let c7 = 4 * f7;
                                t7.flush_rank[hand_flush_key as usize] =
                                    get_rank_seven(&t7.t5, [c1, c2, c3, c4, c5, c6, c7]);
                            }
                        }
                    }
                }
            }
        }
    }

    // flush rank 6 cards
    for (f1, &k1) in zip(5.., flush_key[5..nb_face].iter()) {
        for (f2, &k2) in zip(0.., flush_key[0..f1].iter()) {
            for (f3, &k3) in zip(0.., flush_key[0..f2].iter()) {
                for (f4, &k4) in zip(0.., flush_key[0..f3].iter()) {
                    for (f5, &k5) in zip(0.., flush_key[0..f4].iter()) {
                        for (f6, &k6) in zip(0.., flush_key[0..f5].iter()) {
                            let hand_flush_key = k1 + k2 + k3 + k4 + k5 + k6;
                            // arbitrary suit (7*0)
                            let c1 = 4 * f1;
                            let c2 = 4 * f2;
                            let c3 = 4 * f3;
                            let c4 = 4 * f4;
                            let c5 = 4 * f5;
                            let c6 = 4 * f6;
                            let c7 = 4 * f6 + 1;
                            t7.flush_rank[hand_flush_key as usize] =
                                get_rank_seven(&t7.t5, [c1, c2, c3, c4, c5, c6, c7]);
                        }
                    }
                }
            }
        }
    }

    // flush rank 5 cards
    for (f1, &k1) in zip(4.., flush_key[4..nb_face].iter()) {
        for (f2, &k2) in zip(0.., flush_key[0..f1].iter()) {
            for (f3, &k3) in zip(0.., flush_key[0..f2].iter()) {
                for (f4, &k4) in zip(0.., flush_key[0..f3].iter()) {
                    for (f5, &k5) in zip(0.., flush_key[0..f4].iter()) {
                        let hand_flush_key = k1 + k2 + k3 + k4 + k5;
                        // arbitrary suit (7*0)
                        let c1 = 4 * f1;
                        let c2 = 4 * f2;
                        let c3 = 4 * f3;
                        let c4 = 4 * f4;
                        let c5 = 4 * f5;
                        let c6 = 4 * f5 + 1;
                        let c7 = 4 * f5 + 1;
                        t7.flush_rank[hand_flush_key as usize] = get_rank_seven(&t7.t5, [c1, c2, c3, c4, c5, c6, c7]);
                    }
                }
            }
        }
    }

    // flush suit
    for s1 in 0..nb_suit {
        for s2 in 0..(s1 + 1) {
            for s3 in 0..(s2 + 1) {
                for s4 in 0..(s3 + 1) {
                    for s5 in 0..(s4 + 1) {
                        for s6 in 0..(s5 + 1) {
                            for s7 in 0..(s6 + 1) {
                                let hand_suit_key = t7.t5.pk.suit_key[s1]
                                    + t7.t5.pk.suit_key[s2]
                                    + t7.t5.pk.suit_key[s3]
                                    + t7.t5.pk.suit_key[s4]
                                    + t7.t5.pk.suit_key[s5]
                                    + t7.t5.pk.suit_key[s6]
                                    + t7.t5.pk.suit_key[s7];
                                t7.flush_suit[hand_suit_key as usize] = -1;

                                for suit in 0..nb_suit {
                                    let mut suit_count = 0;
                                    if suit == s1 {
                                        suit_count += 1;
                                    }
                                    if suit == s2 {
                                        suit_count += 1;
                                    }
                                    if suit == s3 {
                                        suit_count += 1;
                                    }
                                    if suit == s4 {
                                        suit_count += 1;
                                    }
                                    if suit == s5 {
                                        suit_count += 1;
                                    }
                                    if suit == s6 {
                                        suit_count += 1;
                                    }
                                    if suit == s7 {
                                        suit_count += 1;
                                    }
                                    if suit_count >= 5 {
                                        t7.flush_suit[hand_suit_key as usize] = suit as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let end = Instant::now();

    if verbose {
        println!("seven::build_tables runtime = {:?}", (end - start));
    }

    Arc::new(t7)
}

pub fn get_rank_seven(t5: &five::TableFive, c: [usize; 7]) -> u32 {
    // input = array of 5 cards all distinct integers from 0 to nb_face*nb_suit
    // in order defined by card_no

    let mut best_hand_rank = 0;
    let mut arr = [0; 5];

    for c1 in 0..7 {
        for c2 in 0..c1 {
            let mut k = 0;
            for i in 0..7 {
                // exclude cards c1 and c2
                if !(i == c1) && !(i == c2) {
                    arr[k] = c[i] as usize;
                    k += 1;
                }
            }
            let hand_rank = five::get_rank_five(&t5, [arr[0], arr[1], arr[2], arr[3], arr[4]]);

            if hand_rank > best_hand_rank {
                best_hand_rank = hand_rank;
            }
        }
    }
    best_hand_rank
}

pub fn get_rank(t7: &TableSeven, c: [usize; 7]) -> u32 {
    // input = array of 7 cards all distinct integers from 0 to nb_face*nb_suit
    // in order defined by card_no

    let card_face_key = &t7.t5.pk.card_face_key;
    let card_flush_key = &t7.t5.pk.card_flush_key;

    let suit_mask = t7.t5.pk.suit_mask;
    let suit_bit_shift = t7.t5.pk.suit_bit_shift;
    let card_suit = &t7.t5.pk.card_suit;

    let face_rank = &t7.face_rank;
    let flush_rank = &t7.flush_rank;
    let flush_suit = &t7.flush_suit;

    let hand_rank;

    let hand_key = card_face_key[c[0]]
        + card_face_key[c[1]]
        + card_face_key[c[2]]
        + card_face_key[c[3]]
        + card_face_key[c[4]]
        + card_face_key[c[5]]
        + card_face_key[c[6]];
    let hand_suit_key = (hand_key & suit_mask) as usize;
    let hand_suit = flush_suit[hand_suit_key];

    if hand_suit == -1 {
        let hand_face_key = hand_key >> suit_bit_shift;
        hand_rank = face_rank[hand_face_key as usize];
    } else {
        let mut hand_flush_key = 0;
        for i in 0..7 {
            if card_suit[c[i]] == hand_suit as usize {
                hand_flush_key += card_flush_key[c[i]];
            }
        }
        hand_rank = flush_rank[hand_flush_key as usize];
    }

    hand_rank
}

#[cfg(test)]
mod tests {

    use super::{build_tables, get_rank, TableSeven};

    use crate::util::is_normal;

    #[test]
    fn check_t7_normal() {
        is_normal::<TableSeven>();
    }

    #[test]
    fn eval_seven() {
        let t7 = build_tables(true);

        for ([c1, c2, c3, c4, c5, c6, c7], r) in [
            ([50, 6, 0, 5, 38, 7, 17], 5124),
            ([23, 16, 34, 26, 0, 10, 8], 1766),
            ([14, 4, 0, 7, 20, 8, 47], 1625),
            ([10, 32, 43, 3, 25, 8, 49], 1925),
            ([1, 16, 49, 24, 43, 42, 33], 3676),
            ([49, 17, 1, 26, 11, 34, 20], 887),
            ([5, 4, 18, 31, 34, 48, 22], 1689),
            ([13, 47, 1, 25, 38, 26, 51], 2815),
            ([44, 2, 28, 1, 3, 18, 22], 5046),
            ([49, 27, 33, 51, 22, 1, 30], 4000),
        ]
        .iter()
        {
            let rank_found = get_rank(&t7, [*c1, *c2, *c3, *c4, *c5, *c6, *c7]);
            let rank_want = *r;
            assert_eq!(
                rank_found,
                rank_want,
                "-> want:rank={} found:rank={} for cards={:?}",
                rank_want,
                rank_found,
                [*c1, *c2, *c3, *c4, *c5, *c6, *c7],
            );
        }
    }
}
