//! ## 5-card hand evaluation
//! Contains the following functions:
//! - [build_tables]: build the lookup tables for 5-card hand evaluation
//! - [get_rank_five]: get the rank of a 5-card hand

use super::target::HandStats;
use crate::keys;
use std::{collections::HashMap, iter::zip, time::Instant};

#[cfg(feature = "serde")]
use serde::Serialize;

/// ## Lookup tables for 5-card hand evaluation
/// + build in function [build_tables]
/// + used in function [get_rank_five]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TableFive {
    // keys
    pub pk: keys::Keys,
    // flush_five_rank[hand_flush_key] = rank
    pub flush_five_rank: Vec<u32>,
    // face_five_rank[hand_face_key] = rank
    pub face_five_rank: Vec<u32>,
    // hand_faces[i] = [f1, f2, f3, f4, f5]
    pub hand_faces: Vec<[usize; 5]>,
    // hand_type[i] = "high-card", "one-pair", etc.
    pub hand_type: Vec<String>,
    // number of hand ranks
    pub nb_hand_five_rank: u32,
    // hand stats
    pub hands: HashMap<String, HandStats>,
}

/// ## Build lookup tables for 5-card hand evaluation
/// The tables are build by going through all possible 5-card hands and assigning a rank to each hand.  
/// Hand types are: high-card
/// + one-pair
/// + two-pairs
/// + three-of-a-kind
/// + straight
/// + flush
/// + full-house
/// + four-of-a-kind
/// + straight-flush  
///
/// The hand types are gone through in their value order.  
/// In each hand type, the card combinations are gone through in their value order.  
///
/// For each hand:
/// + its lookup key is the sum of all its card keys (either `flush` or `face`).  
/// + its rank is the previous rank + 1.
///
/// Thus all possible 5-card hands are assigned a rank.
///
pub fn build_tables(verbose: bool) -> TableFive {
    let start = Instant::now();

    let pk = keys::build();
    let face_key = pk.face_five_key;
    let flush_key = pk.flush_five_key;
    let nb_face = pk.nb_face;

    let mut t5 = TableFive {
        pk: pk.clone(),
        flush_five_rank: vec![0; pk.max_flush_five_key as usize + 1],
        face_five_rank: vec![0; pk.max_face_five_key as usize + 1],
        hand_faces: Vec::new(),
        hand_type: Vec::new(),
        nb_hand_five_rank: 0,
        hands: HashMap::new(),
    };

    let mut rank = 0;

    // High Card
    let mut hand_high_card = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(4.., face_key[4..nb_face].iter()) {
        for (f2, &k2) in zip(0.., face_key[0..f1].iter()) {
            for (f3, &k3) in zip(0.., face_key[0..f2].iter()) {
                for (f4, &k4) in zip(0.., face_key[0..f3].iter()) {
                    for (f5, &k5) in zip(0.., face_key[0..f4].iter()) {
                        // No straights, including A2345
                        if !((f1 - f5 == 4) || (f1 == 12 && f2 == 3)) {
                            let hand_face_key = k1 + k2 + k3 + k4 + k5;
                            t5.face_five_rank[hand_face_key as usize] = rank;
                            t5.hand_faces.push([f1, f2, f3, f4, f5]);
                            t5.hand_type.push("high-card".to_string());
                            rank += 1;
                        }
                    }
                }
            }
        }
    }
    hand_high_card.max_rank = rank - 1;
    hand_high_card.nb_hand = hand_high_card.max_rank - hand_high_card.min_rank + 1;
    t5.hands.insert("high-card".to_string(), hand_high_card);

    // One Pair
    let mut hand_one_pair = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (f2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            for (f3, &k3) in zip(0.., face_key[0..f2].iter()) {
                for (f4, &k4) in zip(0.., face_key[0..f3].iter()) {
                    // No Three of a Kind
                    if !((f1 == f2) || (f1 == f3) || (f1 == f4)) {
                        let hand_face_key = 2 * k1 + k2 + k3 + k4;
                        t5.face_five_rank[hand_face_key as usize] = rank;
                        t5.hand_faces.push([f1, f1, f2, f3, f4]);
                        t5.hand_type.push("one-pair".to_string());
                        rank += 1;
                    }
                }
            }
        }
    }
    hand_one_pair.max_rank = rank - 1;
    hand_one_pair.nb_hand = hand_one_pair.max_rank - hand_one_pair.min_rank + 1;
    t5.hands.insert("one-pair".to_string(), hand_one_pair);

    // Two Pairs
    let mut hand_two_pairs = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (f2, &k2) in zip(0.., face_key[0..f1].iter()) {
            for (f3, &k3) in zip(0.., face_key[0..nb_face].iter()) {
                // No Three of a Kind
                if !((f1 == f3) || (f2 == f3)) {
                    let hand_face_key = 2 * k1 + 2 * k2 + k3;
                    t5.face_five_rank[hand_face_key as usize] = rank;
                    t5.hand_faces.push([f1, f1, f2, f2, f3]);
                    t5.hand_type.push("two-pairs".to_string());
                    rank += 1;
                }
            }
        }
    }
    hand_two_pairs.max_rank = rank - 1;
    hand_two_pairs.nb_hand = hand_two_pairs.max_rank - hand_two_pairs.min_rank + 1;
    t5.hands.insert("two-pairs".to_string(), hand_two_pairs);

    // Three of a kind
    let mut hand_three_of_a_kind = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (f2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            for (f3, &k3) in zip(0.., face_key[0..f2].iter()) {
                // No Four of a Kind
                if !((f1 == f2) || (f1 == f3)) {
                    let hand_face_key = 3 * k1 + k2 + k3;
                    t5.face_five_rank[hand_face_key as usize] = rank;
                    t5.hand_faces.push([f1, f1, f1, f2, f3]);
                    t5.hand_type.push("three-of-a-kind".to_string());
                    rank += 1;
                }
            }
        }
    }
    hand_three_of_a_kind.max_rank = rank - 1;
    hand_three_of_a_kind.nb_hand = hand_three_of_a_kind.max_rank - hand_three_of_a_kind.min_rank + 1;
    t5.hands.insert("three-of-a-kind".to_string(), hand_three_of_a_kind);

    // Straight
    let mut hand_straight = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };

    // Low Straight
    let f1 = 3;
    let f5 = 12;
    let hand_face_key = face_key[f1] + face_key[f1 - 1] + face_key[f1 - 2] + face_key[f1 - 3] + face_key[f5];
    t5.face_five_rank[hand_face_key as usize] = rank;
    t5.hand_faces.push([f1, f1 - 1, f1 - 2, f1 - 3, f5]);
    t5.hand_type.push("straight".to_string());
    rank += 1;

    // Other Straight
    for f1 in 4..nb_face {
        let hand_face_key = face_key[f1] + face_key[f1 - 1] + face_key[f1 - 2] + face_key[f1 - 3] + face_key[f1 - 4];
        t5.face_five_rank[hand_face_key as usize] = rank;
        t5.hand_faces.push([f1, f1 - 1, f1 - 2, f1 - 3, f1 - 4]);
        t5.hand_type.push("straight".to_string());
        rank += 1;
    }

    hand_straight.max_rank = rank - 1;
    hand_straight.nb_hand = hand_straight.max_rank - hand_straight.min_rank + 1;
    t5.hands.insert("straight".to_string(), hand_straight);

    // Flush
    let mut hand_flush = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(4.., flush_key[4..nb_face].iter()) {
        for (f2, &k2) in zip(0.., flush_key[0..f1].iter()) {
            for (f3, &k3) in zip(0.., flush_key[0..f2].iter()) {
                for (f4, &k4) in zip(0.., flush_key[0..f3].iter()) {
                    for (f5, &k5) in zip(0.., flush_key[0..f4].iter()) {
                        // No straights, including A2345
                        if !((f1 - f5 == 4) || (f1 == 12 && f2 == 3)) {
                            let hand_flush_key = k1 + k2 + k3 + k4 + k5;
                            t5.flush_five_rank[hand_flush_key as usize] = rank;
                            t5.hand_faces.push([f1, f2, f3, f4, f5]);
                            t5.hand_type.push("flush".to_string());
                            rank += 1;
                        }
                    }
                }
            }
        }
    }

    hand_flush.max_rank = rank - 1;
    hand_flush.nb_hand = hand_flush.max_rank - hand_flush.min_rank + 1;
    t5.hands.insert("flush".to_string(), hand_flush);

    // Full House
    let mut hand_full_house = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (f2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            // No Four of a Kind
            if !(f1 == f2) {
                let hand_face_key = 3 * k1 + 2 * k2;
                t5.face_five_rank[hand_face_key as usize] = rank;
                t5.hand_faces.push([f1, f1, f1, f2, f2]);
                t5.hand_type.push("full-house".to_string());
                rank += 1;
            }
        }
    }
    hand_full_house.max_rank = rank - 1;
    hand_full_house.nb_hand = hand_full_house.max_rank - hand_full_house.min_rank + 1;
    t5.hands.insert("full-house".to_string(), hand_full_house);

    // Four of a Kind
    let mut hand_four_of_a_kind = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };
    for (f1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (f2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            // No Five of a Kind
            if !(f1 == f2) {
                let hand_face_key = 4 * k1 + k2;
                t5.face_five_rank[hand_face_key as usize] = rank;
                t5.hand_faces.push([f1, f1, f1, f1, f2]);
                t5.hand_type.push("four-of-a-kind".to_string());
                rank += 1;
            }
        }
    }
    hand_four_of_a_kind.max_rank = rank - 1;
    hand_four_of_a_kind.nb_hand = hand_four_of_a_kind.max_rank - hand_four_of_a_kind.min_rank + 1;
    t5.hands.insert("four-of-a-kind".to_string(), hand_four_of_a_kind);

    // Straight Flush
    let mut hand_straight_flush = HandStats {
        min_rank: rank,
        max_rank: rank,
        nb_hand: 0,
        nb_occur: 0,
    };

    // Low Straight Flush
    let f1 = 3;
    let f5 = 12;
    let hand_flush_key = flush_key[f1] + flush_key[f1 - 1] + flush_key[f1 - 2] + flush_key[f1 - 3] + flush_key[f5];
    t5.flush_five_rank[hand_flush_key as usize] = rank;
    t5.hand_faces.push([f1, f1 - 1, f1 - 2, f1 - 3, f5]);
    t5.hand_type.push("straight-flush".to_string());
    rank += 1;

    // Other Straight Flush
    for f1 in 4..nb_face {
        let hand_flush_key =
            flush_key[f1] + flush_key[f1 - 1] + flush_key[f1 - 2] + flush_key[f1 - 3] + flush_key[f1 - 4];
        t5.flush_five_rank[hand_flush_key as usize] = rank;
        t5.hand_faces.push([f1, f1 - 1, f1 - 2, f1 - 3, f1 - 4]);
        t5.hand_type.push("straight-flush".to_string());
        rank += 1;
    }

    hand_straight_flush.max_rank = rank - 1;
    hand_straight_flush.nb_hand = hand_straight_flush.max_rank - hand_straight_flush.min_rank + 1;
    t5.hands.insert("straight-flush".to_string(), hand_straight_flush);

    t5.nb_hand_five_rank = rank;

    let end = Instant::now();

    if verbose {
        println!("five::build_tables runtime = {:?}", (end - start));
        println!("nb_hand_five_rank = {}", t5.nb_hand_five_rank);
    }

    t5
}

