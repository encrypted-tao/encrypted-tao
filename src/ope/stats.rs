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

/*
 * uniform_sample
 *      Sample uniform distribution using coins
 *      as a source of 'randomness'
*/
pub fn uniform_sample(mut in_range: Range, coins: String) -> u64 {
       
    //let mut in_range = Range {start: in_start, end: in_end};
    let mut cur = in_range.copy();
    let mut index = 0;
 
 
    let array = coins.to_string().into_bytes();
            
    while cur.size() > 1 {
                
        let mid = ((cur.start + cur.end) / 2);
                
        if array[index] == 0 {
            cur.end = mid;
        }
     
        if array[index] == 1 {
            cur.start = mid + 1;
        }
 
        index = index + 1;
    }
    return cur.start;
 
}
 
