// use poker::eval;
// use poker::stats;
// use poker::calc;

fn main() {
    println!("TACTICAL");

    // let _t5 = eval::five::build_tables(true);
    // let _t7 = eval::seven::build_tables(true);

    // let _result = eval::seven::get_rank_seven(&_t5, [50, 6, 0, 5, 38, 7, 17]);

    // let _stats_five = stats::build_five(true);
    // let _stats_seven = stats::build_seven(true);

    // try_calc_equity();
}

// fn try_calc_equity() {
//     let arc_t7 = eval::seven::build_tables(true);

//     let player_cards = vec![[7, 29], [4, 11]];
//     let table_cards = vec![];
//     let equity = calc::equity::calc_equity(arc_t7.clone(), player_cards, table_cards, true);
//     if let Ok(equity) = equity {
//         println!("0 table cards - equity = {:#?}", equity);
//     }

//     let player_cards = vec![[7, 29], [4, 11]];
//     let table_cards = vec![30, 41, 42];
//     let equity = calc::equity::calc_equity(arc_t7.clone(), player_cards, table_cards, true);
//     if let Ok(equity) = equity {
//         println!("3 table cards - equity = {:#?}", equity);
//     }

//     let player_cards = vec![[7, 29], [4, 11]];
//     let table_cards = vec![30, 41, 42, 28];
//     let equity = calc::equity::calc_equity(arc_t7.clone(), player_cards, table_cards, true);
//     println!("4 table cards = {:#?}", equity);

//     let player_cards = vec![[7, 29], [4, 11]];
//     let table_cards = vec![30, 41, 42, 28, 13];
//     let equity = calc::equity::calc_equity(arc_t7.clone(), player_cards, table_cards, true);
//     println!("5 table cards = {:#?}", equity);
// }
