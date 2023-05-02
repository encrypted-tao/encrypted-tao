/*
 * File: crypto.rs
 *      Query encryption/decryption
 */
extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate hmac;
extern crate aes;
extern crate sha2;

//use crate::ope::ope::hgd::PRNG;
use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::aes::{KeySize, ctr};

use crate::ope::ope::ope::OPE;
use crate::ope::ope::ope::Range;

pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;


pub fn retrieve_key(key_type: &str) -> String {

    match key_type {
        "ope" =>  "ope-testing-key".to_string(),
        "aes" =>  "my-tao-testing-key".to_string(),
        _ => "not-ope-aes-key".to_string(),
        
    }

}
pub fn encrypt_ope(data: i64) -> i64 {

    // check cache

    let key = retrieve_key("ope");
    let mut ope: OPE = OPE { key:key, 
                        in_range: Range { start: 1, end: DEFAULT_INPUT_RANGE_END }, 
                        out_range: Range { start: 1, end: DEFAULT_OUTPUT_RANGE_END } };

    // add to cache 
    let encrypt = ope.encrypt(data.try_into().unwrap());
    return encrypt.try_into().unwrap();
}

pub fn encrypt_string(data: String) -> String {

    
    let key = retrieve_key("aes");
    let mut aes_cipher =  ctr(KeySize::KeySize256, &key.into_bytes(), &[b'\x00';16]); // change IV?

    let data_bytes = data.into_bytes();

    aes_cipher.process(&data_bytes, &mut data_bytes.clone());

    let data_string: String = data_bytes.iter().map(ToString::to_string).collect();
    return data_string;
}

pub fn encrypt_int(data: i32) -> i32 {

    let key = retrieve_key("aes");
    let mut aes_cipher =  ctr(KeySize::KeySize256, &key.into_bytes(), &[b'\x00';16]); // change IV?

    let data_string = data.to_string();
    let data_bytes = data_string.into_bytes();

    aes_cipher.process(&data_bytes, &mut data_bytes.clone());

    return data_bytes[0].into();


}

pub fn encrypt_idset(data: Vec<i32>) -> Vec<i32> {

    let mut encrypt = data.clone();

    for i in 0..data.len() {
        encrypt[i] = encrypt_int(data[i]);
    }
    return encrypt;

}
/*
 * Query crypto test
 * run `via cargo test`
 */
#[cfg(test)]
mod tests {

    use crate::query::crypto::{encrypt_int, encrypt_idset, encrypt_string};
    use crate::ope::ope::ope::OPE;
    use crate::ope::ope::ope::Range;

    pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
    pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

    #[test]
    fn test_encrypt_int() {
        let res = encrypt_int(8);
        assert_eq!(res, 56);
    }

    #[test]
    fn test_encrypt_idset() {
        let encrypt = encrypt_idset(vec![78, 2, 4, 99]);
        assert_eq!(vec![55, 50, 52, 57], encrypt);
    }

    #[test]
    fn test_encrypt_string() {
        let encrypt = encrypt_string("testing".to_string());
        let test = "116101115116105110103".to_string();
        
        assert_eq!(encrypt, test);
    }
  
}