/// Get the rank of a 5-card hand
/// ## Arguments
/// * `t5`: [TableFive] - must be precalculated
/// * `c`: 5 cards all distinct integers from 0 to nb_face*nb_suit
///
/// ## Example
/// ```
/// use poker_eval::eval::five::{build_tables, get_rank_five};
///
/// // precalculate the lookup tables
/// let t5 = build_tables(true);
///
/// // run the evaluation multiple times
/// let rank = get_rank_five(&t5, [31, 26, 50, 16, 49]);
/// assert_eq!(rank, 3971);
/// ```
pub fn get_rank_five(t5: &TableFive, c: [usize; 5]) -> u32 {
    let card_suit = &t5.pk.card_suit;
    let card_face = &t5.pk.card_face;
    let flush_key = &t5.pk.flush_five_key;
    let face_key = &t5.pk.face_five_key;
    let rank: u32;

    let [s1, s2, s3, s4, s5]: [usize; 5] = c
        .iter()
        .map(|c| card_suit[*c])
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let [f1, f2, f3, f4, f5]: [usize; 5] = c
        .iter()
        .map(|c| card_face[*c])
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    if (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5) {
        // Flush
        let hand_flush_key = flush_key[f1] + flush_key[f2] + flush_key[f3] + flush_key[f4] + flush_key[f5];
        rank = t5.flush_five_rank[hand_flush_key as usize];
    } else {
        // Not Flush
        let hand_face_key = face_key[f1] + face_key[f2] + face_key[f3] + face_key[f4] + face_key[f5];
        rank = t5.face_five_rank[hand_face_key as usize];
    }
    return rank;
}

