pub fn naive_loops() -> i64 {
    let mut sum: i64 = 0;

    // very naive...
    for d1 in 1..=9 {
        for d2 in 0..=9 {
            if d2 == d1 {
                continue;
            }
            for d3 in 0..=9 {
                if d3 == d1 || d3 == d2 {
                    continue;
                }
                for d4 in (0..=9).step_by(2) { // d4 must be even
                    if d4 == d1 || d4 == d2 || d4 == d3 {
                        continue;
                    }
                    for d5 in 0..=9 {
                        if d5 == d1 || d5 == d2 || d5 == d3 || d5 == d4 {
                            continue;
                        }
                        if (d3+d4+d5) % 3 != 0 {  // this bit needs to be a multiple of 3
                            continue
                        }
                        for d6 in [0,5] {   // this bit needs to be a multiple of 5
                            if d6 == d1 || d6 == d2 || d6 == d3 || d6 == d4 || d6 == d5 {
                                continue;
                            }
                            for d7 in 0..=9 {
                                if d7 == d1 || d7 == d2 || d7 == d3 || d7 == d4 || d7 == d5 || d7 == d6 {
                                    continue;
                                }
                                if (d5*100 + d6*10 + d7) % 7 != 0 {
                                    continue;
                                }
                                for d8 in 0..=9 {
                                    if d8 == d1 || d8 == d2 || d8 == d3 || d8 == d4 || d8 == d5 || d8 == d6 || d8 == d7 {
                                        continue;
                                    }
                                    if (d6*100 + d7*10 + d8) % 11 != 0 {
                                        continue;
                                    }
                                    for d9 in 0..=9 {
                                        if d9 == d1 || d9 == d2 || d9 == d3 || d9 == d4 || d9 == d5 || d9 == d6 || d9 == d7 || d9 == d8 {
                                            continue;
                                        }
                                        if (d7*100 + d8*10 + d9) % 13 != 0 {
                                            continue;
                                        }
                                        for d10 in 0..=9 {
                                            if d10 == d1 || d10 == d2 || d10 == d3 || d10 == d4 || d10 == d5 || d10 == d6 || d10 == d7 || d10 == d8 || d10 == d9 {
                                                continue;
                                            }
                                            if (d8*100 + d9*10 + d10) % 17 != 0 {
                                                continue;
                                            }
                                            sum += d10 + d9*10 + d8*100 + d7*1_000 + d6*10_000 + d5*100_000 + d4*1_000_000 + d3*10_000_000 + d2*100_000_000 + d1*1_000_000_000
                                        }
                                    }
                                }
                            }
                        }
                    }

                }
            }
        }
    }

    sum
}