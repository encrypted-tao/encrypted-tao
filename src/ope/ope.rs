/*
 * File: ope.rs
 *      Rust implementation of Order Preserving Encryption OPE
 *      Based off of a python implementation 
 *      (https://github.com/tonyo/pyope/blob/master/pyope/ope.py)
 *
 *      Ref: 
 *          https://faculty.cc.gatech.edu/~aboldyre/papers/bclo.pdf 
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


    const DEFAULT_INPUT_RANGE_START: u64 = 0;
    const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
    const DEFAULT_OUTPUT_RANGE_START: u64 = 1;
    const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;
    
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
    }

    pub struct OPE {
        key: String,
        in_range: Range,
        out_range: Range,
    }

    impl OPE {

        /*
         * hypergeo_sample
         *      Sample hypergeometric distribution using coins
         *      as a source of 'randomness'
         */
        pub fn hypergeo_sample(&mut self, in_range: Range, out_range: Range, seed: u64, coins: &[u8]) -> u64 {
            
            let in_size = in_range.size();
            let out_size = out_range.size();

            let index = seed - out_range.start + 1;

            if in_size == out_size {
                
                return in_range.start + index - 1;
            }

            let hypergeo = Hypergeometric::new(index, in_size, out_size - in_size).unwrap();
            let samples = hypergeo.sample(&mut coins);

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
                    let side = Uniform::new(out_range.start, out_range.end);
                    let ciphertext = rng.sample(side);

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
                    let mut rng = thread_rng().fill(&mut tape);
                    let side = Uniform::new(out_range.start, out_range.end);
                    let sample_text = rng.sample(side); // https://rust-random.github.io/rand/rand/distributions/uniform/index.html

                    
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
