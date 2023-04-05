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
    
    let DEFAULT_INPUT_RANGE_START = 0;
    let DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() - 1;
    let DEFAULT_OUTPUT_RANGE_START = 1;
    let DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() - 1;
    
    pub struct OPE {
        key: String
    }

    impl OPE {
        /*
         * encrypt(self, plaintext)
         *  OPE is recursive encryption, check cases then recursive helper
         */
        pub fn encrypt(&mut self, plaintext) -> {
            
        }
        
        pub fn recurisve_encrypt(&mut self, plaintext, in_range, out_range) {
       
        }
        pub fn decrypt(&mut self, ciphertext) -> {
       
        }

        pub fn recursive_decrypt(&mut self, ciphertext, in_range, out_range) {
        
        }
        /*
         * read_key(self)
         *  Read in private key, test with generated key  
         */
        pub fn read_key(&mut self, key:String) {

            OPE {
                key:key,
            }
                   
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

