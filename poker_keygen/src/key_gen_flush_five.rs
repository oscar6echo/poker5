use std::iter::zip;
use std::time::Instant;

/// Generate integer keys for faces 1, 2, 3,.., 9, T, J, Q, K, A.  
/// These keys are such that the sums of any 2 combinations of 5 distinct flush keys are different.  
pub fn build() {
    //! Does not take any argument.

    println!("start key_gen_flush_five");

    // init
    let mut t: u32; // t=trial key, k=index searched key
    let mut valid: bool; // true if key is valid

    // value `s` is in set if `set[s]` contains current `t` value
    let mut set = Vec::new();
    let mut key = [0, 1, 2, 4, 8, 16, 32, 0, 0, 0, 0, 0, 0]; // bootstrapping - empirical

    println!("bootstrap -> keys={:?}", key);

    let start = Instant::now();
    t = 32;
    for k in 7..13 {
        assert_eq!(key[k - 1] + 1, t + 1);
        t = key[k - 1] + 1;
        let interm = Instant::now();
        valid = false;

        while !valid {
            key[k] = t;
            let c_max = k + 1;

            valid = true;
            set.resize(t as usize * 5, 0);

            'outer: for (c1, &k1) in zip(0.., key[0..c_max].iter()) {
                for (c2, &k2) in zip(c1 + 1.., key[c1 + 1..c_max].iter()) {
                    for (c3, &k3) in zip(c2 + 1.., key[c2 + 1..c_max].iter()) {
                        for (c4, &k4) in zip(c3 + 1.., key[c3 + 1..c_max].iter()) {
                            for (_c5, &k5) in zip(c4 + 1.., key[c4 + 1..c_max].iter()) {
                                let sum = (k1 + k2 + k3 + k4 + k5) as usize;
                                let e = &mut set[sum];
                                if *e == t {
                                    valid = false;
                                    break 'outer;
                                } else {
                                    *e = t;
                                }
                            }
                        }
                    }
                }
            }

            if valid {
                let end = Instant::now();
                println!(
                    "key[{}]={:?} - runtime key={:?}, total={:?}",
                    k,
                    t,
                    end - interm,
                    end - start
                );
            } else {
                t += 1;
            }
        }
    }

    let end = Instant::now();
    println!("runtime = {:?}", (end - start));
    println!("key={:?}", key);
}
