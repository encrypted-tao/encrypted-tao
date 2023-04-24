/*
 * File: stats.rs
 *      Statistics helper functions for Rust implementation of 
 *       Order Preserving Encryption OPE
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
use crate::ope::hgd::PRNG;

/*
 * uniform_sample
 *      Sample uniform distribution using coins
 *      as a source of 'randomness'
*/
pub fn uniform_sample(mut in_range: Range, mut coins: PRNG) -> u64 {
       
    //let mut in_range = Range {start: in_start, end: in_end};
    let mut cur = in_range.copy();
    let mut index = 0;
 
    coins.cipher.process(&[0;32], &mut coins.tape);

    while cur.size() > 1 {
                
        let mid = ((cur.start + cur.end) / 2);
                
        if coins.tape[index] == 0 {
            cur.end = mid;
        }
     
        if coins.tape[index] == 1 {
            cur.start = mid + 1;
        }
 
        index = index + 1;
    }
    return cur.start;
 
}
 
