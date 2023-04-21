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
    use std::io::{Read, Write, Cursor};
    use std::fs::File;
    use std::str;
    use generic_array::{GenericArray, arr, ArrayLength};
    use std::cmp;
    use generic_array::typenum::{UInt, Integer};
 
    pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
    pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

    pub struct Range {
        pub start: u64,
        pub end: u64,
    }

    impl Range {

        pub fn contains(&mut self, number: u64) -> bool {
           return self.start <= number && self.end >= number;
        }

        pub fn size(&mut self) -> u64 {

            return self.end - self.start + 1;
            

        }
        
        pub fn copy(&mut self) -> Range {

            return Range {start:self.start, end:self.end};
        }
       
    }


    pub struct OPE {
        pub key: String,
        pub in_range: Range,
        pub out_range: Range,
    }

    impl OPE {
                
        /*
         * encrypt(self, plaintext)
         *  OPE is recursive encryption, check cases then recursive helper
         */
        pub fn encrypt(&mut self, plaintext: u64) -> u64 {
            
            if !self.in_range.contains(plaintext) {
                println!("range does not contain plaintext\n");
                 return 1 as u64;
            }

            return self.recursive_encrypt(plaintext, self.in_range.start, self.in_range.end, self.out_range.start, self.out_range.end);
        }
        
        pub fn recursive_encrypt(&mut self, plaintext: u64,  in_start: u64, in_end:u64, out_start:u64, out_end:u64) -> u64 {

                let mut in_range = Range {start: in_start, end: in_end};
                let mut out_range = Range {start: out_start, end: out_end};
                let in_size = in_range.size();
                let out_size = out_range.size();
                let in_edge = (in_range.start as i64 - 1) as u64 ;
                let out_edge = (out_range.start as i64 -1) as u64;
                let mid = out_edge + ((out_size as f64 / 2.0)).ceil() as u64;

                // sanity check 
                assert!(in_size <= out_size);

                if in_range.size() == 1 {
                    let min_in = in_range.start;
                    let tape = self.tape_gen(plaintext);
                    let ciphertext = uniform_sample(out_range, tape);
                    return ciphertext;
                }
                
                let tape = self.tape_gen(mid);
                let samples = hypergeo_sample(in_start, in_end, out_start, out_end, mid, tape);

                if plaintext <= samples {
                    return self.recursive_encrypt(plaintext, in_edge + 1, samples, out_edge + 1, mid);
                }  else {
                    return self.recursive_encrypt(plaintext, samples + 1, in_edge + in_size, mid + 1, out_edge + out_size);
                }

        }   
        pub fn decrypt(&mut self, ciphertext: u64) -> u64 {
        
             if !self.in_range.contains(ciphertext) {
                println!("range does not contain ciphertext\n");
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
                let mid = out_edge + ((out_size as f64 / 2.0)).ceil() as u64;

                // sanity check
                assert!(in_size <= out_size);
                    
                if in_range.size() == 1 {
                    let min_in = in_range.start;
                    let tape = self.tape_gen(min_in);
                    let sample_text = uniform_sample(out_range, tape);
                    if sample_text.eq(&ciphertext) {
                        return min_in;
                    }
                    // else -> failure

                }
                
                let tape = self.tape_gen(mid);
                let samples = hypergeo_sample(in_start, in_end, out_start, out_end, mid, tape);

                if ciphertext <= mid {
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

            let mut data_arr = GenericArray::from_slice(&data_bytes).clone();

            aes_cipher.encrypt_block(&mut data_arr);

            
            return std::str::from_utf8(&data_arr).unwrap().to_string();

        }

    }
}    

/*
 * OPE tests
 *  run via `cargo test`
 */
#[cfg(test)]
mod tests {
    use super::*;

    use crate::ope::ope::ope::OPE;
    use crate::ope::ope::ope::Range;

    pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
    pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;


    #[test]
    fn test_encrypt() {

        let mut test = OPE { key:"testing-key".to_string(), in_range: Range {start: 0 , end: DEFAULT_INPUT_RANGE_END}, out_range: Range {start: 1, end: DEFAULT_OUTPUT_RANGE_END}};
        let a = test.encrypt(25 as u64);
        let b = test.encrypt(50 as u64);
        let c = test.encrypt(100 as u64);

        println!("result of a: {}, b: {}, c: {}", a, b, c);

        assert!(a < b);
        assert!(b < c);

    }
    #[test]
    fn test_decrypt() {
       let mut test = OPE { key:"testing-key".to_string(), in_range: Range {start: 0 , end: DEFAULT_INPUT_RANGE_END}, out_range: Range {start: 1, end: DEFAULT_OUTPUT_RANGE_END}};
       
       let num = test.encrypt(23614);

       assert_eq!(23614, test.decrypt(num));
    }

}
