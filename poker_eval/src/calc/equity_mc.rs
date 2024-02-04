//! # Monte Carlo equity calculation
//! This module provides the function to calculate the equity of a hand in a monte carlo mode.

//!   # Example
//!
//! ```
//! use poker_eval::eval::seven::build_tables;
//! use poker_eval::calc::equity_mc::calc_equity_monte_carlo;
//!
//! // you need create Arc<TableSeven> arc_t7 beforehand once
//! let arc_t7 = build_tables(true);
//!
//! // then you can call calc_equity_mc multiple times
//! let equity = calc_equity_monte_carlo(
//!     // clone of Arc<TableSeven>
//!     arc_t7.clone(),
//!     // player cards
//!     vec![vec![8, 9], vec![11, 28], vec![]],
//!     // table cards
//!     vec![15, 47, 23, 33],
//!     // number of game
//!     10_000_000,
//! );
//! println!("equity = {:?}", equity);
//! // Ok(HandEquity { win: 0.3167, tie: 0.0 })
//! ```

use rand::seq::SliceRandom;
use std::{collections::HashSet, sync::Arc, thread, time::Instant};
use thiserror::Error;

use super::equity_det::HandEquity;
use crate::{
    eval::seven::{get_rank, TableSeven},
    keys::DECK_SIZE,
};

/// ## Game description error
/// This error type is used to describe the errors that can occur when describing a monte carlo game.  
#[derive(Error, Debug)]
pub enum McGameError {
    // player
    /// invalid number of players
    #[error("invalid nb players: {0} - must be between 1 and 10")]
    InvalidNbPlayer(u32),
    /// invalid first player
    #[error("invalid first player: {0:?} - 2 cards between 0 and 51 must be provided")]
    InvalidFirstPlayer(Vec<u32>),
    /// invalid other player
    #[error("invalid other player: {0:?} - 0, 1 or 2 cards between 0 and 51 must be provided")]
    InvalidOtherPlayer(u32, Vec<u32>),
    // table
    /// invalid number of table cards
    #[error("invalid nb table cards: {0} - must be between 0 and 5")]
    InvalidNbTableCard(u32),
    /// invalid table card
    #[error("invalid table card {0}: {1} - must be between 0 and 51")]
    InvalidTableCard(u32, u32),
    //both
    /// not distinct cards
    #[error("players: {0:?} table: {1:?} - all cards must be distinct")]
    NotDistinctCards(Vec<Vec<u32>>, Vec<u32>),
}

