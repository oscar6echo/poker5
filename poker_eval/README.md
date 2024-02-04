# Poker hand evaluator

This crate contains fast [Texas hold'em poker](https://en.wikipedia.org/wiki/Texas_hold_%27em) hand equity evaluator.  

Each of the 52 deck cards is attributed an integer key.  
The 9 least significant bits encode the card suit, the others 23 the card face.  

This encoding, based on keys precalculated by crate [poker_keygen](https://github.com/oscar6echo/poker5/tree/main/poker_keygen), in is such that it enables to uniquely identify a hand rank by a few simple operations (sums, bit mask, bit shift, and table lookups).  

## Eval

Building the lookup is performed in sequence:

### 5-card hands

Build 5-card lookup tables: [five::build_tables](eval::five::build_tables).  
Which enables function [get_rank_five](eval::five::get_rank_five).  

```rust
use poker_eval::eval::five::{build_tables, get_rank_five};

// precalculate lookup tables
let t5 = build_tables(false);

// run the evaluation multiple times
let rank = get_rank_five(&t5, [31, 26, 50, 16, 49]);
assert_eq!(rank, 3971)
```

### 7-card hands

Build 7-card lookup tables: [seven::build_tables](eval::seven::build_tables) using function [eval::seven::get_rank_seven] built on [eval::five::get_rank_five].  
Which enables function [get_rank](eval::seven::get_rank).  
This function is the entry point to evaluate a player hand.

```rust
use poker_eval::eval::seven::{build_tables, get_rank};

// precalculate lookup tables
let arc_t7 = build_tables(false);

// run the evaluation multiple times
let rank = get_rank(&arc_t7, [5, 4, 18, 31, 34, 48, 22]);
assert_eq!(rank, 1689)
```

## Calc

From function [get_rank](eval::seven::get_rank), 2 calculation functions are implemented:

+ deterministic, ie. exhaustive
+ monte carlo

### Deterministic

Function [calc_equity_det](calc::equity_det::calc_equity_det):  

+ Calculate the equity of all players hands through exhaustive simulation.  
+ This requires all players cards to be known.  
+ This is feasible because in all cases, the number of simulations is small - and the evaluator fast.  

```rust
use poker_eval::eval::seven::build_tables;
use poker_eval::calc::equity_det::calc_equity_det;

// precalculate lookup tables
let arc_t7 = build_tables(true);

// then you can call calc_equity_det multiple times
let equity = calc_equity_det(
    // clone of Arc<TableSeven>
    arc_t7.clone(),  
    // players cards  
    vec![[7, 8], [22, 27]],  
    // table cards  
    vec![51, 30, 41],  
    // verbose  
    true
);
println!("equity = {:?}", equity);
//Ok([[0.23131312, 0.10707071], [0.55454546, 0.10707071]])
```

### Monte carlo

Function [calc_equity_mc](calc::equity_mc::calc_equity_monte_carlo):  

+ Calculate the equity of the first player though monte carlo simulation.  
+ This does not require all players to be known.  
+ Because the number of cases is potentially massive, a number of simlations must be specified.  

```rust
use poker_eval::eval::seven::build_tables;
use poker_eval::calc::equity_mc::calc_equity_monte_carlo;

// precalculate lookup tables
let arc_t7 = build_tables(true);

// then you can call calc_equity_monte_carlo multiple times
let equity = calc_equity_monte_carlo(
    // clone of Arc<TableSeven>
    arc_t7.clone(),
    // player cards
    vec![vec![8, 9], vec![11, 28], vec![]],
    // table cards
    vec![15, 47, 23, 33],
    // number of game
    100_000_000,
);
println!("equity = {:?}", equity);
// Ok(HandEquity { win: 0.3167, tie: 0.0 })
```
