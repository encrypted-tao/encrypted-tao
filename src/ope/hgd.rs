/*
 * File: hgd.rs
 *      Hypergeometric sampling for Rust implementation of
 *      Order Preserving Encryption OPE
 *      Based off of a python implementation
 *      (https://github.com/tonyo/pyope/blob/master/pyope/ope.py)
 *
 *      Ref:
 *          https://faculty.cc.gatech.edu/~aboldyre/papers/bclo.pdf
 *          https://people.csail.mit.edu/nickolai/papers/popa-mope-eprint.pdf
 *          https://arxiv.org/pdf/2009.05679.pdf
 *          https://www.researchgate.net/publication/220492268_ALGORITHM_668_H2PEC_sampling_from_the_hypergeometric_distribution
 *          https://netlib.org/toms-2014-06-10/668
 *
 *
 */

use crate::ope::ope::ope::Range;
use crate::ope::utils::generate_tape;
use crypto::symmetriccipher::SynchronousStreamCipher;
use std::cmp;

pub struct PRNG {
    pub tape: [u32; 96],
    pub cipher: Box<dyn SynchronousStreamCipher + 'static>,
}

impl PRNG {
    /* PRNG
     *      A pseudo-random number generator using
     *      the tape as a source of randomness
     */
    pub fn draw(&mut self) -> f64 {
        let mut tmp = 0;

        let tape = generate_tape(self);
        let coins = &tape[0..32];

        // sanity check
        assert_eq!(coins.len(), 32);

        for coin in coins {
            tmp = (tmp << 1) | coin;
        }

        let ret = tmp as f64 / (u32::max_value() - 1) as f64;

        // sanity check
        assert!(0.0 <= ret && ret <= 1.0);
        return ret;
    }
}
pub fn log_gamma(x: u64) -> f64 {
    let v = vec![
        8.333333333333333e-02,
        -2.777777777777778e-03,
        7.936507936507937e-04,
        -5.952380952380952e-04,
        8.417508417508418e-04,
        -1.917526917526918e-03,
        6.410256410256410e-03,
        -2.955065359477124e-02,
        1.796443723688307e-01,
        -1.39243221690590e+00,
    ];

    let x = x as f64 * 1.0;
    let mut x0 = x;
    let mut n: u64 = 0;

    if x0 == 1.0 || x0 == 2.0 {
        return 0.0;
    } else if x0 <= 7.0 {
        n = 7 - x as u64;
        x0 = x + n as f64;
    }

    let x2 = 1.0 / (x0 * x0);
    let xp: f64 = 2.0 * std::f64::consts::PI;
    let mut gl0 = v[9];

    for i in (0..9).rev() {
        gl0 *= x2;
        gl0 += v[i];
    }
    let mut gl = gl0 / x0 + 0.5 * xp.ln() + x0 - 0.5 * x0.ln() - x0;

    if x <= 7.0 {
        for _i in 1..(n + 1) {
            gl -= (x0 - 1.0).ln();
            x0 -= 1.0;
        }
    }

    return gl;
}

/*
 * hypergeo_sample
 *      Sample hypergeometric distribution using coins
 *      as a source of 'randomness'
 */
pub fn hypergeo_sample(
    in_start: u64,
    in_end: u64,
    out_start: u64,
    out_end: u64,
    seed: u64,
    mut coins: PRNG,
) -> u64 {
    let mut in_range = Range {
        start: in_start,
        end: in_end,
    };
    let mut out_range = Range {
        start: out_start,
        end: out_end,
    };
    let in_size = in_range.size();
    let mut out_size = out_range.size();
    let index: u64 = seed - out_range.start + 1;
    let mut sample = 0;

    // sanity checks
    assert!(in_size > 0 && out_size > 0);
    assert!(out_range.contains(seed));
    assert!(in_size <= out_size);

    if in_size == out_size {
        /* Input and output range sizes are equal */
        return in_range.start + index - 1;
    } else if index > 10 {
        /* If Index > 10, H2PE (Hypergeometric-2 Points-Exponential Tails */

        let d1: f64 = 1.7155277699214135;
        let d2: f64 = 0.8989161620588988;

        let min = cmp::min(in_size, out_size - in_size);
        let size = in_size + (out_size - in_size);
        let max = cmp::max(in_size, out_size - in_size);

        let min_sample = cmp::min(index, size - index);

        let d4 = min as f64 / size as f64;
        let d5 = 1.0 - d4;
        let d6: f64 = min_sample as f64 * d4 + 0.5;
        let d7: f64 = ((size - min) as f64 * index as f64 * d4 * d5
            / (size - 1) as f64
            + 0.5)
            .sqrt();
        let d8: f64 = d1 * d7 + d2;
        let d9 = ((min_sample + 1) * (min + 1) / (size + 2)) as u64;
        let d10: f64 = log_gamma(d9 + 1)
            + log_gamma(min - d9 + 1)
            + log_gamma(min_sample - d9 + 1)
            + log_gamma(max - min_sample + d9 + 1);
        let d11 = cmp::min(
            (cmp::min(min_sample, min)) + 1,
            (d6 + 16.0 * d7).floor() as u64,
        );

        let mut z = 0;
        loop {
            let x = coins.draw();
            let y = coins.draw();

            let w = d6 + d8 * (y - 0.5) / x;

            if w < 0.0 || w >= d11 as f64 {
                continue;
            }
            z = w.floor() as u64;

            let t = d10
                - (log_gamma(z + 1)
                    + log_gamma(min - z + 1)
                    + log_gamma(min_sample - z + 1)
                    + log_gamma(max - min_sample + z + 1));

            if (x * (4.0 - x) - 3.0) as u64 <= t as u64 {
                break;
            }

            if (x * (x - t)) as u64 >= 1 {
                continue;
            }

            if (2.0 * x.ln()) <= t {
                break;
            }
        }

        sample = z;

        if in_size > (out_size - in_size) {
            sample = min_sample - z;
        }

        if min_sample < index {
            sample = in_size - z;
        }
    } else {
        /* If index <= 10, Inverse Transformation */

        out_size = out_size - in_size;
        let d1 = in_size + (out_size - in_size) - index;
        let d2 = cmp::min(in_size, out_size - in_size);

        let mut y = d2;
        let mut k = index;

        while y > 0 {
            let u = coins.draw();
            y -= (u + y as f64 / (d1 + k) as f64).floor() as u64;
            k -= 1;

            if k == 0 {
                break;
            }
        }
        let z = (d2 - y) as u64;

        if in_size >= out_size - in_size {
            sample = index - z;
        }

        sample = z;
    }
    if sample == 0 {
        return in_range.start;
    } else {
        sample = in_range.start + sample as u64 - 1;
        assert!(in_range.contains(sample));

        return sample;
    }
}
