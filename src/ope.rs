/*
 * File: ope.rs
 *      Rust implementation of Order Preserving Encryption OPE
 *      Based off of a python implementation 
 *      (https://github.com/tonyo/pyope/blob/master/pyope/ope.py)
 */

extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto

 
pub mod ope {
   
    use crypto::mac::Mac;
    use crypto::mac::{hmac, aes, mac, sha2};
    use crypto::aes::KeySize;
    use crypto::symmetriccipher::{SynchronousStreamCipher, Encryptor};
    use std::io::{Read, Write, Cursor};
    use std::fs::File;
    use rand::{Rng, thread_rng};
    use rand::distributions::Uniform; // https://docs.rs/rand/latest/rand/distributions/uniform/index.html
    use rand_distr::{Distribution, Hypergeometric}; // https://rust-random.github.io/rand/rand_distr/struct.Hypergeometric.html

    let DEFAULT_INPUT_RANGE_START = 0;
    let DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() - 1;
    let DEFAULT_OUTPUT_RANGE_START = 1;
    let DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() - 1;
    
    pub struct Range {
        start: u64,
        end: u64,
    }

    impl Range {

        pub fn contains(&mut self, number: u64) -> bool {
            
            return self.start <= number <= self.end;
        }

        pub fn range_size(&mut self) -> i32 {

            return self.end - self.start + 1;
            

        }
    }

    pub struct OPE {
        key: String,
        in_range: RANGE,
        out_range: RANGE,
    }

    impl OPE {

        // TO DO
        pub fn init_ope(&mut self, key:String) {

            let ope = OPE { key, Range { DEFAULT_INPUT_RANGE_START, DEFAULT_INPUT_RANGE_END }, Range { DEFAULT_OUTPUT_RANGE_START, DEFAULT_OUTPUT_RANGE_END } };
                
        }

        /*
         * encrypt(self, plaintext)
         *  OPE is recursive encryption, check cases then recursive helper
         */
        pub fn encrypt(&mut self, plaintext) {
            
            if !(self.in_range.contains(plaintext)
                 return;

            return self.recursive_encrypt(plaintext, self.in_range.start, self.out_range.end);
        }
        
        pub fn recurisve_encrypt(&mut self, plaintext, in_range, out_range) {

                let in_size = in_range.size();
                let out_size = out_range.size();
                let in_edge = in_range.start - 1;
                let out_edge = out_range.start -1;
                let tmp = out_size / 2;
                let mid = out_edge + tmp.ceil();

                // sanity check 
                assert!(in_size <= out_size);

                if (in_range.size() == 1) {
                    let min_in = in_range.start;
                    let tape = self.tape_gen(min_in);
                    
                    let mut rng = thread_rng();
                    let ciphertext = rng.Uniform(out_range, tape);

                    return ciphertext;
                }
                let mut tape = self.tape_gen(mid);
                let hypergeo = Hypergeometric::new(out_size, mid, in_size);
                let sample = hypergeo.sample(&tape); 

                if (plaintext <= sample) {
                    in_range = Range { in_edge + 1, sample};
                    out_range = Range { out_edge + 1, mid };
                }  else {
                    in_range = Range { sample + 1, in_edge + in_size };
                    out_range = Range { mid + 1, out_edge + out_size };
                }

                return self.recursive_encrypt(plaintext, in_range, out_range)


        }   
        pub fn decrypt(&mut self, ciphertext) {
        
             if !(self.in_range.contains(plaintext)
                 return;

            return self.recursive_encrypt(plaintext, self.out_range.start, self.out_range.end);

       
        }

        pub fn recursive_decrypt(&mut self, ciphertext, in_range, out_range) {
            
                let in_size = in_range.size();
                let out_size = out_range.size();
                let in_edge = in_range.start - 1;
                let out_edge = out_range.start -1;
                let tmp = out_size / 2;
                let mid = out_edge + tmp.ceil();

                // sanity check
                assert!(in_size <= out_size);
                    
                if (in_range.size() == 1) {
                    let min_in = in_range.start;
                    let tape = self.tape_gen(min_in);
                    
                    let mut rng = thread_rng();
                    let sample_text = rng.Uniform(out_range, tape);
                    
                    if (sample_text.eq(&cipher_text) {
                        return min_in;
                    }

                    return -1; // failure 
                }
                let mut tape = self.tape_gen(mid);
                let hypergeo = Hypergeometric::new(out_size, mid, in_size);
                let sample = hypergeo.sample(&tape);

                if (ciphertext <= mid) {
                    in_range = Range { in_edge + 1, sample};
                    out_range = Range { out_edge + 1, mid };
                }  else {
                    in_range = Range { sample + 1, in_edge + in_size };
                    out_range = Range { mid + 1, out_edge + out_size };
                }


                return self.recursive_decrypt(ciphertext, in_range, out_range);

        }

        /*
         * tape_gen(self, data)
         *  Return: bit string of data
         */
        pub fn tape_gen(&mut self, data: u64) {
            
            let data_str = data.to_string();
            let mut hmac_obj = hmac::Hmac::new(sha2::Sha256::new(), self.key);
            let aes_cipher = aes::Aes256::new(self.key);

            hmac_obj.update(data_str.as_bytes());

            // sanity check
            assert_eq!(hmac_obj.digest_size, 32); 

           cipher.encrypt_block(&mut hmac_obj.result().as_bytes());

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
    
    }

}
