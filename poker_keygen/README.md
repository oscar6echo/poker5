# Poker Keys Generation

## Overview

The purpose of this repo is to brute force search for keys which enable to uniquely identify a poker hand with a few sums, bitshift and bitmask.  
AFAIK this algo was introduced by [SpecialK](https://github.com/kennethshackleton/SKPokerEval).  

The sets of keys are:

+ **suit** keys:  
    4 keys mapped to suits Spades, Hearts, Diamonds, Clubs.  
    They are such that the sums of any 2 combinations of 7 suits are distinct.  
    They enable to determine whether a 7-card hand is a flush.  

+ **flush five** keys:  
    13 keys for faces 1, 2, 3,.., 9, T, J, Q, K, A.  
    They are such that the sums of any 2 combinations of 5 distinct faces are distinct.  
    They allow to uuid a 5-card flush hand.  

+ **face five** keys:  
    13 keys for faces 1, 2, 3,.., 9, T, J, Q, K, A.  
    They are such that the sums of any 2 combinations of 5 faces (with max same 4) are distinct.  
    They allow to uuid a 5-card non-flush hand.  

+ **flush seven** keys:  
    13 keys for faces 1, 2, 3,.., 9, T, J, Q, K, A.  
    They are such that the sums of any 2 combinations of 5 or 6 or 7 distinct faces are distinct.  
    They allow to uuid a 7-card flush hand.  

+ **face seven** keys:  
    13 keys for faces 1, 2, 3,.., 9, T, J, Q, K, A.  
    They are such that the sums of any 2 combinations of 7 faces (with max same 4) are distinct.  
    They allow to uuid a 7-card non-flush hand.  

It turns out that:

+ max(suit keys) < 2^9
+ max(face keys) < 2^23

So a card (among #suit x #face = 4 x 13 = 52) can be encoded in 32 bits - this plays well with computers !  
From there if all possible cases are pre calculated, a hand rank can be looked up as follows:  

+ Sum each card `suit_key` (extracted by bit shift) and lookup if hand is flush.  
+ If so: Sum each card (with suit="flush suit") `flush_seven` and lookup hand rank.  
+ If not: Sum each card `face_seven` (extracted by bit shift) and lookup hand rank.  

This provides a very fast way to evaluate hand ranks.

## Compute

Cf. conversation [this rust-lang forum conversation](https://users.rust-lang.org/t/rust-vs-c-vs-go-runtime-speed-comparison/104107).

The runtime is crazy fast.

## Run

Commands:

```sh
# run
cargo run --release
```

Output:

```txt
---------- key_gen_suit ----------
start key_suit
n_sol=1  - key=[0, 1, 26, 36]
n_sol=2  - key=[0, 1, 28, 39]
n_sol=3  - key=[0, 1, 29, 37]
n_sol=4  - key=[0, 1, 32, 39]
n_sol=5  - key=[0, 2, 30, 39]
n_sol=6  - key=[0, 2, 31, 38]
n_sol=7  - key=[0, 3, 30, 35]
n_sol=8  - key=[0, 3, 32, 37]
n_sol=9  - key=[0, 3, 33, 38]
n_sol=10 - key=[0, 3, 34, 39]
n_sol=11 - key=[0, 4, 34, 39]
n_sol=12 - key=[0, 5, 32, 35]
n_sol=13 - key=[0, 5, 34, 37]
n_sol=14 - key=[0, 5, 35, 38]
n_sol=15 - key=[0, 5, 35, 39]
n_sol=16 - key=[0, 5, 36, 39]
n_sol=17 - key=[0, 7, 36, 38]
n_sol=18 - key=[0, 7, 38, 39]
n_sol=19 - key=[0, 8, 36, 37]
n_sol=20 - key=[0, 9, 37, 39]
n_sol=21 - key=[0, 10, 35, 36]

runtime = 4.377632ms

---------- key_gen_flush_five ----------
start key_gen_flush_five
bootstrap -> keys=[0, 1, 2, 4, 8, 16, 32, 0, 0, 0, 0, 0, 0]
key[7]=56 - runtime key=39.734µs, total=39.796µs
key[8]=104 - runtime key=53.367µs, total=110.709µs
key[9]=192 - runtime key=70.388µs, total=185.169µs
key[10]=352 - runtime key=98.838µs, total=287.81µs
key[11]=672 - runtime key=215.281µs, total=507.327µs
key[12]=1288 - runtime key=539.178µs, total=1.050669ms
runtime = 1.05555ms
key=[0, 1, 2, 4, 8, 16, 32, 56, 104, 192, 352, 672, 1288]

---------- key_gen_flush_seven ----------
start key_gen_flush_five
bootstrap -> keys=[1, 2, 4, 8, 16, 32, 64, 128, 0, 0, 0, 0, 0]
key[8]=240 - runtime key=187.59µs, total=187.647µs
key[9]=464 - runtime key=406.029µs, total=598.453µs
key[10]=896 - runtime key=1.914396ms, total=2.517351ms
key[11]=1728 - runtime key=2.558433ms, total=5.083892ms
key[12]=3328 - runtime key=4.729589ms, total=9.824159ms
runtime = 9.839363ms
key=[1, 2, 4, 8, 16, 32, 64, 128, 240, 464, 896, 1728, 3328]

---------- key_gen_face_five_parallel ----------
start key-gen-face-five
bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
key[3]=22 - runtime key=102.427µs, total=102.493µs
key[4]=94 - runtime key=94.402µs, total=201.375µs
key[5]=312 - runtime key=132.967µs, total=338.839µs
key[6]=992 - runtime key=371.218µs, total=713.997µs
key[7]=2422 - runtime key=1.026149ms, total=1.745262ms
key[8]=5624 - runtime key=2.415269ms, total=4.16572ms
key[9]=12522 - runtime key=5.664388ms, total=9.8394ms
key[10]=19998 - runtime key=5.168113ms, total=15.019448ms
key[11]=43258 - runtime key=13.713849ms, total=28.74281ms
key[12]=79415 - runtime key=33.736905ms, total=62.491737ms
runtime = 62.50435ms
key=[0, 1, 5, 22, 94, 312, 992, 2422, 5624, 12522, 19998, 43258, 79415]

---------- key_gen_face_seven_parallel ----------
start key-gen-face-seven
bootstrap -> keys=[0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
key[3]=22 - runtime key=93.367µs, total=93.414µs
key[4]=98 - runtime key=64.458µs, total=162.028µs
key[5]=453 - runtime key=334.035µs, total=499.09µs
key[6]=2031 - runtime key=2.871583ms, total=3.376098ms
key[7]=8698 - runtime key=3.139063ms, total=6.524241ms
key[8]=22854 - runtime key=8.429608ms, total=14.962855ms
key[9]=83661 - runtime key=77.347271ms, total=92.318331ms
key[10]=262349 - runtime key=443.042953ms, total=535.376083ms
key[11]=636345 - runtime key=1.453851023s, total=1.989241379s
key[12]=1479181 - runtime key=6.226583853s, total=8.215835694s
runtime = 8.215845692s
key=[0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181]
```
