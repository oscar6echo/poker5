use std::time::Instant;

pub fn build() {
    // generate keys for suits Spades, Hearts, Diamonds, Clubs
    // keys are such that the sums of any 2 combinations of 7 suits are distinct
    // (discarding all other card info)

    println!("start key_suit");

    // init
    const BOUND: usize = 40;
    let mut sums = [0; BOUND * 7]; // all possible sums
    let mut key = [0, 0, 0, 0];
    let mut n_sol = 0;

    let start = Instant::now();

    for k0 in 0..BOUND {
        for k1 in (k0 + 1)..BOUND {
            for k2 in (k1 + 1)..BOUND {
                for k3 in (k2 + 1)..BOUND {
                    key[0] = k0;
                    key[1] = k1;
                    key[2] = k2;
                    key[3] = k3;

                    let mut c = 0;
                    for c1 in 0..4 {
                        for c2 in c1..4 {
                            for c3 in c2..4 {
                                for c4 in c3..4 {
                                    for c5 in c4..4 {
                                        for c6 in c5..4 {
                                            for c7 in c6..4 {
                                                if c1 != c7 {
                                                    sums[c] = key[c1]
                                                        + key[c2]
                                                        + key[c3]
                                                        + key[c4]
                                                        + key[c5]
                                                        + key[c6]
                                                        + key[c7];
                                                    c += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let valid = (0..c - 1).all(|i| !sums[i + 1..c].contains(&sums[i]));

                    if valid {
                        n_sol += 1;
                        println!("n_sol={:<2} - key={:?}", n_sol, key);
                        if n_sol > 20 {
                            let end = Instant::now();
                            println!("\nruntime = {:?}", (end - start));
                            return;
                        }
                    }
                }
            }
        }
    }
}
