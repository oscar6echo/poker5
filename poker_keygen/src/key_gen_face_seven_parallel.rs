use std::{iter::zip, sync::Mutex, thread, time::Instant};

/// Generate integer keys for faces 1, 2, 3,.., 9, T, J, Q, K, A.  
/// These keys are such that the sums of any 2 combinations of 7 faces (with max same 4) are different.  
pub fn build() {
    //! Does not take any argument.

    println!("start key-gen-face-seven");

    // init
    let mut t: u32; // t=trial key, k=index searched key
    let mut key = [0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // bootstrapping - empirical

    println!("bootstrap -> keys={:?}", key);

    let start = Instant::now();
    t = 5;

    for k in 3..13 {
        assert_eq!(key[k - 1] + 1u32, t + 1);
        let t_init = key[k - 1] + 1u32;

        let interm = Instant::now();

        let parallel: usize = std::thread::available_parallelism().unwrap().into();

        let n: u32 = if k < 7 { 1 } else { parallel as u32 };

        let found = Mutex::new(None);
        let found_ref = &found;

        thread::scope(|s| {
            for offset in 0..n {
                let mut key = key;

                let mut set = vec![];

                let mut task = move || {
                    'outer: for t in (t_init + offset..).step_by(n as usize) {
                        if let Some(t2) = *found_ref.lock().unwrap() {
                            if t2 < t {
                                break;
                            }
                        }

                        key[k] = t;
                        let c_max = k + 1;

                        set.resize(t as usize * 7, 0);

                        for (c1, &k1) in zip(0.., key[0..c_max].iter()) {
                            for (c2, &k2) in zip(c1.., key[c1..c_max].iter()) {
                                for (c3, &k3) in zip(c2.., key[c2..c_max].iter()) {
                                    for (c4, &k4) in zip(c3.., key[c3..c_max].iter()) {
                                        for (c5, &k5) in zip(c4.., key[c4..c_max].iter()) {
                                            for (c6, &k6) in zip(c5.., key[c5..c_max].iter()) {
                                                for (c7, &k7) in zip(c6.., key[c6..c_max].iter()) {
                                                    if (c1 != c5) && (c2 != c6) & (c3 != c7) {
                                                        let sum = (k1 + k2 + k3 + k4 + k5 + k6 + k7)
                                                            as usize;
                                                        let e = &mut set[sum];
                                                        if *e == t {
                                                            continue 'outer;
                                                        } else {
                                                            *e = t;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let found2 = &mut *found_ref.lock().unwrap();

                        match found2 {
                            None => *found2 = Some(t),
                            Some(t2) if *t2 > t => *found2 = Some(t),
                            _ => (),
                        }

                        return;
                    }
                };
                if offset == n - 1 {
                    task();
                } else {
                    s.spawn(task);
                }
            }
        });
        t = found.into_inner().unwrap().unwrap();

        key[k] = t;

        let end = Instant::now();
        println!(
            "key[{}]={:?} - runtime key={:?}, total={:?}",
            k,
            t,
            end - interm,
            end - start
        );
    }

    let end = Instant::now();
    println!("runtime = {:?}", (end - start));
    println!("key={:?}", key);
}
