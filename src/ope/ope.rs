/*
 * File: ope.rs
 *      Rust implementation of Order Preserving Encryption OPE
 *      Based off of a python implementation 
 *      (https://github.com/tonyo/pyope/blob/master/pyope/ope.py)
 *
 *      Ref: 
 *          https://faculty.cc.gatech.edu/~aboldyre/papers/bclo.pdf 
 *          https://people.csail.mit.edu/nickolai/papers/popa-mope-eprint.pdf 
 *          https://arxiv.org/pdf/2009.05679.pdf
 *
 */

extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate rand;
extern crate rand_distr;
extern crate hmac;
extern crate aes;
extern crate sha2;

pub mod ope {
   
    use hmac::{Mac, Hmac};
    use sha2::Sha256;
    use aes::Aes256;
    use aes::cipher::{
        BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit};
    use aes_prng::AesRng;
    use std::io::{Read, Write, Cursor};
    use std::fs::File;
    use rand::{Rng, thread_rng, SeedableRng};
    use rand::distributions::Uniform; // https://docs.rs/rand/latest/rand/distributions/uniform/index.html
    use rand_distr::{Distribution, Hypergeometric}; // https://rust-random.github.io/rand/rand_distr/struct.Hypergeometric.html
    use generic_array::{GenericArray, arr};
    use std::cmp;

    const DEFAULT_INPUT_RANGE_START: u64 = 0;
    const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
    const DEFAULT_OUTPUT_RANGE_START: u64 = 1;
    const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;
    
    pub struct PRNG {
        tape: u64,
    }
    impl PRNG {
        
        /* PRNG
         *      A pseudo-random number generator using
         *      the tape as a source of randomness
         */
        
        pub fn draw(&mut self) -> u64 {

            let coins: Vec<char> = self.tape.chars().collect();
           
            // sanity check
            assert_eq!(coins.len(), 32);
            
            let tmp = 0;

            for coin in &mut coins {

                tmp = (tmp << 1) | coin;
                
            }

            let ret = 1.0 * tmp / (DEFAULT_OUTPUT_RANGE_END);

            return  ret;
        }

    }

    pub struct Range {
        start: u64,
        end: u64,
    }

    impl Range {

        pub fn contains(&mut self, number: u64) -> bool {
            
            return self.start.ge(&number) && self.end.le(&number);
        }

        pub fn size(&mut self) -> u64 {

            return self.end - self.start + 1;
            

        }
        
        pub fn copy(&mut self) -> Range {

            return Range {start:self.start, end:self.end};
    }

    pub struct OPE {
        key: String,
        in_range: Range,
        out_range: Range,
    }

    impl OPE {

        pub fn log_gamma(x: f64) -> f64 {
            
            let v = vec![8.333333333333333e-02, -2.777777777777778e-03,
             7.936507936507937e-04, -5.952380952380952e-04,
             8.417508417508418e-04, -1.917526917526918e-03,
             6.410256410256410e-03, -2.955065359477124e-02,
             1.796443723688307e-01, -1.39243221690590e+00];

            let mut x0 = x * 1.0;
            let mut n = 0;

            if x0.eq(1.0) || x0.eq(2.0) {
                return 0.0;
            }

            else if x.le(7.0) {
                n = 7 - x as i64;
                x0 = (x * 1.0) + n;
            }

            let x2 = 1.0 / (x0 * x0);
            let xp = 2 * f64::consts::PI;
            let mut gl0 = v[9];

            for i in -1..8.rev() {
                gl0 *= x2;
                gl0 += a[i];
            }
            let mut gl = gl0 / x0 + 0.5 * xp.log() + (x0 - 0.5) * x0.log() - x0;

            if x.le(7.0) {
                for i in 1..(n+1) {
                    gl -= (x0 - 1.0).log();
                    x0 -= 1;
                }
            }
            return gl;
        }
        /*
         * hypergeo_sample
         *      Sample hypergeometric distribution using coins
         *      as a source of 'randomness'
         */
        pub fn hypergeo_sample(&mut self, in_range: Range, out_range: Range, seed: u64, coins: &[u8]) -> u64 {
            
   
            let pnrg = PNRG { coins: coins };
            let in_size = in_range.size();
            let out_size = out_range.size();

            let index = seed - out_range.start + 1;

            if in_size == out_size {
                
                return in_range.start + index - 1;
            
            }

            //let hypergeo = Hypergeometric::new(index, in_size, out_size - in_size).unwrap();

            let mut sample = 0;

            if index.gt(10) {

                let d1: f64 = 1.7155277699214135;
                let d2: f64 = 0.8989161620588988;

                let min: f64 = cmp::min(in_size, (out_size - in_size));
                let size = in_size + (out_size - in_size);
                let max: f64 = cmp::max(in_size, (out_size - in_size));

                let min_sample = cmp::min(index, size - sample);
                let d4: f64 = min as f64 / size;
                let d5 = 1.0 - d4;
                let d6 = min_sample * d4 + 0.5;
                let d7 = ((size - min) * index * d4 * d5 / (size - 1) + 0.5).sqrt();
                let d8 = d1 * d7 + d2;
                let d9 = ((min_sample + 1) * (min + 1) / (size + 2)).floor();
                let d10 =


                

                
            } else {
                
                let d1 = in_size + (out_size - in_size) - index;
                let d2: f64 = cmp::min(in_size, (out_size - in_size));

                let Y = d2;
                let K = index;

                while Y.gt(0.0)  {

                    let U = prng.draw();
                    Y -= (U + Y / (d1 + K)).floor();
                    K -= 1;

                    if K == 0 {
                        break;
                    }

                }
                let Z = d2 - Y;

                if in_size.gt((out_size - in_size)) {
                
                    sample = index - Z;

                }
                
                sample = Z;


            }

            if sample == 0 {
                
                return in_range.start;

            } else {

                return in_range.start + sample - 1;
            }

        }
        /*
         * uniform_sample
         *      Sample uniform distribution using coins
         *      as a source of 'randomness'
         */
        pub fn uniform_sample(&mut self, in_range: u64, coins: &[u8]) -> u64 {
       
           let cur = in_range.copy();
           let mut index = 0;
           
           while cur.size() > 1 {
               
               let mid = ((cur.start + cur.end) / 2).floor();
               
               if coins[index] == 0 {
                   cur.end = mid;
                }
    
               if coins[index == 1] {
                   cur.start = mid + 1;
               }

               index = index + 1;
           }
           return cur.start;

        }