/// ## Calculate equity of hand in monte carlo mode
/// This does not require knowing all players cards.  
pub fn calc_equity_monte_carlo(
    t7: Arc<TableSeven>,
    player_cards: Vec<Vec<u32>>,
    table_cards: Vec<u32>,
    nb_game: u32,
) -> Result<HandEquity, McGameError> {
    let deck_size = DECK_SIZE as u32;
    let nb_player = player_cards.len() as u32;
    let nb_table_card = table_cards.len();

    // start check input
    match nb_player {
        1..=10 => (),
        _ => return Err(McGameError::InvalidNbPlayer(nb_player)),
    }

    for (i, p) in player_cards.iter().enumerate() {
        let err = match i {
            0 => McGameError::InvalidFirstPlayer(p.clone()),
            _ => McGameError::InvalidOtherPlayer(i as u32, p.clone()),
        };
        if i == 0 {
            match p.len() {
                2 => (),
                _ => return Err(err),
            }
        } else {
            match p.len() {
                0..=2 => (),
                _ => return Err(err),
            }
        }
        for c in p.iter() {
            match *c {
                x if (x < deck_size) => (),
                _ => return Err(err),
            }
        }
    }

    match nb_table_card {
        0..=5 => (),
        _ => return Err(McGameError::InvalidNbTableCard(nb_table_card as u32)),
    }
    for (i, t) in table_cards.iter().enumerate() {
        match t {
            x if (*x < deck_size) => (),
            _ => return Err(McGameError::InvalidTableCard(i as u32, *t)),
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
        false => return Err(McGameError::NotDistinctCards(player_cards, table_cards)),
    }
    // end check input

    let start = Instant::now();

    let deck = (0..deck_size)
        .filter(|c| !all_cards_set.contains(c))
        .collect::<Vec<u32>>();

    let mut arr_eqty = vec![];

    let mut handles = vec![];

    let n_thread = thread::available_parallelism().unwrap().get();
    let n_game_per_thread = nb_game / n_thread as u32;

    // println!("n_thread = {}", n_thread);
    // println!("n_game_per_thread = {}", n_game_per_thread);

    for _ in 0..n_thread {
        let t7_ = Arc::clone(&t7);
        let player_cards_ = player_cards.clone();
        let table_cards_ = table_cards.clone();
        let deck_ = deck.clone();

        let handle = thread::spawn(move || {
            let eqty_ = calc_eqty_batch(t7_, player_cards_, table_cards_, deck_, n_game_per_thread);
            eqty_
        });

        handles.push(handle);
    }

    for (_i, handle) in handles.into_iter().enumerate() {
        let eqty = handle.join().unwrap();
        // println!("eqty[{}] = {:?}", _i, eqty);
        arr_eqty.push(eqty);
    }

    let eqty = HandEquity {
        win: arr_eqty.iter().map(|x| x.win).sum::<f64>() / arr_eqty.len() as f64,
        tie: arr_eqty.iter().map(|x| x.tie).sum::<f64>() / arr_eqty.len() as f64,
    };

    let end = Instant::now();
    println!("runtime = {:?}", end - start);

    Ok(eqty)
}

fn calc_eqty_batch(
    t7: Arc<TableSeven>,
    player_cards: Vec<Vec<u32>>,
    table_cards: Vec<u32>,
    deck: Vec<u32>,
    nb_game: u32,
) -> HandEquity {
    let _start = Instant::now();

    let nb_player = player_cards.len() as u32;
    let nb_player_cards = player_cards.iter().map(|p| p.len()).sum::<usize>();
    let nb_table_cards = table_cards.len();
    let nb_rnd_cards = 2 * (nb_player as usize) - nb_player_cards + (5 - nb_table_cards);

    let mut deck_ = deck.clone();

    let mut rnd_cards = vec![0u32; nb_rnd_cards];
    let mut rnd_table_cards = vec![0u32; 5 - nb_table_cards];
    let mut rank = vec![0u32; nb_player as usize];
    let mut cards = [0u32; 7];

    let mut rnd_state = 0usize;
    let mut rnd_count = 0u32;

    let mut eqty = HandEquity { win: 0.0, tie: 0.0 };

    // // debug
    // let mut freq_rnd = (0..deck.len())
    //     .into_iter()
    //     .map(|x| (x as u32, 0.0))
    //     .collect::<HashMap<u32, f64>>();

    for _g in 0..nb_game {
        draw_card(&mut rnd_cards, &mut deck_, &mut rnd_state, &mut rnd_count);

        // // debug
        // for x in rnd_cards.iter() {
        //     match freq_rnd.get(x) {
        //         Some(count) => freq_rnd.insert(*x, count + 1.0),
        //         None => freq_rnd.insert(*x, 1.0),
        //     };
        // }

        let mut r = 0;

        for i in 0..5 - nb_table_cards {
            rnd_table_cards[i] = rnd_cards[i];
            r += 1;
        }

        for (p, player) in player_cards.iter().enumerate() {
            if p == 0 {
                cards[0] = player[0];
                cards[1] = player[1];
            } else {
                match player.len() {
                    2 => {
                        cards[0] = player[0];
                        cards[1] = player[1];
                    }
                    1 => {
                        cards[0] = player[0];
                        cards[1] = rnd_cards[r];
                        r += 1;
                    }
                    _ => {
                        cards[0] = rnd_cards[r];
                        cards[1] = rnd_cards[r + 1];
                        r += 2;
                    }
                }
            }
            for (t, table) in table_cards.iter().enumerate() {
                cards[2 + t] = *table;
            }
            for (i, rnd) in rnd_table_cards.iter().enumerate() {
                cards[2 + nb_table_cards + i] = *rnd;
            }

            let cards_ = cards.map(|x| x as usize);
            rank[p] = get_rank(&t7, cards_);
        }
        assert_eq!(r, rnd_cards.len());

        let mut max_rank = rank[0];
        let mut nb_max = 1;
        for p in 1..nb_player as usize {
            if rank[p] > max_rank {
                max_rank = rank[p];
                nb_max = 1;
            } else if rank[p] == max_rank {
                nb_max += 1;
            }
        }

        if rank[0] == max_rank {
            if nb_max == 1 {
                eqty.win += 1.0;
            } else {
                eqty.tie += 1.0 / nb_max as f64;
            }
        }
    }

    eqty.win /= nb_game as f64;
    eqty.tie /= nb_game as f64;

    // // debug
    // for (_, v) in freq_rnd.iter_mut() {
    //     *v /= nb_game as f64;
    //     *v *= 100.0;
    // }
    // println!("freq_rnd = {:?}", freq_rnd);

    let _end = Instant::now();
    // println!("runtime = {:?}", _end - _start);

    eqty
}

fn draw_card(rnd_card: &mut Vec<u32>, deck: &mut Vec<u32>, state: &mut usize, count: &mut u32) -> () {
    for c in 0..rnd_card.len() {
        rnd_card[c] = deck[*state];

        *state += 1;
        if *state == deck.len() {
            *state = 0;
        }
    }
    *count += 1;
    if *count % 100 == 0 {
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);
    }

    ()
}

