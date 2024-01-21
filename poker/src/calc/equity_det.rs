use std::{collections::HashSet, sync::Arc, thread, time::Instant, vec};

use thiserror::Error;

use crate::{
    eval::seven::{get_rank, TableSeven},
    keys::DECK_SIZE,
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HandEquity {
    pub win: f64,
    pub tie: f64,
}

#[derive(Error, Debug)]
pub enum GameError {
    // player
    #[error("invalid nb players: {0} - must be between 2 and 10")]
    InvalidNbPlayer(u32),
    #[error("invalid player card: {1} for player {0} - must be between 0 and 51")]
    InvalidPlayerCard(u32, u32),
    // table
    #[error("invalid nb table cards: {0} - must be among 0, 3, 4 or 5")]
    InvalidNbTableCard(u32),
    #[error("invalid table card: {0} - must be between 0 and 51")]
    InvalidTableCard(u32),
    // both
    #[error("players: {0:?} table: {1:?} - all cards must be distinct")]
    NotDistinctCards(Vec<[u32; 2]>, Vec<u32>),
}

pub fn calc_equity_det(
    t7: Arc<TableSeven>,
    player_cards: Vec<[u32; 2]>,
    table_cards: Vec<u32>,
    verbose: bool,
) -> Result<Vec<HandEquity>, GameError> {
    let start = Instant::now();

    let nb_player = player_cards.len();
    let deck_size = DECK_SIZE as u32;

    // start check input
    match nb_player {
        2..=10 => (),
        _ => return Err(GameError::InvalidNbPlayer(nb_player as u32)),
    }

    for (i, p) in player_cards.iter().enumerate() {
        for c in p.iter() {
            match *c {
                x if (x < deck_size) => (),
                _ => return Err(GameError::InvalidPlayerCard(i as u32, *c as u32)),
            }
        }
    }

    let nb_table_card = table_cards.len();
    match nb_table_card {
        0 | 3 | 4 | 5 => (),
        _ => return Err(GameError::InvalidNbTableCard(nb_table_card as u32)),
    }
    for t in table_cards.iter() {
        match t {
            0..=51 => (),
            _ => return Err(GameError::InvalidTableCard(*t)),
        }
    }

    let all_cards_set = player_cards
        .iter()
        .flatten()
        .copied()
        .chain(table_cards.iter().copied())
        .collect::<HashSet<u32>>();

    let all_cards_vec = player_cards
        .iter()
        .flatten()
        .copied()
        .chain(table_cards.iter().copied())
        .collect::<Vec<u32>>();

    match all_cards_vec.len() == all_cards_set.len() {
        true => (),
        false => return Err(GameError::NotDistinctCards(player_cards, table_cards)),
    }
    // end check input

    let deck = (0..deck_size)
        .filter(|c| !all_cards_set.contains(c))
        .collect::<Vec<u32>>();

    let nb_deck = deck.len() as usize;

    let mut eqty = vec![HandEquity { win: 0.0, tie: 0.0 }; nb_player];
    let mut rank = vec![0; nb_player];
    let mut n_game = 0;

    let player_cards_ = Arc::new(player_cards);
    let table_cards_ = Arc::new(table_cards);
    let deck_ = Arc::new(deck);

    // zero table cards
    if nb_table_card == 0 {
        let mut handles = vec![];

        for i1 in 0..nb_deck {
            let t7_ = Arc::clone(&t7);
            let player_cards_ = Arc::clone(&player_cards_);
            let deck_ = Arc::clone(&deck_);

            let mut eqty_ = Vec::new();
            for _ in 0..nb_player {
                eqty_.push(HandEquity { win: 0.0, tie: 0.0 });
            }

            let handle = thread::spawn(move || {
                let mut rank_ = vec![0; nb_player];
                let mut n_game_ = 0;

                for i2 in 0..i1 {
                    for i3 in 0..i2 {
                        for i4 in 0..i3 {
                            for i5 in 0..i4 {
                                for p in 0..nb_player {
                                    let cards = [
                                        player_cards_[p][0],
                                        player_cards_[p][1],
                                        deck_[i1],
                                        deck_[i2],
                                        deck_[i3],
                                        deck_[i4],
                                        deck_[i5],
                                    ];
                                    rank_[p] = get_rank(&t7_, cards.map(|x| x as usize));
                                }
                                update_eqty(&mut eqty_, &rank_);
                                n_game_ += 1;
                            }
                        }
                    }
                }
                (eqty_, n_game_)
            });
            handles.push(handle);
        }

        for handle in handles {
            let (eqty_, n_game_) = handle.join().unwrap();
            for p in 0..nb_player {
                eqty[p].win += eqty_[p].win;
                eqty[p].tie += eqty_[p].tie;
            }
            n_game += n_game_;
        }
    }

    // // zero table cards
    // if nb_table_card == 0 {
    //     for i1 in 0..nb_deck {
    //         for i2 in 0..i1 {
    //             for i3 in 0..i2 {
    //                 for i4 in 0..i3 {
    //                     for i5 in 0..i4 {
    //                         for p in 0..nb_player {
    //                             let cards = [
    //                                 player_cards[p][0],
    //                                 player_cards[p][1],
    //                                 deck[i1],
    //                                 deck[i2],
    //                                 deck[i3],
    //                                 deck[i4],
    //                                 deck[i5],
    //                             ];
    //                             rank[p] = get_rank(&t7, cards.map(|x| x as usize));
    //                         }
    //                         update_eqty(&mut eqty, &rank);
    //                         n_game += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // 3 table cards
    if nb_table_card == 3 {
        let t7_ = Arc::clone(&t7);
        let player_cards_ = Arc::clone(&player_cards_);
        let table_cards_ = Arc::clone(&table_cards_);
        let deck_ = Arc::clone(&deck_);

        for i1 in 0..nb_deck {
            for i2 in 0..i1 {
                for p in 0..nb_player {
                    let cards = [
                        player_cards_[p][0],
                        player_cards_[p][1],
                        table_cards_[0],
                        table_cards_[1],
                        table_cards_[2],
                        deck_[i1],
                        deck_[i2],
                    ];
                    rank[p] = get_rank(&t7_, cards.map(|x| x as usize));
                }
                update_eqty(&mut eqty, &rank);
                n_game += 1;
            }
        }
    }

    // 4 table cards
    if nb_table_card == 4 {
        let t7_ = Arc::clone(&t7);
        let player_cards_ = Arc::clone(&player_cards_);
        let table_cards_ = Arc::clone(&table_cards_);
        let deck_ = Arc::clone(&deck_);

        for i1 in 0..nb_deck {
            for p in 0..nb_player {
                let cards = [
                    player_cards_[p][0],
                    player_cards_[p][1],
                    table_cards_[0],
                    table_cards_[1],
                    table_cards_[2],
                    table_cards_[3],
                    deck_[i1],
                ];
                rank[p] = get_rank(&t7_, cards.map(|x| x as usize));
            }
            update_eqty(&mut eqty, &rank);
            n_game += 1;
        }
    }

    // 5 table cards
    if nb_table_card == 5 {
        let t7_ = Arc::clone(&t7);
        let player_cards_ = Arc::clone(&player_cards_);
        let table_cards_ = Arc::clone(&table_cards_);

        for p in 0..nb_player {
            let cards = [
                player_cards_[p][0],
                player_cards_[p][1],
                table_cards_[0],
                table_cards_[1],
                table_cards_[2],
                table_cards_[3],
                table_cards_[4],
            ];
            rank[p] = get_rank(&t7_, cards.map(|x| x as usize));
        }
        update_eqty(&mut eqty, &rank);
        n_game += 1;
    }

    let mut equity = Vec::new();
    for e in eqty.iter() {
        equity.push(HandEquity {
            win: e.win / (n_game as f64),
            tie: e.tie / (n_game as f64),
        });
    }

    let end = Instant::now();

    if verbose {
        println!("calc_equity_det runtime: {:?}", end - start);
    }

    Ok(equity)
}

fn update_eqty(eqty: &mut Vec<HandEquity>, rank: &Vec<u32>) -> () {
    let nb_player = eqty.len();

    let mut max_rank = rank[0];
    let mut nb_max = 1;

    for p in 1..nb_player {
        if rank[p] > max_rank {
            max_rank = rank[p];
            nb_max = 1;
        } else if rank[p] == max_rank {
            nb_max += 1;
        }
    }

    for p in 0..nb_player {
        if rank[p] == max_rank {
            if nb_max == 1 {
                eqty[p].win += 1.0;
            } else {
                eqty[p].tie += 1.0 / (nb_max as f64);
            }
        }
    }

    ()
}

#[cfg(test)]
mod tests {

    use crate::calc;
    use crate::eval::seven;

    use super::HandEquity;

    #[test]
    fn calc_equity_det() {
        let arc_t7 = seven::build_tables(true);

        let tests: Vec<(Vec<[u32; 2]>, Vec<u32>, Vec<[f64; 2]>)> = vec![
            (
                vec![[8, 9], [11, 28]],
                vec![15, 47, 23, 33],
                vec![[0.75, 0.0], [0.25, 0.0]],
            ),
            (
                vec![[8, 29], [4, 11]],
                vec![],
                vec![[0.6336246, 0.052030772], [0.2623138, 0.052030772]],
            ),
            (
                vec![[8, 29], [4, 11]],
                vec![13, 14, 50],
                vec![[0.42020202, 0.1550505], [0.26969698, 0.1550505]],
            ),
            (
                vec![[8, 29], [4, 11]],
                vec![13, 14, 50, 1],
                vec![[0.0, 0.03409091], [0.9318182, 0.03409091]],
            ),
            (
                vec![[8, 29], [4, 11]],
                vec![13, 14, 50, 22],
                vec![[0.6363636, 0.056818184], [0.25, 0.0568181]],
            ),
            (
                vec![[7, 8], [22, 27]],
                vec![],
                vec![[0.32409608, 0.023546053], [0.62881184, 0.023546053]],
            ),
            (
                vec![[7, 8], [22, 27]],
                vec![51, 30, 41],
                vec![[0.23131312, 0.10707071], [0.55454546, 0.10707071]],
            ),
            (
                vec![[7, 8], [22, 27]],
                vec![51, 30, 41, 9],
                vec![[0.8636364, 0.0], [0.13636364, 0.0]],
            ),
            (
                vec![[7, 8], [22, 27]],
                vec![51, 30, 41, 9, 5],
                vec![[1.0, 0.0], [0.0, 0.0]],
            ),
        ];

        for (players, table, results) in tests.iter() {
            let results_ = results
                .to_vec()
                .iter()
                .map(|[w, t]| HandEquity { win: *w, tie: *t })
                .collect::<Vec<HandEquity>>();

            let equity = calc::equity_det::calc_equity_det(arc_t7.clone(), players.clone(), table.clone(), true);
            assert!(
                equity.is_ok(),
                "-> fails: players={:?}, table={:?}",
                players.clone(),
                table.clone(),
            );

            let precision = 1e-3;
            if let Ok(equity) = equity {
                for (i, e) in equity.iter().enumerate() {
                    assert!(
                        (e.win - results_[i].win).abs() < precision,
                        "-> fails: players={:?}, table={:?}\nfound:equity={:?}, want:equity={:?}",
                        players,
                        table,
                        e,
                        results_[i]
                    );
                    assert!(
                        (e.tie - results_[i].tie).abs() < precision,
                        "-> fails: players={:?}, table={:?}\nfound:equity={:?}, want:equity={:?}",
                        players,
                        table,
                        e,
                        results_[i]
                    );
                }
            }
        }
    }
}
