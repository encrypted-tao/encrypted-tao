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

//mod utils;


pub mod ope {

    use crate::ope::utils::aes_init;
    use crate::ope::hgd::hypergeo_sample;
    use crate::ope::stats::uniform_sample;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use aes::Aes256;
    use aes::cipher::{
        BlockCipher, BlockEncrypt, BlockDecrypt};
    use aes_prng::AesRng;
    use std::io::{Read, Write, Cursor};
    use std::fs::File;
    use std::str;
    use generic_array::{GenericArray, arr, ArrayLength};
    use std::cmp;
    use generic_array::typenum::{UInt, Integer};
    //use utils::aes_init;

    const DEFAULT_INPUT_RANGE_START: u64 = 0;
    const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
    const DEFAULT_OUTPUT_RANGE_START: u64 = 1;
    const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

    pub struct Range {
        pub start: u64,
        pub end: u64,
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
        pub fn Copy(&mut self) -> Range {

            return Range {start:self.start, end:self.end};
        }
    }


    pub struct OPE {
        key: String,
        in_range: Range,
        out_range: Range,
    }

    impl OPE {
                
        /*
         * encrypt(self, plaintext)
         *  OPE is recursive encryption, check cases then recursive helper
         */
        pub fn encrypt(&mut self, plaintext:u64) -> u64 {
            
            if !self.in_range.contains(plaintext) {
                 return 1 as u64;
            }

            return self.recursive_encrypt(plaintext, self.in_range.start, self.in_range.end, self.out_range.start, self.out_range.end);
        }
        
        pub fn recursive_encrypt(&mut self, plaintext: u64,  in_start: u64, in_end:u64, out_start:u64, out_end:u64) -> u64 {

            let mut in_range = Range {start: in_start, end: in_end};
            let mut out_range = Range {start: out_start, end: out_end};
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
                    let tape = self.tape_gen(min_in);
                    let ciphertext = uniform_sample(in_range, tape);
                    return ciphertext;
                }
                
                let tape = self.tape_gen(mid);
                let samples = hypergeo_sample(in_start, in_end, out_start, out_end, mid, tape);

                if plaintext.le(&samples) {
                    return self.recursive_encrypt(plaintext, in_edge + 1, samples, out_edge + 1, mid);
                }  else {
                    let new_in = Range { start:samples + 1, end:in_edge + in_size };
                    let new_out = Range { start:mid + 1, end:out_edge + out_size };
                    return self.recursive_encrypt(plaintext, samples + 1, in_edge + in_size, mid + 1, out_edge + out_size);
                }

        }   
        pub fn decrypt(&mut self, ciphertext: u64) -> u64 {
        
             if !self.in_range.contains(ciphertext) {
                 return 1 as u64;
             }
            return self.recursive_decrypt(ciphertext, self.in_range.start, self.in_range.end, self.out_range.start, self.out_range.end);
       
        }

        pub fn recursive_decrypt(&mut self, ciphertext: u64, in_start: u64, in_end:u64, out_start:u64, out_end:u64) -> u64 {
            
               let mut in_range = Range {start: in_start, end: in_end};
               let mut out_range = Range {start: out_start, end: out_end};
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
                    let tape = self.tape_gen(min_in);
                    let sample_text = uniform_sample(in_range, tape);
                    if sample_text.eq(&ciphertext) {
                        return min_in;
                    }
                    // else -> failure

                }
                
                let tape = self.tape_gen(mid);
                let samples = hypergeo_sample(in_start, in_end, out_start, out_end, mid, tape);

                if ciphertext.le(&mid) {
                    return self.recursive_decrypt(ciphertext, in_edge + 1, samples, out_edge+1, mid)
                }  else {
                    return self.recursive_decrypt(ciphertext, samples+1, in_edge+in_size, mid+1, out_edge+out_size);
                }



        }

        /*
         * tape_gen(self, data)
         *  Return: bit using of data
         */
        pub fn tape_gen(&mut self, data: u64) ->  String {

            let data_str = data.to_string();
            
            let data_bytes = data_str.as_bytes();
        
            type HmacSha256 = Hmac<Sha256>;

            let mut hmac_obj = HmacSha256::new_from_slice(self.key.as_bytes()).unwrap();
            
            hmac_obj.update(&data_bytes);

            let hmac_res = hmac_obj.finalize();
            

            let aes_cipher = aes_init(&mut hmac_res.into_bytes());

            aes_cipher.encrypt_block(&mut GenericArray::from_slice(&data_bytes));
            
            return std::str::from_utf8(&data_bytes).unwrap().to_string();

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
