use std::{iter::zip, time::Instant};

use crate::keys;

#[derive(Debug)]
pub struct TableFive {
    pub pk: keys::Keys,
    pub flush_five_rank: Vec<u32>,
    pub face_five_rank: Vec<u32>,
    pub hand_faces: Vec<[usize; 5]>,
    pub hand_type: Vec<String>,
    pub nb_hand_five_rank: u32,
}

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
    };

    let mut rank = 0;

    // High Card
    for (c1, &k1) in zip(4.., face_key[4..nb_face].iter()) {
        for (c2, &k2) in zip(0.., face_key[0..c1].iter()) {
            for (c3, &k3) in zip(0.., face_key[0..c2].iter()) {
                for (c4, &k4) in zip(0.., face_key[0..c3].iter()) {
                    for (c5, &k5) in zip(0.., face_key[0..c4].iter()) {
                        // No straights, including A2345
                        if !((c1 - c5 == 4) || (c1 == 12 && c2 == 3)) {
                            let hand_face_key = k1 + k2 + k3 + k4 + k5;
                            t5.face_five_rank[hand_face_key as usize] = rank;
                            t5.hand_faces.push([c1, c2, c3, c4, c5]);
                            t5.hand_type.push("high-card".to_string());
                            rank += 1;
                        }
                    }
                }
            }
        }
    }

    // One Pair
    for (c1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (c2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            for (c3, &k3) in zip(0.., face_key[0..c2].iter()) {
                for (c4, &k4) in zip(0.., face_key[0..c3].iter()) {
                    // No Three of a Kind
                    if !((c1 == c2) || (c1 == c3) || (c1 == c4)) {
                        let hand_face_key = 2 * k1 + k2 + k3 + k4;
                        t5.face_five_rank[hand_face_key as usize] = rank;
                        t5.hand_faces.push([c1, c1, c2, c3, c4]);
                        t5.hand_type.push("one-pair".to_string());
                        rank += 1;
                    }
                }
            }
        }
    }

    // Two Pairs
    for (c1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (c2, &k2) in zip(0.., face_key[0..c1].iter()) {
            for (c3, &k3) in zip(0.., face_key[0..nb_face].iter()) {
                // No Three of a Kind
                if !((c1 == c3) || (c2 == c3)) {
                    let hand_face_key = 2 * k1 + 2 * k2 + k3;
                    t5.face_five_rank[hand_face_key as usize] = rank;
                    t5.hand_faces.push([c1, c1, c2, c2, c3]);
                    t5.hand_type.push("two-pairs".to_string());
                    rank += 1;
                }
            }
        }
    }

    // Three of a kind
    for (c1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (c2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            for (c3, &k3) in zip(0.., face_key[0..c2].iter()) {
                // No Four of a Kind
                if !((c1 == c2) || (c1 == c3)) {
                    let hand_face_key = 3 * k1 + k2 + k3;
                    t5.face_five_rank[hand_face_key as usize] = rank;
                    t5.hand_faces.push([c1, c1, c1, c2, c3]);
                    t5.hand_type.push("three-of-a-kind".to_string());
                    rank += 1;
                }
            }
        }
    }

    // Low Straight
    let c1 = 3;
    let c5 = 12;
    let hand_face_key = face_key[c1] + face_key[c1 - 1] + face_key[c1 - 2] + face_key[c1 - 3] + face_key[c5];
    t5.face_five_rank[hand_face_key as usize] = rank;
    t5.hand_faces.push([c1, c1 - 1, c1 - 2, c1 - 3, c5]);
    t5.hand_type.push("straight".to_string());
    rank += 1;

    // Other Straight
    for c1 in 4..nb_face {
        let hand_face_key = face_key[c1] + face_key[c1 - 1] + face_key[c1 - 2] + face_key[c1 - 3] + face_key[c1 - 4];
        t5.face_five_rank[hand_face_key as usize] = rank;
        t5.hand_faces.push([c1, c1 - 1, c1 - 2, c1 - 3, c1 - 4]);
        t5.hand_type.push("straight".to_string());
        rank += 1;
    }

    // Flush
    for (c1, &k1) in zip(4.., flush_key[4..nb_face].iter()) {
        for (c2, &k2) in zip(0.., flush_key[0..c1].iter()) {
            for (c3, &k3) in zip(0.., flush_key[0..c2].iter()) {
                for (c4, &k4) in zip(0.., flush_key[0..c3].iter()) {
                    for (c5, &k5) in zip(0.., flush_key[0..c4].iter()) {
                        // No straights, including A2345
                        if !((c1 - c5 == 4) || (c1 == 12 && c2 == 3)) {
                            let hand_flush_key = k1 + k2 + k3 + k4 + k5;
                            t5.flush_five_rank[hand_flush_key as usize] = rank;
                            t5.hand_faces.push([c1, c2, c3, c4, c5]);
                            t5.hand_type.push("flush".to_string());
                            rank += 1;
                        }
                    }
                }
            }
        }
    }

    // Full House
    for (c1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (c2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            // No Four of a Kind
            if !(c1 == c2) {
                let hand_face_key = 3 * k1 + 2 * k2;
                t5.face_five_rank[hand_face_key as usize] = rank;
                t5.hand_faces.push([c1, c1, c1, c2, c2]);
                t5.hand_type.push("full-house".to_string());
                rank += 1;
            }
        }
    }

    // Four of a Kind
    for (c1, &k1) in zip(0.., face_key[0..nb_face].iter()) {
        for (c2, &k2) in zip(0.., face_key[0..nb_face].iter()) {
            // No Five of a Kind
            if !(c1 == c2) {
                let hand_face_key = 4 * k1 + k2;
                t5.face_five_rank[hand_face_key as usize] = rank;
                t5.hand_faces.push([c1, c1, c1, c1, c2]);
                t5.hand_type.push("four-of-a-kind".to_string());
                rank += 1;
            }
        }
    }

    // Low Straight Flush
    let c1 = 3;
    let c5 = 12;
    let hand_flush_key = flush_key[c1] + flush_key[c1 - 1] + flush_key[c1 - 2] + flush_key[c1 - 3] + flush_key[c5];
    t5.flush_five_rank[hand_flush_key as usize] = rank;
    t5.hand_faces.push([c1, c1 - 1, c1 - 2, c1 - 3, c5]);
    t5.hand_type.push("straight".to_string());
    rank += 1;

    // Other Straight Flush
    for c1 in 4..nb_face {
        let hand_flush_key =
            flush_key[c1 - 1] + flush_key[c1 - 1] + flush_key[c1 - 2] + flush_key[c1 - 3] + flush_key[c1 - 4];
        t5.flush_five_rank[hand_flush_key as usize] = rank;
        t5.hand_faces.push([c1, c1 - 1, c1 - 2, c1 - 3, c1 - 4]);
        t5.hand_type.push("straight".to_string());
        rank += 1;
    }

    t5.nb_hand_five_rank = rank;

    let end = Instant::now();

    if verbose {
        println!("five::build_tables runtime = {:?}", (end - start));
        println!("nb_hand_five_rank = {}", t5.nb_hand_five_rank);
    }

    t5
}

pub fn get_rank_five(t5: &TableFive, c1: usize, c2: usize, c3: usize, c4: usize, c5: usize) -> u32 {
    // input = 5 cards all distinct integers from 0 to nb_face*nb_suit
    // in order defined by card_no

    let suit = &t5.pk.card_suit;
    let card_face = &t5.pk.card_face;
    let flush_key = &t5.pk.flush_five_key;
    let face_key = &t5.pk.face_five_key;
    let rank: u32;

    let (f1, f2, f3, f4, f5) = (
        card_face[c1],
        card_face[c2],
        card_face[c3],
        card_face[c4],
        card_face[c5],
    );

    if (suit[c1] == suit[c2]) && (suit[c1] == suit[c3]) && (suit[c1] == suit[c4]) && (suit[c1] == suit[c5]) {
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

    use crate::eval::five::{build_tables, get_rank_five};

    #[test]
    fn eval_five() {
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
        ]
        .iter()
        {
            let rank_found = get_rank_five(&t5, *c1, *c2, *c3, *c4, *c5);
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