        /*
         * encrypt(self, plaintext)
         *  OPE is recursive encryption, check cases then recursive helper
         */
        pub fn encrypt(&mut self, plaintext:u64) -> u64 {
            
            if !self.in_range.contains(plaintext) {
                 return 1 as u64;
            }

            return self.recursive_encrypt(plaintext, self.in_range, self.out_range);
        }
        
        pub fn recursive_encrypt(&mut self, plaintext: u64, in_range:Range, out_range:Range) -> u64 {

                let in_size = in_range.size();
                let out_size = out_range.size();
                let in_edge = in_range.start - 1;
                let out_edge = out_range.start -1;
                let tmp = (out_size as f64 / 2.0).ceil() as u64;
                let mid = out_edge + tmp;

                // sanity check 
                assert!(in_size.le(&out_size));

                if in_range.size() == 1 {
                    let min_in = in_range.start;
                    let mut tape = self.tape_gen(min_in);
                    let ciphertext = self.uniform_sample(in_range, tape);
                    return ciphertext;
                }
                
                let mut tape = self.tape_gen(mid);
                let samples = self.hypergeo_sample(in_range, out_range, mid, tape);

                if plaintext.le(&samples) {
                    in_range = Range { start:in_edge + 1, end:samples };
                    out_range = Range { start:out_edge + 1, end:mid };
                }  else {
                    in_range = Range { start:samples + 1, end:in_edge + in_size };
                    out_range = Range { start:mid + 1, end:out_edge + out_size };
                }

                return self.recursive_encrypt(plaintext, in_range, out_range)


        }   
        pub fn decrypt(&mut self, ciphertext: u64) -> u64 {
        
             if !self.in_range.contains(ciphertext) {
                 return 1 as u64;
             }

            return self.recursive_decrypt(ciphertext, self.out_range, self.out_range);

       
        }

        pub fn recursive_decrypt(&mut self, ciphertext: u64, in_range:Range, out_range:Range) -> u64 {
            
                let in_size = in_range.size();
                let out_size = out_range.size();
                let in_edge = in_range.start - 1;
                let out_edge = out_range.start -1;
                let tmp = (out_size as f64 / 2.0).ceil() as u64;
                let mid = out_edge + tmp;

                // sanity check
                assert!(in_size.le(&out_size));
                    
                if in_range.size() == 1 {
                    let min_in = in_range.start;
                    let mut tape = self.tape_gen(min_in);
                    let sample_text = self.uniform_sample(in_range, tape);
                    if sample_text.eq(&ciphertext) {
                        return min_in;
                    }
                    // else -> failure

                }
                
                let mut tape = self.tape_gen(mid);
                let samples = self.hypergeo_sample(in_range, out_range, mid, tape);

                if ciphertext.le(&mid) {
                    in_range = Range { start:in_edge + 1, end:samples };
                    out_range = Range { start:out_edge + 1, end:mid };
                }  else {
                    in_range = Range { start:samples + 1, end:in_edge + in_size };
                    out_range = Range { start:mid + 1, end:out_edge + out_size };
                }


                return self.recursive_decrypt(ciphertext, in_range, out_range);

        }

        /*
         * tape_gen(self, data)
         *  Return: bit string of data
         */
        pub fn tape_gen(&mut self, data: u64) -> &[u8] {
            
            let mut data_str = GenericArray::from(data.to_string().as_bytes());
            type HmacSha256 = Hmac<Sha256>;
            let mut hmac_obj = HmacSha256::new_from_slice(self.key.as_bytes());
            let aes_cipher = aes::Aes256::new(&mut hmac_obj.result());

            
            // sanity check
            assert_eq!(hmac_obj.digest_size, 32); 

           aes_cipher.encrypt_block(&mut data_str);
        
           return &data_str;

        }


    }
}    

/*
 * OPE tests
 *  run via `cargo test`
 *  TO DO: add more testing
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {

        let test = OPE { key:"", in_range: Range {start:DEFAULT_INPUT_RANGE_START, end: DEFAULT_INPUT_RANGE_END}, out_range: Range {start: DEFAULT_OUTPUT_RANGE_START, end: DEFAULT_OUTPUT_RANGE_END}};
        
        return true; 
    }

}
