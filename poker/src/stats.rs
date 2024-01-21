use std::sync::Arc;
use std::thread;
use std::{cmp, collections::HashMap, collections::HashSet, time::Instant};

use crate::eval;
use crate::eval::five::TableFive;
use crate::eval::seven::TableSeven;
use crate::eval::target::{HandStats, STATS_FIVE, STATS_SEVEN};
use crate::keys::{self, NO_VALUE};

pub fn build_five(t5: Arc<TableFive>, verbose: bool) -> HashMap<String, HandStats> {
    let start = Instant::now();

    let deck_size = keys::DECK_SIZE;

    // let t5 = eval::five::build_tables(false);
    let t5_ = Arc::new(t5);
    let hand_types: HashSet<String> = STATS_FIVE.into_iter().map(|(ht, _)| ht.to_string()).collect();

    let mut hand_stats = hand_types
        .iter()
        .map(|ht| {
            (
                ht.to_string(),
                HandStats {
                    nb_hand: 0,
                    min_rank: NO_VALUE,
                    max_rank: 0,
                    nb_occur: 0,
                },
            )
        })
        .collect::<HashMap<String, HandStats>>();

    let mut rank_count = HashMap::new();

    for c1 in 0..deck_size {
        for c2 in 0..c1 {
            for c3 in 0..c2 {
                for c4 in 0..c3 {
                    for c5 in 0..c4 {
                        let rank = eval::five::get_rank_five(&t5_, [c1, c2, c3, c4, c5]);

                        if let Some(r) = rank_count.get(&rank) {
                            rank_count.insert(rank, r + 1);
                        } else {
                            rank_count.insert(rank, 1);
                        }
                    }
                }
            }
        }
    }

    for (rank, count) in rank_count.iter() {
        let ht = &t5_.hand_type[*rank as usize];
        let hs = hand_stats.get_mut(ht).unwrap();

        hs.nb_hand += 1;
        hs.nb_occur += count;

        hs.max_rank = cmp::max(hs.max_rank, *rank);
        hs.min_rank = cmp::min(hs.min_rank, *rank);
    }

    let end = Instant::now();

    if verbose {
        println!("stats::build_five runtime = {:?}", end - start);
        // println!("hand_stats = {:#?}", hand_stats);
    }

    hand_stats
}