#[cfg(test)]
mod tests {

    use super::HandEquity;
    use crate::calc;
    use crate::eval::seven;

    #[test]
    fn calc_equity_mc() {
        let arc_t7 = seven::build_tables(true);

        let tests: Vec<(Vec<Vec<u32>>, Vec<u32>, u32, [f64; 2], f64)> = vec![
            (
                vec![vec![8, 9], vec![11, 28]],
                vec![15, 47, 23, 33],
                100_000_000,
                [0.7502161, 0.0],
                1e-3,
            ),
            (
                vec![vec![8, 9], vec![11, 28]],
                vec![15, 47, 23],
                100_000_000,
                [0.5231909, 0.0192552],
                1e-3,
            ),
            (
                vec![vec![8, 9], vec![11, 28], vec![]],
                vec![15, 47, 23, 33],
                100_000_000,
                [0.3167, 0.0],
                1e-3,
            ),
            (
                vec![vec![8, 9], vec![11]],
                vec![15, 47, 23, 33],
                100_000_000,
                [0.3934488, 0.0088431],
                1e-3,
            ),
            (
                vec![vec![8, 9], vec![11], vec![]],
                vec![15, 47, 23, 33],
                100_000_000,
                [0.1678894, 0.003496],
                1e-3,
            ),
        ];

        for (players, table, nb_game, result, precision) in tests.iter() {
            let result_ = HandEquity {
                win: result[0],
                tie: result[1],
            };

            let equity =
                calc::equity_mc::calc_equity_monte_carlo(arc_t7.clone(), players.clone(), table.clone(), *nb_game);
            println!("equity = {:?}", equity);

            assert!(
                equity.is_ok(),
                "-> fails: players={:?}, table={:?}",
                players.clone(),
                table.clone(),
            );

            if let Ok(eqty) = equity {
                assert!(
                    (eqty.win - result_.win).abs() < *precision,
                    "-> fails: players={:?}, table={:?}\nfound:equity={:?}, want:equity={:?}",
                    players,
                    table,
                    eqty,
                    result_
                );
            }
        }
    }
}
