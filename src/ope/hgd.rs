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
 *
 */

use std::cmp;
use std::str;
use crate::ope::ope::ope::Range;

pub struct PRNG {
    tape: String,
}

impl PRNG {
    
    /* PRNG
     *      A pseudo-random number generator using
     *      the tape as a source of randomness
     */
    
    pub fn draw(&mut self) -> f64 {

        // sanity check
        assert_eq!(self.tape.len(), 32);
        
        let mut tmp = 0;

        for coin in self.tape.chars() {

            tmp = (tmp << 1) | coin.to_digit(10).unwrap();
            
        }

        let ret = 1.0 * tmp as f64 / ((u32::max_value() - 1) as f64);

        return ret;
    }

}
 pub fn log_gamma(x: f64) -> f64 {
            
    let v = vec![8.333333333333333e-02, -2.777777777777778e-03,
     7.936507936507937e-04, -5.952380952380952e-04,
     8.417508417508418e-04, -1.917526917526918e-03,
     6.410256410256410e-03, -2.955065359477124e-02,
     1.796443723688307e-01, -1.39243221690590e+00];

    let mut x0 = x * 1.0;
    let mut n = 0.0;

    if x0.eq(&1.0) || x0.eq(&2.0) {
        return 0.0;
    }

    else if x.le(&7.0) {
        n = 7.0 - x;
        x0 = (x * 1.0) + n;
    }

    let x2 = 1.0 / (x0 * x0);
    let xp: f64 = 2.0 * std::f64::consts::PI;
    let mut gl0 = v[9];

    for i in (0..9).rev() {
        gl0 *= x2;
        gl0 += v[i-1];
    }
    let mut gl = gl0 / x0 + 0.5 * xp.log(std::f64::consts::E /* f64 */) + (x0 - 0.5) * x0.log(std::f64::consts::E /* f64 */) - x0;

    if x.le(&7.0) {
        for i in 1..((n+1.0) as i16) {
            gl -= (x0 - 1.0).log(std::f64::consts::E /* f64 */);
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
pub fn hypergeo_sample(in_start: u64, in_end:u64, out_start:u64, out_end:u64, seed: u64, coins: String) -> u64 {
            
   
        let mut prng = PRNG { tape: coins };
        let mut in_range = Range {start: in_start, end: in_end};
        let mut out_range = Range {start: out_start, end: out_end};
        let mut in_size = in_range.size();
        let mut out_size = out_range.size();

        let mut index: f64 = (seed - out_range.start + 1) as f64;

        if in_size.eq(&out_size) {
                
            return in_range.start + (index as u64) - 1;
            
        }

        let mut sample = 0.0;

        if index.gt(&10.0) {

            let d1: f64 = 1.7155277699214135;
            let d2: f64 = 0.8989161620588988;

            let min: f64 = cmp::min(in_size, out_size - in_size) as f64;
            let size: f64 = (in_size + (out_size - in_size)) as f64;
            let max: f64 = cmp::max(in_size, out_size - in_size) as f64;

            let min_sample: f64 = cmp::min(index as i32, (size - index) as i32) as f64;
            let d4: f64 = min as f64 / size;
            let d5: f64 = 1.0 - d4;
            let d6: f64 = min_sample * d4 + 0.5;
            let d7: f64 = ((size - min) * index as f64 * d4 * d5 / (size - 1.0) + 0.5).sqrt();
            let d8: f64 = d1 * d7 + d2;
            let d9: f64 = ((min_sample + 1.0) * (min + 1.0) / (size + 2.0)).floor();
            let d10: f64 = log_gamma(d9+1.0) + log_gamma(min-d9+1.0) + log_gamma((min_sample-d9+1.0) as f64) + log_gamma((max-min_sample+d9+1.0) as f64);
            let d11: f64 =  cmp::min((cmp::min(min_sample as u64, min as u64) + 1) as u64, (d6 + 16.0 * d7 as f64).floor() as u64) as f64;

            let mut Z: f64 = 0.0;

            loop {
                let X = prng.draw();
                let Y = prng.draw();

                let W = d6 + d8 * (Y - 0.5) / X;

                if W.lt(&0.0) || W.ge(&d11) {
                    continue;
                }

                Z = W.floor();
                let T = d10 - (log_gamma(Z+1.0) + log_gamma(min-Z+1.0) + log_gamma((min_sample-Z+1.0) as f64) + log_gamma(max-min_sample as f64+Z+1.0));

                if (X*(4.0-X)-3.0).le(&T) {
                    break;
                }

                if (X*(X-T)).ge(&1.0) {
                    continue;
                }

                if (2.0 * X.log(std::f64::consts::E /* f64 */)).le(&T) {
                     break;
                }

            }

            sample = Z;

             if in_size.gt(&(out_size - in_size)) {
                sample = min_sample - Z;
            }

            if min_sample.lt(&index) {
                sample = (in_size - (Z as u64)) as f64;
            }   

                
                
        } else {
            let d1: f64 = (in_size + (out_size - in_size) - (index as u64)) as f64;
            let d2: f64 = cmp::min(in_size, out_size - in_size) as f64;

            let mut Y = d2;
            let mut K = index;

            while Y.gt(&0.0)  {

                let U = prng.draw();
                Y -= (U + Y / (d1 + K)).floor();
                K -= 1.0;

                if K == 0.0 {
                     break;
                }

            }
            let Z = d2 - Y;

            if in_size.gt(&(out_size - in_size)) {
                
                sample = index - Z as f64;
                
            }
                
            sample = Z as f64;


        }

        if sample == 0.0 {
                
            return in_range.start;

        } else {

            return in_range.start + (sample - 1.0) as u64;
        }

}