#[cfg(test)]
mod tests {

    use super::{build_tables, get_rank_five, TableFive};
    use crate::util::is_normal;

    #[test]
    fn check_t5_normal() {
        is_normal::<TableFive>();
    }

    #[test]
    fn eval_five_straight_flush() {
        let t5 = build_tables(true);

        let nb_suit = t5.pk.nb_suit;
        let nb_face = t5.pk.nb_face;

        for s in 0..nb_suit {
            let c1 = nb_suit * 3 + s;
            let c2 = nb_suit * 2 + s;
            let c3 = nb_suit * 1 + s;
            let c4 = nb_suit * 0 + s;
            let c5 = nb_suit * 12 + s;
            let rank_found = get_rank_five(&t5, [c1, c2, c3, c4, c5]);
            assert_eq!(rank_found, 7452);

            for (i, f1) in (4..nb_face).enumerate() {
                let c1 = nb_suit * f1 + s;
                let c2 = nb_suit * (f1 - 1) + s;
                let c3 = nb_suit * (f1 - 2) + s;
                let c4 = nb_suit * (f1 - 3) + s;
                let c5 = nb_suit * (f1 - 4) + s;
                let rank_found = get_rank_five(&t5, [c1, c2, c3, c4, c5]);
                assert_eq!(rank_found, 7453 + i as u32);
            }
        }
    }