pub fn build_seven(t7: Arc<TableSeven>, verbose: bool) -> HashMap<String, HandStats> {
    let start = Instant::now();

    let deck_size = keys::DECK_SIZE;

    let hand_types: HashSet<String> = STATS_SEVEN.into_iter().map(|(ht, _)| ht.to_string()).collect();

    let mut hand_stats = hand_types
        .iter()
        .map(|ht| {
            (
                ht.to_string(),
                HandStats {
                    nb_hand: 0,
                    min_rank: NO_VALUE,
                    max_rank: 0,
                    nb_occur: 0,
                },
            )
        })
        .collect::<HashMap<String, HandStats>>();

    let t7_ = Arc::new(t7);

    let mut handles = vec![];
    for c1 in 0..deck_size {
        let t7_ = Arc::clone(&t7_);
        let handle = thread::spawn(move || {
            let mut rank_count_ = HashMap::new();
            // let mut n = 0;
            for c2 in 0..c1 {
                for c3 in 0..c2 {
                    for c4 in 0..c3 {
                        for c5 in 0..c4 {
                            for c6 in 0..c5 {
                                for c7 in 0..c6 {
                                    let rank = eval::seven::get_rank(&t7_, [c1, c2, c3, c4, c5, c6, c7]);

                                    if let Some(r) = rank_count_.get(&rank) {
                                        rank_count_.insert(rank, r + 1);
                                    } else {
                                        rank_count_.insert(rank, 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            rank_count_
        });
        handles.push(handle);
    }

    let mut rank_count = HashMap::new();
    for handle in handles {
        let rank_count_sub = handle.join().unwrap();
        for (rank, count) in rank_count_sub.iter() {
            if let Some(r) = rank_count.get(rank) {
                rank_count.insert(*rank, r + count);
            } else {
                rank_count.insert(*rank, *count);
            }
        }
    }

    // for c1 in 0..deck_size {
    //     for c2 in 0..c1 {
    //         for c3 in 0..c2 {
    //             for c4 in 0..c3 {
    //                 for c5 in 0..c4 {
    //                     for c6 in 0..c5 {
    //                         for c7 in 0..c6 {
    //                             let rank = eval::seven::get_rank(&t7, [c1, c2, c3, c4, c5, c6, c7]);

    //                             if let Some(r) = rank_count.get(&rank) {
    //                                 rank_count.insert(rank, r + 1);
    //                             } else {
    //                                 rank_count.insert(rank, 1);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    let t7_ = Arc::clone(&t7_);
    for (rank, count) in rank_count.iter() {
        let ht = &t7_.t5.hand_type[*rank as usize];
        let hs = hand_stats.get_mut(ht).unwrap();

        hs.nb_hand += 1;
        hs.nb_occur += count;

        hs.max_rank = cmp::max(hs.max_rank, *rank);
        hs.min_rank = cmp::min(hs.min_rank, *rank);
    }

    let end = Instant::now();

    if verbose {
        println!("stats::build_seven runtime = {:?}", end - start);
        println!("hand_stats = {:#?}", hand_stats);
    }

    hand_stats
}

#[cfg(test)]
mod tests {

    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    use super::{build_five, build_seven};
    use crate::eval::target::{HandStats, STATS_FIVE, STATS_SEVEN};
    use crate::eval::{five, seven};
    use crate::util::is_normal;

    #[test]
    fn check_stats_normal() {
        is_normal::<HashMap<String, HandStats>>();
    }

    #[test]
    fn check_stats_five() {
        let t5 = five::build_tables(false);
        let t5_ = Arc::new(t5);
        let stats = build_five(t5_, false);
        // println!("stats = {:#?}", stats);

        let hand_types_found = stats.keys().map(|ht| ht.as_str()).collect::<HashSet<&str>>();
        let hand_types_want = STATS_FIVE.into_iter().map(|(ht, _)| ht).collect::<HashSet<&str>>();
        assert_eq!(
            hand_types_found, hand_types_want,
            "-> want:hand_types={:?} found:hand_types={:?}",
            hand_types_found, hand_types_want,
        );

        for (ht, hs_want) in STATS_FIVE.iter() {
            let hs_found = stats.get(*ht).unwrap();

            assert_eq!(
                hs_found.nb_hand, hs_want.nb_hand,
                "-> hand_type:{} want:nb_hand={} found:nb_hand={}",
                ht, hs_want.nb_hand, hs_found.nb_hand,
            );

            assert_eq!(
                hs_found.min_rank, hs_want.min_rank,
                "-> hand_type:{} want:min_rank={} found:min_rank={}",
                ht, hs_want.min_rank, hs_found.min_rank,
            );

            assert_eq!(
                hs_found.max_rank, hs_want.max_rank,
                "-> hand_type:{} want:max_rank={} found:max_rank={}",
                ht, hs_want.max_rank, hs_found.max_rank,
            );

            assert_eq!(
                hs_found.nb_occur, hs_want.nb_occur,
                "-> hand_type:{} want:nb_occur={} found:nb_occur={}",
                ht, hs_want.nb_occur, hs_found.nb_occur,
            );

            // println!("ht {:#?} ok", ht);
        }
    }

    #[test]
    fn check_stats_seven() {
        let t7 = seven::build_tables(false);
        let stats = build_seven(t7, true);
        // println!("stats = {:#?}", stats);

        let hand_types_found = stats.keys().map(|ht| ht.as_str()).collect::<HashSet<&str>>();
        let hand_types_want = STATS_SEVEN.into_iter().map(|(ht, _)| ht).collect::<HashSet<&str>>();
        assert_eq!(
            hand_types_found, hand_types_want,
            "-> want:hand_types={:?} found:hand_types={:?}",
            hand_types_found, hand_types_want,
        );

        for (ht, hs_want) in STATS_SEVEN.iter() {
            let hs_found = stats.get(*ht).unwrap();

            assert_eq!(
                hs_found.nb_hand, hs_want.nb_hand,
                "-> hand_type:{} want:nb_hand={} found:nb_hand={}",
                ht, hs_want.nb_hand, hs_found.nb_hand,
            );

            assert_eq!(
                hs_found.min_rank, hs_want.min_rank,
                "-> hand_type:{} want:min_rank={} found:min_rank={}",
                ht, hs_want.min_rank, hs_found.min_rank,
            );

            assert_eq!(
                hs_found.max_rank, hs_want.max_rank,
                "-> hand_type:{} want:max_rank={} found:max_rank={}",
                ht, hs_want.max_rank, hs_found.max_rank,
            );

            assert_eq!(
                hs_found.nb_occur, hs_want.nb_occur,
                "-> hand_type:{} want:nb_occur={} found:nb_occur={}",
                ht, hs_want.nb_occur, hs_found.nb_occur,
            );

            // println!("ht{:#?} ok", ht);
        }
    }
}