    #[test]
    fn eval_five_samples() {
        let t5 = build_tables(true);

        for ([c1, c2, c3, c4, c5], r) in [
            ([21, 33, 24, 22, 39], 2459),
            ([51, 38, 14, 36, 17], 3431),
            ([45, 8, 48, 34, 5], 1171),
            ([13, 37, 33, 20, 35], 3106),
            ([31, 26, 50, 16, 49], 3971),
            ([28, 24, 25, 29, 2], 4434),
            ([41, 13, 28, 25, 16], 310),
            ([20, 36, 7, 42, 43], 3572),
            ([38, 42, 8, 22, 44], 761),
            ([32, 3, 18, 5, 42], 320),
            ([12, 8, 4, 0, 48], 7452),
            ([50, 46, 42, 38, 34], 7461),
            ([51, 47, 43, 39, 35], 7461),
            ([16, 12, 8, 4, 0], 7453),
        ]
        .iter()
        {
            let rank_found = get_rank_five(&t5, [*c1, *c2, *c3, *c4, *c5]);
            let rank_want = *r;
            assert_eq!(
                rank_found,
                rank_want,
                "-> want:rank={} found:rank={} for cards={:?}",
                rank_want,
                rank_found,
                [*c1, *c2, *c3, *c4, *c5],
            );
        }
    }
}
